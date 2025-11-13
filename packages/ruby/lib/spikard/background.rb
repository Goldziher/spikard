# frozen_string_literal: true

module Spikard
  # Background job helpers.
  module Background
    module_function

    @queue = Queue.new
    @worker = Thread.new do
      loop do
        job = @queue.pop
        begin
          job.call
        rescue StandardError => e
          warn("[spikard.background] job failed: #{e.message}")
        end
      end
    end

    # Schedule a block to run after the response has been returned.
    def run(&block)
      raise ArgumentError, 'background.run requires a block' unless block

      @queue << block
    end
  end
end
