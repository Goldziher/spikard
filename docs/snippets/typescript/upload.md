```typescript
import { Spikard, UploadFile } from "spikard";
import { promises as fs } from "fs";
import path from "path";
import { v4 as uuidv4 } from "uuid";

const app = new Spikard();

// Basic upload handler
app.addRoute(
  { method: "POST", path: "/upload", handler_name: "upload", is_async: true },
  async (req) => {
    const body = req.json<{ file: UploadFile; description?: string }>();
    const size = body.file.size;
    return { filename: body.file.filename, size };
  },
);

// Complete upload handler with validation and storage
app.addRoute(
  { method: "POST", path: "/upload/complete", handler_name: "uploadComplete", is_async: true },
  async (req) => {
    const body = req.json<{ file: UploadFile }>();
    const file = body.file;

    // Validate file size (10MB limit)
    const MAX_SIZE = 10 * 1024 * 1024;
    if (file.size > MAX_SIZE) {
      throw new Error(`File size ${file.size} exceeds ${MAX_SIZE} bytes`);
    }

    // Validate MIME type
    const ALLOWED_TYPES = ["image/jpeg", "image/png", "image/gif", "application/pdf"];
    if (!ALLOWED_TYPES.includes(file.content_type)) {
      throw new Error(`File type ${file.content_type} not allowed`);
    }

    // Prevent path traversal - sanitize filename
    const safeFilename = path.basename(file.filename);
    const uniqueFilename = `${uuidv4()}_${safeFilename}`;

    // Save to local filesystem
    const uploadDir = "/var/uploads";
    await fs.mkdir(uploadDir, { recursive: true });
    const filePath = path.join(uploadDir, uniqueFilename);

    await fs.writeFile(filePath, Buffer.from(file.content));

    return {
      filename: safeFilename,
      stored_as: uniqueFilename,
      size: file.size,
      content_type: file.content_type,
      url: `/files/${uniqueFilename}`,
    };
  },
);

// Upload to S3/cloud storage
app.addRoute(
  { method: "POST", path: "/upload/s3", handler_name: "uploadToS3", is_async: true },
  async (req) => {
    const { S3Client, PutObjectCommand } = require("@aws-sdk/client-s3");
    const body = req.json<{ file: UploadFile }>();
    const file = body.file;

    const s3Client = new S3Client({ region: "us-east-1" });
    const bucketName = "my-uploads-bucket";

    // Validate and sanitize
    const safeFilename = path.basename(file.filename);
    const s3Key = `uploads/${uuidv4()}/${safeFilename}`;

    // Upload to S3
    await s3Client.send(
      new PutObjectCommand({
        Bucket: bucketName,
        Key: s3Key,
        Body: Buffer.from(file.content),
        ContentType: file.content_type,
      }),
    );

    return {
      filename: safeFilename,
      s3_key: s3Key,
      url: `https://${bucketName}.s3.amazonaws.com/${s3Key}`,
    };
  },
);
```
