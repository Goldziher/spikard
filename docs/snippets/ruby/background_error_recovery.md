```ruby
# app/jobs/resilient_job.rb
class ResilientJob
  include Sidekiq::Job
  sidekiq_options retry: 5, dead: true

  sidekiq_retries_exhausted do |msg, _ex|
    # Move to dead letter queue or log
    DeadLetterQueue.push(msg)
    logger.error("Job exhausted retries: #{msg['jid']}")
  end

  def perform(data)
    # Idempotent operation
    return if already_processed?(data['id'])

    process_with_retry(data)
    mark_processed(data['id'])
  end

  private

  def process_with_retry(data)
    attempt = 0
    max_attempts = 3

    begin
      external_api_call(data)
    rescue => e
      attempt += 1
      raise if attempt >= max_attempts
      sleep(2 ** attempt)
      retry
    end
  end
end
```
