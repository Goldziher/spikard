```ruby
# test/jobs/process_upload_job_test.rb
require 'test_helper'
require 'sidekiq/testing'

class ProcessUploadJobTest < ActiveSupport::TestCase
  setup do
    Sidekiq::Testing.fake! # Queue jobs, don't process
  end

  test "enqueues job with correct arguments" do
    assert_equal 0, ProcessUploadJob.jobs.size

    ProcessUploadJob.perform_async(123)

    assert_equal 1, ProcessUploadJob.jobs.size
    assert_equal [123], ProcessUploadJob.jobs.last['args']
  end

  test "processes upload successfully" do
    Sidekiq::Testing.inline! do # Actually run the job
      file = files(:sample_file)

      assert_difference -> { ProcessedFile.count }, 1 do
        ProcessUploadJob.perform_async(file.id)
      end
    end
  end

  test "handles missing file gracefully" do
    Sidekiq::Testing.inline! do
      assert_raises(RuntimeError) do
        ProcessUploadJob.perform_async(999999)
      end
    end
  end

  test "retries on transient failures" do
    Sidekiq::Testing.inline! do
      file = files(:sample_file)

      # Simulate transient error
      FileProcessor.stub(:process, -> { raise "Temporary error" }) do
        assert_raises(RuntimeError) do
          ProcessUploadJob.perform_async(file.id)
        end
      end
    end
  end
end
```
