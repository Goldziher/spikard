```ruby
# Gemfile
gem 'sidekiq'
gem 'redis'

# config/sidekiq.rb
require 'sidekiq'

Sidekiq.configure_server do |config|
  config.redis = { url: ENV['REDIS_URL'] || 'redis://localhost:6379/0' }
end

Sidekiq.configure_client do |config|
  config.redis = { url: ENV['REDIS_URL'] || 'redis://localhost:6379/0' }
end

# app/jobs/process_upload_job.rb
class ProcessUploadJob
  include Sidekiq::Job
  sidekiq_options queue: :default, retry: 3

  def perform(file_id)
    file = File.find(file_id)
    raise "File not found: #{file_id}" unless file

    # Process file asynchronously
    process_file(file)
    notify_completion(file)
  rescue StandardError => e
    logger.error("Failed to process file #{file_id}: #{e.message}")
    raise # Let Sidekiq handle retry
  end

  private

  def process_file(file)
    # Heavy processing work
    sleep 5 # Simulate long operation
  end

  def notify_completion(file)
    # Send notification
  end
end

# app/jobs/send_email_job.rb
class SendEmailJob
  include Sidekiq::Job
  sidekiq_options queue: :mailers, retry: 5

  def perform(user_id, template, params = {})
    user = User.find(user_id)
    mailer = EmailService.new(user.email)
    mailer.send_template(template, params)
  end
end

# In Spikard handler
require 'spikard'
require_relative 'jobs/process_upload_job'
require_relative 'jobs/send_email_job'

app = Spikard::Application.new

app.post '/upload' do |req|
  file_id = req.params['file_id']
  job = ProcessUploadJob.perform_async(file_id)

  { status: 'processing', job_id: job }
end

app.post '/signup' do |req|
  user = User.create!(req.json)
  SendEmailJob.perform_async(user.id, 'welcome', { name: user.name })

  { id: user.id, email: user.email }
end
```
