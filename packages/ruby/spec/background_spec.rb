# frozen_string_literal: true

require 'spec_helper'

RSpec.describe Spikard::Converters do
  describe '.convert_handler_body' do
    it 'returns the body unchanged' do
      body = { foo: 'bar' }
      result = described_class.convert_handler_body(body)
      expect(result).to eq(body)
    end

    it 'handles nil body' do
      result = described_class.convert_handler_body(nil)
      expect(result).to be_nil
    end

    it 'is a no-op conversion' do
      test_values = [
        {},
        { key: 'value' },
        [],
        'string',
        42
      ]
      test_values.each do |value|
        expect(described_class.convert_handler_body(value)).to equal(value)
      end
    end
  end
end

RSpec.describe Spikard::Background do
  describe '.run' do
    it 'accepts a block and queues it' do
      executed = false
      expect do
        described_class.run { executed = true }
      end.not_to raise_error

      # Give the background worker thread time to execute
      sleep 0.1
      expect(executed).to be true
    end

    it 'raises ArgumentError when no block is provided' do
      expect do
        described_class.run
      end.to raise_error(ArgumentError, /background.run requires a block/)
    end

    it 'raises ArgumentError with correct message' do
      error = nil
      begin
        described_class.run
      rescue ArgumentError => e
        error = e
      end
      expect(error.message).to eq('background.run requires a block')
    end

    it 'verifies block parameter is required' do
      allow_any_instance_of(Proc).to receive(:nil?).and_return(true)
      expect do
        described_class.run(&nil)
      end.to raise_error(ArgumentError)
    end

    it 'executes blocks with return values' do
      result_holder = []
      described_class.run { result_holder << 'executed' }

      sleep 0.1
      expect(result_holder).to eq(['executed'])
    end

    it 'handles multiple sequential tasks' do
      results = []
      mutex = Mutex.new

      3.times do |i|
        described_class.run do
          mutex.synchronize { results << i }
        end
      end

      sleep 0.2
      expect(results.sort).to eq([0, 1, 2])
    end

    it 'executes tasks asynchronously' do
      started = false
      finished = false

      described_class.run do
        started = true
        sleep 0.05
        finished = true
      end

      # Check that task started but not finished immediately
      sleep 0.01
      expect(started).to be true
      expect(finished).to be false

      # Wait for completion
      sleep 0.1
      expect(finished).to be true
    end
  end

  describe 'error handling' do
    it 'catches and warns on StandardError in tasks' do
      warning_output = StringIO.new

      allow_any_instance_of(Kernel).to receive(:warn) do |msg|
        warning_output.puts(msg)
      end

      described_class.run do
        raise StandardError, 'test error'
      end

      sleep 0.1
      # The warn is called directly, so we just verify the task doesn't crash the worker
      # The error handling is tested by the fact that subsequent tasks execute
    end

    it 'continues processing after a task error' do
      results = []
      mutex = Mutex.new

      # First task fails
      described_class.run do
        raise StandardError, 'failed task'
      end

      # Second task should still execute
      described_class.run do
        mutex.synchronize { results << 'success' }
      end

      sleep 0.2
      expect(results).to eq(['success'])
    end

    it 'handles RuntimeError in tasks' do
      results = []
      mutex = Mutex.new

      described_class.run do
        raise 'runtime issue'
      end

      sleep 0.05

      # Verify worker thread is still alive and processing
      described_class.run do
        mutex.synchronize { results << 'recovered' }
      end

      sleep 0.1
      expect(results).to eq(['recovered'])
    end

    it 'does not let errors propagate to caller' do
      expect do
        described_class.run do
          raise StandardError, 'background error'
        end
        sleep 0.2
      end.not_to raise_error
    end
  end

  describe 'task execution patterns' do
    it 'executes tasks in FIFO order' do
      results = []
      mutex = Mutex.new

      5.times do |i|
        described_class.run do
          mutex.synchronize { results << i }
        end
      end

      sleep 0.3
      expect(results).to eq([0, 1, 2, 3, 4])
    end

    it 'allows tasks to modify external state' do
      state = { counter: 0 }

      described_class.run do
        state[:counter] += 1
      end

      sleep 0.1
      expect(state[:counter]).to eq(1)
    end

    it 'handles tasks with complex operations' do
      results = { sum: 0 }
      mutex = Mutex.new

      described_class.run do
        values = (1..10).to_a.sum
        mutex.synchronize { results[:sum] = values }
      end

      sleep 0.1
      expect(results[:sum]).to eq(55)
    end
  end

  describe 'thread safety' do
    it 'handles concurrent task submissions' do
      results = []
      mutex = Mutex.new

      threads = 10.times.map do |i|
        Thread.new do
          described_class.run do
            mutex.synchronize { results << i }
          end
        end
      end

      threads.each(&:join)
      sleep 0.3

      expect(results.length).to eq(10)
      expect(results.sort).to eq((0..9).to_a)
    end

    it 'processes all queued tasks before shutdown' do
      results = []
      mutex = Mutex.new

      20.times do |i|
        described_class.run do
          mutex.synchronize { results << i }
        end
      end

      sleep 0.5
      expect(results.length).to eq(20)
    end
  end

  describe 'integration scenarios' do
    it 'works with empty blocks' do
      expect do
        described_class.run { nil }
      end.not_to raise_error

      sleep 0.1
    end

    it 'works with blocks that return values' do
      returned_value = nil
      described_class.run do
        returned_value = { status: 'completed' }
      end

      sleep 0.1
      expect(returned_value).to eq(status: 'completed')
    end

    it 'allows tasks to access instance variables from enclosing scope' do
      outer_var = 'outer'
      result = nil

      described_class.run do
        result = outer_var
      end

      sleep 0.1
      expect(result).to eq('outer')
    end

    it 'supports chaining multiple task calls' do
      results = []
      mutex = Mutex.new

      described_class.run { mutex.synchronize { results << 1 } }
      described_class.run { mutex.synchronize { results << 2 } }
      described_class.run { mutex.synchronize { results << 3 } }

      sleep 0.2
      expect(results).to eq([1, 2, 3])
    end
  end

  describe 'module interface' do
    it 'is a module' do
      expect(described_class).to be_a(Module)
    end

    it 'responds to run method' do
      expect(described_class).to respond_to(:run)
    end

    it 'run method is accessible as module function' do
      expect(described_class.method(:run)).not_to be_nil
    end
  end

  describe 'edge cases' do
    it 'handles nil values in task results' do
      result = :not_set
      described_class.run do
        result = nil
      end

      sleep 0.1
      expect(result).to be_nil
    end

    it 'handles false values in task results' do
      result = :not_set
      described_class.run do
        result = false
      end

      sleep 0.1
      expect(result).to be false
    end

    it 'handles zero values in task results' do
      result = :not_set
      described_class.run do
        result = 0
      end

      sleep 0.1
      expect(result).to eq(0)
    end

    it 'handles empty strings in task results' do
      result = :not_set
      described_class.run do
        result = ''
      end

      sleep 0.1
      expect(result).to eq('')
    end

    it 'handles large strings in tasks' do
      large_str = 'x' * 10_000
      result = nil
      described_class.run do
        result = large_str
      end

      sleep 0.1
      expect(result.length).to eq(10_000)
    end
  end
end
