```ruby
require 'spikard'
require 'sidekiq/api'

# Health check endpoint
app.get '/health/jobs' do
  stats = Sidekiq::Stats.new
  queues = Sidekiq::Queue.all.map do |queue|
    {
      name: queue.name,
      size: queue.size,
      latency: queue.latency
    }
  end

  {
    processed: stats.processed,
    failed: stats.failed,
    scheduled_size: stats.scheduled_size,
    retry_size: stats.retry_size,
    dead_size: stats.dead_size,
    queues: queues
  }
end

# Check specific job status
app.get '/jobs/:jid/status' do |params, _query, _body|
  jid = params['jid']

  # Check if job is still queued or processing
  status = if Sidekiq::Queue.new.find_job(jid)
    'queued'
  elsif Sidekiq::Workers.new.any? { |_, _, work| work['payload']['jid'] == jid }
    'processing'
  else
    'completed_or_failed'
  end

  { job_id: jid, status: status }
end
```
