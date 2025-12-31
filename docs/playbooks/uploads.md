# File Uploads

Handle multipart uploads with consistent patterns per binding.

## Upload handler

=== "Python"

    --8<-- "snippets/python/upload.md"

=== "TypeScript"

    --8<-- "snippets/typescript/upload.md"

=== "Ruby"

    --8<-- "snippets/ruby/upload.md"

=== "PHP"

    --8<-- "snippets/php/upload.md"

=== "Rust"

    --8<-- "snippets/rust/upload.md"

## Validation

Always validate uploaded files to prevent security issues and resource exhaustion:

**File size limits**: Prevent denial-of-service attacks by enforcing maximum file sizes before processing.

**MIME type validation**: Check the `content_type` field to ensure only expected file types are accepted. Never trust file extensions alone.

**Filename sanitization**: Use `path.basename()` or equivalent to prevent directory traversal attacks. Malicious filenames like `../../etc/passwd` could overwrite system files.

**Content validation**: For images, consider using libraries to validate that the file content matches the declared MIME type.

## Storage strategies

### Local filesystem

Store files on the server's filesystem for simple deployments:

- Generate unique filenames using UUIDs to prevent collisions
- Create organized directory structures (e.g., by date or user ID)
- Set appropriate file permissions
- Consider disk space monitoring

### Cloud storage (S3, GCS, Azure Blob)

For production deployments, use object storage:

- Direct uploads reduce server load
- Built-in redundancy and scalability
- CDN integration for faster downloads
- Lifecycle policies for automatic cleanup

### Database storage

Store small files (< 1MB) directly in databases:

- Simplifies backup and replication
- Keeps data and metadata together
- Not recommended for large files due to performance impact

## Testing file uploads

Test your upload handlers with different scenarios:

=== "Python"

    ```python
    import pytest
    from spikard.testing import TestClient

    def test_upload_success(client: TestClient):
        response = client.post(
            "/upload/complete",
            files={"file": ("test.jpg", b"fake image data", "image/jpeg")}
        )
        assert response.status_code == 200
        assert response.json()["filename"] == "test.jpg"

    def test_upload_exceeds_size_limit(client: TestClient):
        large_file = b"x" * (11 * 1024 * 1024)  # 11MB
        response = client.post(
            "/upload/complete",
            files={"file": ("large.jpg", large_file, "image/jpeg")}
        )
        assert response.status_code == 400

    def test_upload_invalid_type(client: TestClient):
        response = client.post(
            "/upload/complete",
            files={"file": ("malware.exe", b"fake data", "application/x-msdownload")}
        )
        assert response.status_code == 400

    def test_upload_path_traversal(client: TestClient):
        response = client.post(
            "/upload/complete",
            files={"file": ("../../etc/passwd", b"fake data", "image/jpeg")}
        )
        # Should sanitize to just "passwd"
        assert "../../" not in response.json()["stored_as"]
    ```

=== "TypeScript"

    ```typescript
    import { TestClient } from "spikard/testing";

    test("upload success", async () => {
      const client = new TestClient(app);
      const response = await client.post("/upload/complete", {
        file: {
          filename: "test.jpg",
          content: Buffer.from("fake image data"),
          content_type: "image/jpeg",
        },
      });
      expect(response.status).toBe(200);
      expect(response.json().filename).toBe("test.jpg");
    });

    test("upload exceeds size limit", async () => {
      const client = new TestClient(app);
      const largeFile = Buffer.alloc(11 * 1024 * 1024); // 11MB
      const response = await client.post("/upload/complete", {
        file: {
          filename: "large.jpg",
          content: largeFile,
          content_type: "image/jpeg",
        },
      });
      expect(response.status).toBe(400);
    });
    ```

=== "Ruby"

    ```ruby
    require "rack/test"

    RSpec.describe "File uploads" do
      include Rack::Test::Methods

      def app
        @app
      end

      it "uploads file successfully" do
        post "/upload/complete", {
          file: Rack::Test::UploadedFile.new(
            StringIO.new("fake image data"),
            "image/jpeg",
            original_filename: "test.jpg"
          )
        }
        expect(last_response.status).to eq(200)
        expect(JSON.parse(last_response.body)["filename"]).to eq("test.jpg")
      end

      it "rejects files exceeding size limit" do
        large_file = StringIO.new("x" * (11 * 1024 * 1024))
        post "/upload/complete", {
          file: Rack::Test::UploadedFile.new(
            large_file,
            "image/jpeg",
            original_filename: "large.jpg"
          )
        }
        expect(last_response.status).to eq(400)
      end
    end
    ```

## Security considerations

- **Virus scanning**: For user-generated content, integrate antivirus scanning before storage
- **Content-Type spoofing**: Validate file signatures (magic bytes) in addition to MIME types
- **Resource limits**: Set timeouts for upload processing to prevent slowloris attacks
- **Access control**: Verify user permissions before allowing uploads
- **Temporary file cleanup**: Ensure temporary files are deleted after processing or on errors

## Tips
- Enforce size/type limits via middleware or schema where supported.
- For large uploads, stream chunks instead of reading all bytes into memory.
- Return metadata (filename, size, type) and store bytes in durable storage.
- Use presigned URLs for direct client-to-S3 uploads to bypass your server entirely.
- Implement rate limiting to prevent abuse of upload endpoints.
