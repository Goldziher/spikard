```elixir
app = Spikard.App.new()

# Basic upload handler.
# Multipart file fields are parsed by the Rust core and delivered on
# conn.body as %{"filename" => ..., "size" => ..., "content_type" => ...,
# "content" => <base64-encoded bytes>}.
app =
  Spikard.App.post(app, "/upload", fn conn ->
    file = conn.body["file"]
    %{"filename" => file["filename"], "size" => file["size"]}
  end)

max_size = 10 * 1024 * 1024
allowed_types = ["image/jpeg", "image/png", "image/gif", "application/pdf"]

# Complete upload handler with validation and storage
app =
  Spikard.App.post(app, "/upload/complete", fn conn ->
    file = conn.body["file"]

    cond do
      file["size"] > max_size ->
        %{"error" => "File size #{file["size"]} exceeds #{max_size} bytes"}

      file["content_type"] not in allowed_types ->
        %{"error" => "File type #{file["content_type"]} not allowed"}

      true ->
        safe_filename = Path.basename(file["filename"])
        unique_filename = "#{System.unique_integer([:positive, :monotonic])}_#{safe_filename}"
        upload_dir = "/var/uploads"
        File.mkdir_p!(upload_dir)
        file_path = Path.join(upload_dir, unique_filename)
        File.write!(file_path, Base.decode64!(file["content"]))

        %{
          "filename" => safe_filename,
          "stored_as" => unique_filename,
          "size" => file["size"],
          "content_type" => file["content_type"],
          "url" => "/files/#{unique_filename}"
        }
    end
  end)

Spikard.App.run(app)
```
