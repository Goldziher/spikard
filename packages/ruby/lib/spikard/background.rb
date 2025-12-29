# frozen_string_literal: true

module Spikard
  # Background job helpers.
  module Background
    module_function

    @queue = Queue.new
    @worker = nil
    @worker_mutex = Mutex.new
    SHUTDOWN = Object.new

    def ensure_worker
      return if @worker&.alive?

      @worker_mutex.synchronize do
        return if @worker&.alive?

        @worker = Thread.new do
          loop do
            job = @queue.pop
            break if job.equal?(SHUTDOWN)

            begin
              job.call
            rescue StandardError => e
              warn("[spikard.background] job failed: #{e.message}")
            end
          end
        end
      end
    end

    # Schedule a block to run after the response has been returned.
    def run(&block)
      raise ArgumentError, 'background.run requires a block' unless block

      ensure_worker
      @queue << block
    end

    # Stop the background worker thread to allow process shutdown.
    def shutdown
      @worker_mutex.synchronize do
        return unless @worker&.alive?

        @queue << SHUTDOWN
        @worker.join(1)
        @worker.kill if @worker.alive?
        @worker = nil
      end
    end

    at_exit do
      shutdown
    end
  end
end
