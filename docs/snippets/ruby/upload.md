```ruby
require "spikard"
require "securerandom"
require "fileutils"

app = Spikard::App.new

# Basic upload handler
app.post "/upload" do |_params, _query, body|
  file = body["file"]
  { filename: file[:filename], size: file[:tempfile].size }
end

# Complete upload handler with validation and storage
app.post "/upload/complete" do |_params, _query, body|
  file = body["file"]
  tempfile = file[:tempfile]
  filename = file[:filename]
  content_type = file[:type]

  # Validate file size (10MB limit)
  max_size = 10 * 1024 * 1024
  file_size = tempfile.size
  raise "File size #{file_size} exceeds #{max_size} bytes" if file_size > max_size

  # Validate MIME type
  allowed_types = ["image/jpeg", "image/png", "image/gif", "application/pdf"]
  raise "File type #{content_type} not allowed" unless allowed_types.include?(content_type)

  # Prevent path traversal - sanitize filename
  safe_filename = File.basename(filename)
  unique_filename = "#{SecureRandom.uuid}_#{safe_filename}"

  # Save to local filesystem
  upload_dir = "/var/uploads"
  FileUtils.mkdir_p(upload_dir)
  file_path = File.join(upload_dir, unique_filename)

  File.open(file_path, "wb") do |f|
    f.write(tempfile.read)
  end

  {
    filename: safe_filename,
    stored_as: unique_filename,
    size: file_size,
    content_type: content_type,
    url: "/files/#{unique_filename}"
  }
end

# Upload to S3/cloud storage
app.post "/upload/s3" do |_params, _query, body|
  require "aws-sdk-s3"

  file = body["file"]
  tempfile = file[:tempfile]
  filename = file[:filename]
  content_type = file[:type]

  s3_client = Aws::S3::Client.new(region: "us-east-1")
  bucket_name = "my-uploads-bucket"

  # Validate and sanitize
  safe_filename = File.basename(filename)
  s3_key = "uploads/#{SecureRandom.uuid}/#{safe_filename}"

  # Upload to S3
  s3_client.put_object(
    bucket: bucket_name,
    key: s3_key,
    body: tempfile.read,
    content_type: content_type
  )

  {
    filename: safe_filename,
    s3_key: s3_key,
    url: "https://#{bucket_name}.s3.amazonaws.com/#{s3_key}"
  }
end
```
