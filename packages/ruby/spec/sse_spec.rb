# frozen_string_literal: true

require 'spec_helper'

RSpec.describe Spikard::SseEvent do
  describe '#initialize' do
    it 'creates an event with required data parameter' do
      event = described_class.new(data: { message: 'hello' })
      expect(event.data).to eq({ message: 'hello' })
    end

    it 'accepts optional event_type parameter' do
      event = described_class.new(data: { count: 1 }, event_type: 'notification')
      expect(event.event_type).to eq('notification')
    end

    it 'accepts optional id parameter' do
      event = described_class.new(data: { count: 1 }, id: '123')
      expect(event.id).to eq('123')
    end

    it 'accepts optional retry_ms parameter' do
      event = described_class.new(data: { count: 1 }, retry_ms: 5000)
      expect(event.retry_ms).to eq(5000)
    end

    it 'initializes all attributes as nil when not provided' do
      event = described_class.new(data: {})
      expect(event.event_type).to be_nil
      expect(event.id).to be_nil
      expect(event.retry_ms).to be_nil
    end

    it 'allows all optional parameters together' do
      event = described_class.new(
        data: { message: 'test' },
        event_type: 'update',
        id: 'abc',
        retry_ms: 3000
      )
      expect(event.data).to eq({ message: 'test' })
      expect(event.event_type).to eq('update')
      expect(event.id).to eq('abc')
      expect(event.retry_ms).to eq(3000)
    end
  end

  describe '#to_h' do
    it 'converts event to hash with data field' do
      event = described_class.new(data: { message: 'hello' })
      hash = event.to_h
      expect(hash).to include(data: { message: 'hello' })
    end

    it 'includes event_type in hash when provided' do
      event = described_class.new(data: {}, event_type: 'notification')
      hash = event.to_h
      expect(hash).to include(event_type: 'notification')
    end

    it 'includes id in hash when provided' do
      event = described_class.new(data: {}, id: '456')
      hash = event.to_h
      expect(hash).to include(id: '456')
    end

    it 'maps retry_ms to retry key in hash' do
      event = described_class.new(data: {}, retry_ms: 2000)
      hash = event.to_h
      expect(hash).to include(retry: 2000)
      expect(hash).not_to have_key(:retry_ms)
    end

    it 'excludes nil values from hash (uses compact)' do
      event = described_class.new(data: { key: 'value' }, event_type: nil, id: nil)
      hash = event.to_h
      expect(hash).to eq({ data: { key: 'value' } })
      expect(hash).not_to have_key(:event_type)
      expect(hash).not_to have_key(:id)
      expect(hash).not_to have_key(:retry)
    end

    it 'includes all fields when all are provided' do
      event = described_class.new(
        data: { status: 'active' },
        event_type: 'status_change',
        id: '789',
        retry_ms: 1000
      )
      hash = event.to_h
      expect(hash).to eq({
                           data: { status: 'active' },
                           event_type: 'status_change',
                           id: '789',
                           retry: 1000
                         })
    end

    it 'handles empty data hash' do
      event = described_class.new(data: {})
      hash = event.to_h
      expect(hash).to eq({ data: {} })
    end

    it 'preserves complex data structures' do
      complex_data = {
        nested: { key: 'value' },
        array: [1, 2, 3],
        string: 'text'
      }
      event = described_class.new(data: complex_data)
      hash = event.to_h
      expect(hash[:data]).to eq(complex_data)
    end
  end

  describe 'attribute accessors' do
    it 'allows modification of data after creation' do
      event = described_class.new(data: { count: 1 })
      event.data = { count: 2 }
      expect(event.data).to eq({ count: 2 })
    end

    it 'allows modification of event_type after creation' do
      event = described_class.new(data: {}, event_type: 'type1')
      event.event_type = 'type2'
      expect(event.event_type).to eq('type2')
    end

    it 'allows modification of id after creation' do
      event = described_class.new(data: {}, id: '1')
      event.id = '2'
      expect(event.id).to eq('2')
    end

    it 'allows modification of retry_ms after creation' do
      event = described_class.new(data: {}, retry_ms: 1000)
      event.retry_ms = 2000
      expect(event.retry_ms).to eq(2000)
    end
  end
end

RSpec.describe Spikard::SseEventProducer do
  describe '#next_event' do
    it 'raises NotImplementedError when not overridden' do
      producer = described_class.new
      expect { producer.next_event }.to raise_error(NotImplementedError)
    end

    it 'raises NotImplementedError with informative message' do
      producer = described_class.new
      error_msg = nil
      begin
        producer.next_event
      rescue NotImplementedError => e
        error_msg = e.message
      end
      expect(error_msg).to include('Spikard::SseEventProducer')
      expect(error_msg).to include('must implement #next_event')
    end
  end

  describe '#on_connect' do
    it 'does not raise when called (optional hook)' do
      producer = described_class.new
      expect { producer.on_connect }.not_to raise_error
    end

    it 'default implementation returns nil' do
      producer = described_class.new
      result = producer.on_connect
      expect(result).to be_nil
    end
  end

  describe '#on_disconnect' do
    it 'does not raise when called (optional hook)' do
      producer = described_class.new
      expect { producer.on_disconnect }.not_to raise_error
    end

    it 'default implementation returns nil' do
      producer = described_class.new
      result = producer.on_disconnect
      expect(result).to be_nil
    end
  end

  describe 'subclass implementation' do
    let(:test_producer_class) do
      Class.new(described_class) do
        def initialize
          @count = 0
          @connected = false
        end

        def next_event
          return nil if @count >= 3

          event = Spikard::SseEvent.new(
            data: { message: "Event #{@count}" },
            event_type: 'test',
            id: @count.to_s
          )
          @count += 1
          event
        end

        def on_connect
          @connected = true
        end

        def on_disconnect
          @connected = false
        end

        attr_reader :count, :connected
      end
    end

    it 'allows subclassing with custom next_event implementation' do
      producer = test_producer_class.new
      event1 = producer.next_event
      expect(event1).to be_a(Spikard::SseEvent)
      expect(event1.data).to eq({ message: 'Event 0' })
    end

    it 'allows generation of multiple events' do
      producer = test_producer_class.new
      events = []
      loop do
        event = producer.next_event
        break if event.nil?

        events << event
      end
      expect(events.length).to eq(3)
      expect(events.map { |e| e.data[:message] }).to eq(['Event 0', 'Event 1', 'Event 2'])
    end

    it 'supports event IDs in sequence' do
      producer = test_producer_class.new
      event = producer.next_event
      expect(event.id).to eq('0')
      event = producer.next_event
      expect(event.id).to eq('1')
    end

    it 'allows tracking connection state in subclass' do
      producer = test_producer_class.new
      expect(producer.connected).to be false
      producer.on_connect
      expect(producer.connected).to be true
      producer.on_disconnect
      expect(producer.connected).to be false
    end

    it 'returns nil to signal end of stream' do
      producer = test_producer_class.new
      3.times { producer.next_event } # Exhaust the stream
      final_event = producer.next_event
      expect(final_event).to be_nil
    end

    it 'maintains state across multiple next_event calls' do
      producer = test_producer_class.new
      expect(producer.count).to eq(0)
      producer.next_event
      expect(producer.count).to eq(1)
      producer.next_event
      expect(producer.count).to eq(2)
    end
  end

  describe 'integration with SseEvent' do
    let(:test_producer_class) do
      Class.new(described_class) do
        def initialize(event_data)
          @event_data = event_data
          @index = 0
        end

        def next_event
          return nil if @index >= @event_data.length

          event_config = @event_data[@index]
          @index += 1

          Spikard::SseEvent.new(
            data: event_config[:data],
            event_type: event_config[:event_type],
            id: event_config[:id],
            retry_ms: event_config[:retry_ms]
          )
        end
      end
    end

    it 'creates events with all optional attributes' do
      event_data = [
        {
          data: { msg: 'test' },
          event_type: 'message',
          id: '1',
          retry_ms: 1000
        }
      ]
      producer = test_producer_class.new(event_data)
      event = producer.next_event
      expect(event.data).to eq({ msg: 'test' })
      expect(event.event_type).to eq('message')
      expect(event.id).to eq('1')
      expect(event.retry_ms).to eq(1000)
    end

    it 'supports multiple events with different structures' do
      event_data = [
        { data: { count: 1 }, event_type: 'type1', id: '1', retry_ms: 1000 },
        { data: { count: 2 }, event_type: 'type2', id: '2', retry_ms: nil },
        { data: { count: 3 }, event_type: nil, id: nil, retry_ms: nil }
      ]
      producer = test_producer_class.new(event_data)
      events = []
      loop do
        event = producer.next_event
        break if event.nil?

        events << event
      end
      expect(events.length).to eq(3)
      expect(events[0].event_type).to eq('type1')
      expect(events[1].event_type).to eq('type2')
      expect(events[2].event_type).to be_nil
    end
  end
end
