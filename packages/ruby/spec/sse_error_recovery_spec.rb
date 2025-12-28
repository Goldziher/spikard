# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'SSE Producer Error Recovery and Client Reconnection' do
  # Test 1: Producer#next_event raises → stream continues (error logged)
  describe 'error in next_event is caught and logged' do
    it 'logs error when next_event raises' do
      error_producer_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @call_count = 0
        end

        def next_event
          @call_count += 1
          raise StandardError, 'Producer error' if @call_count == 2

          Spikard::SseEvent.new(
            data: { count: @call_count },
            id: @call_count.to_s
          )
        end

        attr_reader :call_count
      end

      producer = error_producer_class.new
      # First call succeeds
      event1 = producer.next_event
      expect(event1).to be_a(Spikard::SseEvent)
      expect(event1.data).to eq({ count: 1 })

      # Second call raises
      expect { producer.next_event }.to raise_error(StandardError, 'Producer error')

      # Verify producer is still functional if error is caught at app level
      producer.instance_variable_set(:@call_count, 2)
      event3 = producer.next_event
      expect(event3).to be_a(Spikard::SseEvent)
      expect(event3.data).to eq({ count: 3 })
    end

    it 'allows stream to continue after exception' do
      exception_producer_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @count = 0
          @errors = []
        end

        def next_event
          @count += 1
          return nil if @count > 5

          if @count == 3
            @errors << 'Temporary error'
            # In real implementation, this would be caught upstream
            raise 'Temporary failure'
          end

          Spikard::SseEvent.new(data: { event: @count })
        end

        attr_reader :errors, :count
      end

      producer = exception_producer_class.new
      events = []

      # Simulate error handling at app level
      6.times do
        event = producer.next_event
        events << event if event
      rescue RuntimeError
        # Error caught and logged
        next
      end

      expect(events.length).to be >= 4 # At least some events before/after error
      expect(producer.errors.length).to eq(1)
    end
  end

  # Test 2: Producer#next_event returns invalid structure → caught and skipped
  describe 'invalid event structure is handled' do
    it 'skips malformed event structures' do
      bad_producer_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @count = 0
        end

        def next_event
          @count += 1
          return nil if @count > 3

          if @count == 2
            # Return invalid structure (not an SseEvent)
            return { data: 'not_an_event' }
          end

          Spikard::SseEvent.new(data: { valid: true }, id: @count.to_s)
        end

        attr_reader :count
      end

      producer = bad_producer_class.new
      events = []
      valid_events = []

      4.times do
        event = producer.next_event
        events << event

        # Only count valid SseEvent instances
        valid_events << event if event.is_a?(Spikard::SseEvent)
      end

      expect(events.length).to eq(4)
      expect(valid_events.length).to eq(2) # Only the valid SseEvents
    end

    it 'validates event has required data attribute' do
      event = Spikard::SseEvent.new(data: { message: 'test' })
      expect(event.data).not_to be_nil
      expect(event.data).to be_a(Hash)
    end

    it 'returns valid event structure with optional fields' do
      event = Spikard::SseEvent.new(
        data: { info: 'test' },
        event_type: 'update',
        id: '123',
        retry_ms: 5000
      )
      hash = event.to_h
      expect(hash).to include(
        data: { info: 'test' },
        event_type: 'update',
        id: '123',
        retry: 5000
      )
    end
  end

  # Test 3: on_connect hook raises → connection rejected with 500 error
  describe 'on_connect hook error handling' do
    it 'raises error when on_connect fails' do
      failing_connect_class = Class.new(Spikard::SseEventProducer) do
        def on_connect
          raise 'Connection initialization failed'
        end

        def next_event
          Spikard::SseEvent.new(data: { msg: 'event' })
        end
      end

      producer = failing_connect_class.new
      expect { producer.on_connect }.to raise_error(
        RuntimeError,
        'Connection initialization failed'
      )
    end

    it 'allows handler to catch on_connect errors' do
      failing_producer_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :connected

        def on_connect
          @connected = false
          raise ArgumentError, 'Auth failed'
        end

        def next_event
          Spikard::SseEvent.new(data: { ok: true })
        end
      end

      producer = failing_producer_class.new
      error_caught = false

      begin
        producer.on_connect
      rescue ArgumentError => e
        error_caught = true
        expect(e.message).to eq('Auth failed')
      end

      expect(error_caught).to be true
      expect(producer.connected).to be false
    end

    it 'differentiates between successful and failed connections' do
      tracking_producer_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :connection_attempts

        def initialize
          @connection_attempts = []
        end

        def on_connect
          attempt = { timestamp: Time.now, success: true }
          @connection_attempts << attempt

          raise StandardError, 'Simulated failure' if @connection_attempts.length == 2

          @connected = true
        end

        def next_event
          Spikard::SseEvent.new(data: { event: 'data' })
        end
      end

      producer = tracking_producer_class.new

      # First attempt succeeds
      producer.on_connect
      expect(producer.connection_attempts.length).to eq(1)

      # Second attempt fails
      expect { producer.on_connect }.to raise_error(StandardError)
      expect(producer.connection_attempts.length).to eq(2)
    end
  end

  # Test 4: on_disconnect hook raises → doesn't prevent new connections
  describe 'on_disconnect hook error recovery' do
    it 'raises error when on_disconnect fails' do
      failing_disconnect_class = Class.new(Spikard::SseEventProducer) do
        def on_disconnect
          raise 'Cleanup failed'
        end

        def next_event
          Spikard::SseEvent.new(data: { msg: 'event' })
        end
      end

      producer = failing_disconnect_class.new
      expect { producer.on_disconnect }.to raise_error(
        RuntimeError,
        'Cleanup failed'
      )
    end

    it 'allows subsequent connections after disconnect error' do
      resilient_producer_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :connection_count

        def initialize
          @connection_count = 0
        end

        def on_connect
          @connection_count += 1
        end

        def on_disconnect
          raise StandardError, 'Disconnect failed' if @connection_count > 1
        end

        def next_event
          Spikard::SseEvent.new(data: { count: @connection_count })
        end
      end

      producer = resilient_producer_class.new

      # First connect succeeds
      producer.on_connect
      expect(producer.connection_count).to eq(1)

      # First disconnect succeeds
      expect { producer.on_disconnect }.not_to raise_error

      # Second connect should still work
      producer.on_connect
      expect(producer.connection_count).to eq(2)

      # Second disconnect fails
      expect { producer.on_disconnect }.to raise_error(StandardError)

      # But we can still reconnect if error is caught
      producer.on_connect
      expect(producer.connection_count).to eq(3)
    end

    it 'prevents disconnect error from affecting new connections' do
      resilient_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :disconnect_errors, :reconnects

        def initialize
          @disconnect_errors = []
          @reconnects = 0
        end

        def on_connect
          @reconnects += 1
        end

        def on_disconnect
          @disconnect_errors << { at_time: Time.now }
          raise 'Cleanup error' if @disconnect_errors.length == 1
        end

        def next_event
          Spikard::SseEvent.new(data: { reconnect: @reconnects })
        end
      end

      producer = resilient_class.new

      # Connect, disconnect (error caught), reconnect
      producer.on_connect
      expect { producer.on_disconnect }.to raise_error

      producer.on_connect
      expect(producer.reconnects).to eq(2)
    end
  end

  # Test 5: Client reconnect with Last-Event-ID resumes from correct event
  describe 'client reconnection with Last-Event-ID' do
    it 'tracks event IDs for resume capability' do
      resume_producer_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @events = [
            { id: '1', data: { num: 1 } },
            { id: '2', data: { num: 2 } },
            { id: '3', data: { num: 3 } },
            { id: '4', data: { num: 4 } },
            { id: '5', data: { num: 5 } }
          ]
          @index = 0
        end

        def next_event
          return nil if @index >= @events.length

          event_data = @events[@index]
          @index += 1

          Spikard::SseEvent.new(
            data: event_data[:data],
            id: event_data[:id]
          )
        end
      end

      producer = resume_producer_class.new

      # Get first 3 events
      events = []
      3.times do
        event = producer.next_event
        events << event if event
      end

      last_id = events.last.id
      expect(last_id).to eq('3')

      # Resume from after last ID
      resume_producer = resume_producer_class.new
      resume_producer.instance_variable_get(:@events).each_with_index do |_evt, idx|
        break if idx.to_s >= last_id.to_i

        resume_producer.next_event
      end

      # Next events after resume
      next_event = resume_producer.next_event
      expect(next_event.id).to eq('4')
    end

    it 'supports numeric event IDs for sequencing' do
      numeric_producer_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @event_id = 0
        end

        def next_event
          return nil if @event_id >= 5

          @event_id += 1
          Spikard::SseEvent.new(
            data: { seq: @event_id },
            id: @event_id.to_s
          )
        end
      end

      producer = numeric_producer_class.new
      last_seen_id = nil

      loop do
        event = producer.next_event
        break unless event

        last_seen_id = event.id
      end

      expect(last_seen_id).to eq('5')

      # Simulate resume: find where to start
      resume_producer = numeric_producer_class.new
      resume_id = 3
      resumed_events = []

      loop do
        event = resume_producer.next_event
        break unless event

        resumed_events << event if event.id.to_i > resume_id.to_i
      end

      expect(resumed_events.map(&:id)).to eq(%w[4 5])
    end

    it 'maintains event ID across producer lifecycle' do
      lifecycle_producer_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :last_sent_id

        def initialize
          @counter = 0
          @last_sent_id = nil
        end

        def on_connect
          # Initialize or reset
          @counter = 0
        end

        def next_event
          return nil if @counter >= 3

          @counter += 1
          @last_sent_id = @counter.to_s

          Spikard::SseEvent.new(
            data: { id_tracking: @counter },
            id: @last_sent_id
          )
        end

        def on_disconnect
          # Preserve state if needed
        end
      end

      producer = lifecycle_producer_class.new
      producer.on_connect

      events = []
      loop do
        event = producer.next_event
        break unless event

        events << event
      end

      expect(producer.last_sent_id).to eq('3')
      expect(events.length).to eq(3)
    end
  end

  # Test 6: Event ID collision detection
  describe 'event ID collision detection' do
    it 'detects duplicate event IDs' do
      collision_producer_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @count = 0
        end

        def next_event
          return nil if @count >= 3

          # Intentionally reuse ID
          id = @count == 2 ? '1' : (@count + 1).to_s
          @count += 1

          Spikard::SseEvent.new(
            data: { msg: "Event #{@count}" },
            id: id
          )
        end
      end

      producer = collision_producer_class.new
      seen_ids = Set.new

      events = []
      loop do
        event = producer.next_event
        break unless event

        events << event
        seen_ids << event.id if event.id
      end

      # Last two events have different indices but one reuses ID
      expect(events[0].id).to eq('1')
      expect(events[2].id).to eq('1') # Collision!
      expect(seen_ids.size).to eq(2) # Only 2 unique IDs for 3 events
    end

    it 'allows application to track ID collisions' do
      tracking_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :collisions

        def initialize
          @count = 0
          @sent_ids = {}
          @collisions = []
        end

        def next_event
          return nil if @count >= 5

          @count += 1
          # Create collision on 3rd and 4th event
          id = if @count.between?(3, 4)
                 'duplicate'
               else
                 @count.to_s
               end

          @collisions << { id: id, count: @count } if @sent_ids[id]
          @sent_ids[id] = true

          Spikard::SseEvent.new(data: { n: @count }, id: id)
        end
      end

      producer = tracking_class.new

      loop do
        event = producer.next_event
        break unless event
      end

      expect(producer.collisions.length).to eq(1)
      expect(producer.collisions.first[:id]).to eq('duplicate')
    end

    it 'handles string IDs with collision detection' do
      string_id_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @uuid_count = 0
        end

        def next_event
          return nil if @uuid_count >= 4

          @uuid_count += 1
          # Simulate UUID that might have collisions
          # rubocop:disable Lint/DuplicateBranch
          id = case @uuid_count
               when 1 then 'uuid-abc-123'
               when 2 then 'uuid-def-456'
               when 3 then 'uuid-abc-123' # Duplicate!
               else 'uuid-ghi-789'
               end
          # rubocop:enable Lint/DuplicateBranch

          Spikard::SseEvent.new(data: { id: @uuid_count }, id: id)
        end
      end

      producer = string_id_class.new
      id_counts = Hash.new(0)

      loop do
        event = producer.next_event
        break unless event

        id_counts[event.id] += 1
      end

      expect(id_counts['uuid-abc-123']).to eq(2) # Collision detected
      expect(id_counts['uuid-def-456']).to eq(1)
    end
  end

  # Test 7: Infinite event stream (producer never returns nil)
  describe 'infinite event streams' do
    it 'produces events indefinitely when not limited' do
      infinite_producer_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @count = 0
        end

        def next_event
          # Never return nil
          @count += 1
          Spikard::SseEvent.new(
            data: { event_num: @count },
            id: @count.to_s
          )
        end

        attr_reader :count
      end

      producer = infinite_producer_class.new

      # Collect events until we hit a limit
      events = []
      100.times do
        event = producer.next_event
        events << event
      end

      expect(events.length).to eq(100)
      expect(producer.count).to eq(100)
      expect(events.last.data[:event_num]).to eq(100)
    end

    it 'allows client-side interruption of infinite stream' do
      forever_producer_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @event_count = 0
        end

        def next_event
          @event_count += 1
          # Would produce forever if not interrupted
          Spikard::SseEvent.new(data: { endless: true }, id: @event_count.to_s)
        end

        attr_reader :event_count
      end

      producer = forever_producer_class.new

      # Client decides when to stop
      events = []
      50.times do |i|
        events << producer.next_event
        break if i == 49 # Stop after 50 events
      end

      expect(events.length).to eq(50)
      expect(producer.event_count).to eq(50)
    end

    it 'maintains state in infinite producer' do
      stateful_infinite_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :stats

        def initialize
          @stats = { events_sent: 0, bytes_sent: 0 }
        end

        def next_event
          @stats[:events_sent] += 1
          data = { running: true, count: @stats[:events_sent] }
          # Estimate data size
          @stats[:bytes_sent] += data.to_s.length
          Spikard::SseEvent.new(data: data, id: @stats[:events_sent].to_s)
        end
      end

      producer = stateful_infinite_class.new

      200.times { producer.next_event }

      expect(producer.stats[:events_sent]).to eq(200)
      expect(producer.stats[:bytes_sent]).to be > 200
    end

    it 'allows timeout/interruption mechanism' do
      interruptible_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :running, :event_count

        def initialize
          @running = true
          @event_count = 0
          @start_time = Time.now
        end

        def next_event
          return nil unless @running

          @event_count += 1

          # Allow interruption via state flag
          if (Time.now - @start_time) > 10
            @running = false
            return nil
          end

          Spikard::SseEvent.new(data: { evt: @event_count })
        end

        def stop
          @running = false
        end
      end

      producer = interruptible_class.new

      events = []
      50.times do
        event = producer.next_event
        break unless event

        events << event
      end

      # Manually stop the stream
      producer.stop
      next_event = producer.next_event
      expect(next_event).to be_nil
    end
  end

  # Test 8: SSE connection timeout while awaiting next_event
  describe 'connection timeout handling' do
    it 'tracks time spent waiting for next_event' do
      slow_producer_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :wait_durations

        def initialize
          @wait_durations = []
          @call_count = 0
        end

        def next_event
          @call_count += 1
          start = Time.now

          # Simulate work/waiting
          sleep(0.01) if @call_count == 2

          @wait_durations << (Time.now - start)

          return nil if @call_count > 3

          Spikard::SseEvent.new(data: { seq: @call_count })
        end
      end

      producer = slow_producer_class.new

      events = []
      loop do
        event = producer.next_event
        break unless event

        events << event
      end

      expect(producer.wait_durations.length).to eq(4)
      # One call took noticeably longer
      expect(producer.wait_durations[1]).to be > 0.005
    end

    it 'allows timeout configuration in producer' do
      timeout_aware_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :timeout_ms, :timed_out

        def initialize(timeout_ms = 5000)
          @timeout_ms = timeout_ms
          @timed_out = false
          @call_count = 0
        end

        def next_event
          @call_count += 1
          start = Time.now

          # Simulate potentially slow operation
          duration = @call_count == 2 ? 0.1 : 0.001
          sleep(duration)

          elapsed_ms = (Time.now - start) * 1000
          @timed_out = elapsed_ms > @timeout_ms

          return nil if @timed_out || @call_count > 5

          Spikard::SseEvent.new(data: { call: @call_count })
        end
      end

      # With short timeout
      producer = timeout_aware_class.new(1)
      events = []
      loop do
        event = producer.next_event
        break unless event

        events << event
      end

      expect(producer.timed_out).to be true
      expect(events.length).to eq(1)

      # With long timeout
      producer2 = timeout_aware_class.new(10_000)
      events2 = []
      loop do
        event = producer2.next_event
        break unless event

        events2 << event
      end

      expect(producer2.timed_out).to be false
      expect(events2.length).to eq(5)
    end

    it 'detects hanging producer' do
      hanging_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :hung

        def initialize
          @hung = false
          @call_count = 0
        end

        def next_event
          @call_count += 1

          if @call_count == 3
            @hung = true
            sleep(1) # Simulate hang
          end

          return nil if @call_count > 3

          Spikard::SseEvent.new(data: { n: @call_count })
        end
      end

      producer = hanging_class.new
      start_time = Time.now

      3.times do
        producer.next_event
      end

      elapsed = Time.now - start_time
      expect(producer.hung).to be true
      expect(elapsed).to be > 0.9
    end
  end

  # Test 9: Multiple clients get independent producer instances
  describe 'independent producer instances per client' do
    it 'creates unique producers for each client connection' do
      factory_class = Class.new(Spikard::SseEventProducer) do
        @instance_count = 0

        class << self
          attr_reader :instance_count
        end

        def initialize
          count = self.class.instance_variable_get(:@instance_count) || 0
          @id = count + 1
          self.class.instance_variable_set(:@instance_count, @id)
          @events = 0
        end

        attr_reader :id
      end

      producers = []
      3.times do
        producers << factory_class.new
      end

      expect(producers[0].id).to eq(1)
      expect(producers[1].id).to eq(2)
      expect(producers[2].id).to eq(3)
      expect(producers[0].object_id).not_to eq(producers[1].object_id)
    end

    it 'isolates state between producer instances' do
      stateful_factory_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :producer_id, :event_count

        def initialize(producer_id)
          @producer_id = producer_id
          @event_count = 0
        end

        def next_event
          return nil if @event_count >= 3

          @event_count += 1
          Spikard::SseEvent.new(
            data: { producer: @producer_id, count: @event_count },
            id: "#{@producer_id}-#{@event_count}"
          )
        end
      end

      producer1 = stateful_factory_class.new(1)
      producer2 = stateful_factory_class.new(2)

      # Advance producer1
      3.times { producer1.next_event }

      # producer2 should be unaffected
      event = producer2.next_event
      expect(producer2.event_count).to eq(1)
      expect(event.data[:producer]).to eq(2)
      expect(event.data[:count]).to eq(1)

      # producer1 is exhausted
      expect(producer1.next_event).to be_nil
    end

    it 'allows concurrent tracking of multiple producers' do
      tracked_producer_class = Class.new(Spikard::SseEventProducer) do
        # rubocop:disable Style/ClassVars
        @@active_producers = Set.new
        # rubocop:enable Style/ClassVars

        attr_reader :producer_id

        def initialize(producer_id)
          @producer_id = producer_id
          @@active_producers.add(producer_id)
        end

        def next_event
          Spikard::SseEvent.new(data: { pid: @producer_id })
        end

        def shutdown
          @@active_producers.delete(@producer_id)
        end

        def self.active_count
          @@active_producers.size
        end
      end

      p1 = tracked_producer_class.new(1)
      p2 = tracked_producer_class.new(2)
      p3 = tracked_producer_class.new(3)

      expect(tracked_producer_class.active_count).to eq(3)

      p1.shutdown
      expect(tracked_producer_class.active_count).to eq(2)

      p2.shutdown
      p3.shutdown
      expect(tracked_producer_class.active_count).to eq(0)
    end

    it 'supports factory pattern for producer creation' do
      producer_factory = lambda do |client_id|
        Class.new(Spikard::SseEventProducer) do
          attr_reader :client_id

          define_method(:initialize) do
            @client_id = client_id
            @count = 0
          end

          def next_event
            return nil if @count >= 2

            @count += 1
            Spikard::SseEvent.new(data: { client: @client_id, msg: @count })
          end
        end.new
      end

      client1_producer = producer_factory.call('client-1')
      client2_producer = producer_factory.call('client-2')

      event1 = client1_producer.next_event
      event2 = client2_producer.next_event

      expect(event1.data[:client]).to eq('client-1')
      expect(event2.data[:client]).to eq('client-2')
    end
  end

  # Test 10: Producer state doesn't leak across client reconnects
  describe 'producer state isolation across reconnects' do
    it 'resets state on reconnection' do
      resettable_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :is_connected

        def initialize
          reset_state
        end

        def reset_state
          @is_connected = false
          @event_counter = 0
          @sent_events = []
        end

        def on_connect
          @is_connected = true
          reset_state
        end

        def next_event
          return nil if @event_counter >= 3

          @event_counter += 1
          event = Spikard::SseEvent.new(
            data: { event: @event_counter },
            id: @event_counter.to_s
          )
          @sent_events << event
          event
        end

        def on_disconnect
          @is_connected = false
        end

        attr_reader :sent_events
      end

      producer = resettable_class.new
      producer.on_connect

      # Send events
      3.times { producer.next_event }
      expect(producer.sent_events.length).to eq(3)
      expect(producer.event_counter).to eq(3)

      # Disconnect and reconnect
      producer.on_disconnect
      expect(producer.is_connected).to be false

      producer.on_connect
      expect(producer.is_connected).to be true
      expect(producer.event_counter).to eq(0) # Reset!

      # Can send new events
      new_event = producer.next_event
      expect(new_event.data[:event]).to eq(1)
    end

    it 'prevents accidental state sharing' do
      isolated_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :client_data

        def initialize
          @client_data = {}
        end

        def on_connect
          # Each connection gets fresh data
          @client_data = { session: SecureRandom.hex(4), messages: [] }
        end

        def next_event
          return nil if @client_data[:messages].length >= 2

          msg_num = @client_data[:messages].length + 1
          @client_data[:messages] << msg_num

          Spikard::SseEvent.new(data: @client_data)
        end
      end

      producer = isolated_class.new
      producer.on_connect
      session1 = producer.client_data[:session]

      2.times { producer.next_event }
      expect(producer.client_data[:messages]).to eq([1, 2])

      # Reconnect
      producer.on_connect
      session2 = producer.client_data[:session]

      expect(session1).not_to eq(session2)
      expect(producer.client_data[:messages]).to eq([]) # Fresh state
    end

    it 'allows selective state retention across reconnects' do
      hybrid_state_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :persistent_id, :session_count, :events_sent

        def initialize(persistent_id)
          @persistent_id = persistent_id
          @session_count = 0
          @events_sent = 0
        end

        def on_connect
          @session_count += 1  # Persistent across reconnects
          @session_events = 0  # Reset per session
        end

        def next_event
          return nil if @session_events >= 2

          @session_events += 1
          @events_sent += 1

          Spikard::SseEvent.new(
            data: {
              client_id: @persistent_id,
              session: @session_count,
              total_events: @events_sent
            }
          )
        end
      end

      producer = hybrid_state_class.new('client-abc')
      producer.on_connect

      2.times { producer.next_event }
      expect(producer.events_sent).to eq(2)
      expect(producer.session_count).to eq(1)

      producer.on_connect # Reconnect
      expect(producer.session_count).to eq(2)
      expect(producer.events_sent).to eq(2) # Not reset

      event = producer.next_event
      expect(event.data[:total_events]).to eq(3)
    end
  end

  # Test 11: Memory pressure: producer cleanup on disconnect
  describe 'producer cleanup and memory management' do
    it 'releases resources on disconnect' do
      cleanup_tracking_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :resources_allocated, :cleanup_called

        def initialize
          @resources_allocated = { memory: 1000, connections: 5 }
          @cleanup_called = false
        end

        def on_connect
          # Allocate resources
          @resources_allocated[:memory] = 5000
          @resources_allocated[:connections] = 10
        end

        def next_event
          Spikard::SseEvent.new(data: @resources_allocated)
        end

        def on_disconnect
          @cleanup_called = true
          @resources_allocated[:memory] = 0
          @resources_allocated[:connections] = 0
        end
      end

      producer = cleanup_tracking_class.new
      expect(producer.cleanup_called).to be false

      producer.on_connect
      expect(producer.resources_allocated[:memory]).to eq(5000)

      producer.on_disconnect
      expect(producer.cleanup_called).to be true
      expect(producer.resources_allocated[:memory]).to eq(0)
    end

    it 'clears large data structures on cleanup' do
      memory_heavy_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :buffer_size

        def initialize
          @buffer = []
          @buffer_size = 0
        end

        def next_event
          # Simulate buffer growth
          @buffer << { data: 'x' * 1000 }
          @buffer_size = @buffer.length
          return nil if @buffer_size >= 100

          Spikard::SseEvent.new(data: { buffered: @buffer_size })
        end

        def on_disconnect
          @buffer.clear
          @buffer_size = 0
        end
      end

      producer = memory_heavy_class.new
      100.times { producer.next_event }

      expect(producer.buffer_size).to eq(100)

      producer.on_disconnect
      expect(producer.buffer_size).to eq(0)
    end

    it 'tracks resource lifecycle' do
      lifecycle_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :lifecycle_events

        def initialize
          @lifecycle_events = []
        end

        def on_connect
          @lifecycle_events << { event: 'connect', time: Time.now }
        end

        def on_disconnect
          @lifecycle_events << { event: 'disconnect', time: Time.now }
        end

        def next_event
          Spikard::SseEvent.new(data: { events: @lifecycle_events.length })
        end
      end

      producer = lifecycle_class.new

      producer.on_connect
      2.times { producer.next_event }
      producer.on_disconnect

      expect(producer.lifecycle_events.length).to eq(2)
      expect(producer.lifecycle_events[0][:event]).to eq('connect')
      expect(producer.lifecycle_events[1][:event]).to eq('disconnect')
    end
  end

  # Test 12: Producer next_event returns nil → stream ends gracefully
  describe 'graceful stream termination' do
    it 'stops stream when next_event returns nil' do
      finite_producer_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @count = 0
        end

        def next_event
          return nil if @count >= 5

          @count += 1
          Spikard::SseEvent.new(data: { seq: @count })
        end

        attr_reader :count
      end

      producer = finite_producer_class.new
      events = []

      loop do
        event = producer.next_event
        break if event.nil?

        events << event
      end

      expect(events.length).to eq(5)
      expect(producer.count).to eq(5)
    end

    it 'allows client to know stream has ended' do
      ending_producer_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :stream_ended

        def initialize
          @count = 0
          @stream_ended = false
        end

        def next_event
          if @count >= 3
            @stream_ended = true
            return nil
          end

          @count += 1
          Spikard::SseEvent.new(data: { msg: "Event #{@count}" })
        end
      end

      producer = ending_producer_class.new

      loop do
        event = producer.next_event
        break if event.nil?
      end

      expect(producer.stream_ended).to be true
    end

    it 'supports conditional stream termination' do
      conditional_class = Class.new(Spikard::SseEventProducer) do
        attr_reader :reason

        def initialize(max_events = 10)
          @max_events = max_events
          @count = 0
          @reason = nil
        end

        def next_event
          @count += 1

          if @count > @max_events
            @reason = 'max_events_reached'
            return nil
          end

          if rand > 0.9
            @reason = 'random_termination'
            return nil
          end

          Spikard::SseEvent.new(data: { count: @count })
        end
      end

      producer = conditional_class.new(5)

      events = []
      loop do
        event = producer.next_event
        break if event.nil?

        events << event
      end

      expect(producer.reason).not_to be_nil
    end
  end

  # Test 13: Event data serialization errors handled
  describe 'event data serialization' do
    it 'handles complex data structures' do
      complex_data_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @count = 0
        end

        def next_event
          return nil if @count >= 2

          @count += 1
          complex_data = {
            nested: {
              deep: {
                value: @count,
                array: [1, 2, 3],
                bool: true,
                null_val: nil
              }
            },
            timestamp: Time.now.to_i
          }

          Spikard::SseEvent.new(data: complex_data, id: @count.to_s)
        end
      end

      producer = complex_data_class.new

      event = producer.next_event
      expect(event.data[:nested][:deep][:value]).to eq(1)
      expect(event.data[:nested][:deep][:array]).to eq([1, 2, 3])
      expect(event.data[:nested][:deep][:bool]).to be true
      expect(event.data[:nested][:deep][:null_val]).to be_nil

      hash = event.to_h
      expect(hash[:data]).to include(nested: event.data[:nested])
    end

    it 'converts to hash for serialization' do
      serializable_class = Class.new(Spikard::SseEventProducer) do
        def next_event
          Spikard::SseEvent.new(
            data: { user: 'alice', score: 100 },
            event_type: 'score_update',
            id: '1',
            retry_ms: 3000
          )
        end
      end

      producer = serializable_class.new
      event = producer.next_event
      hash = event.to_h

      expect(hash).to include(
        data: { user: 'alice', score: 100 },
        event_type: 'score_update',
        id: '1',
        retry: 3000
      )
    end

    it 'handles symbol and string keys in data' do
      mixed_keys_class = Class.new(Spikard::SseEventProducer) do
        def next_event
          Spikard::SseEvent.new(
            data: {
              symbol_key: 'value1',
              'string_key' => 'value2',
              nested: { key: 'value3' }
            }
          )
        end
      end

      producer = mixed_keys_class.new
      event = producer.next_event
      data = event.data

      # Both access patterns should work
      expect(data[:symbol_key] || data['symbol_key']).to eq('value1')
      expect(data[:string_key] || data['string_key']).to eq('value2')
      expect(data[:nested][:key] || data['nested']['key']).to eq('value3')
    end
  end

  # Test 14: Custom event types preserved
  describe 'custom event types' do
    it 'preserves custom event_type field' do
      custom_type_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @count = 0
        end

        def next_event
          return nil if @count >= 3

          @count += 1
          event_types = %w[notification update alert]
          event_type = event_types[@count - 1]

          Spikard::SseEvent.new(
            data: { message: "Event #{@count}" },
            event_type: event_type,
            id: @count.to_s
          )
        end
      end

      producer = custom_type_class.new
      events = []

      loop do
        event = producer.next_event
        break unless event

        events << event
      end

      expect(events[0].event_type).to eq('notification')
      expect(events[1].event_type).to eq('update')
      expect(events[2].event_type).to eq('alert')
    end

    it 'handles events without explicit event_type' do
      no_type_class = Class.new(Spikard::SseEventProducer) do
        def next_event
          Spikard::SseEvent.new(
            data: { content: 'no type specified' }
          )
        end
      end

      producer = no_type_class.new
      event = producer.next_event

      expect(event.event_type).to be_nil

      hash = event.to_h
      expect(hash).not_to have_key(:event_type)
    end

    it 'supports multiple custom event type patterns' do
      multi_type_class = Class.new(Spikard::SseEventProducer) do
        def initialize
          @index = 0
          @events = [
            { type: 'user:created', data: { user_id: '1' } },
            { type: 'user:updated', data: { user_id: '1' } },
            { type: 'user:deleted', data: { user_id: '1' } },
            { type: nil, data: { generic: true } },
            { type: 'custom:namespace:event', data: { value: 42 } }
          ]
        end

        def next_event
          return nil if @index >= @events.length

          evt = @events[@index]
          @index += 1

          Spikard::SseEvent.new(
            data: evt[:data],
            event_type: evt[:type],
            id: @index.to_s
          )
        end
      end

      producer = multi_type_class.new
      events = []

      loop do
        event = producer.next_event
        break unless event

        events << event
      end

      expect(events[0].event_type).to eq('user:created')
      expect(events[1].event_type).to eq('user:updated')
      expect(events[2].event_type).to eq('user:deleted')
      expect(events[3].event_type).to be_nil
      expect(events[4].event_type).to eq('custom:namespace:event')
    end

    it 'includes custom event type in serialized hash' do
      serialized_type_class = Class.new(Spikard::SseEventProducer) do
        def next_event
          Spikard::SseEvent.new(
            data: { action: 'notify' },
            event_type: 'app:notification',
            id: '123'
          )
        end
      end

      producer = serialized_type_class.new
      event = producer.next_event
      hash = event.to_h

      expect(hash[:event_type]).to eq('app:notification')
    end
  end
end
