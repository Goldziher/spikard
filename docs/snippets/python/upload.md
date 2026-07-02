```python
from spikard import Spikard, UploadFile
import os
import uuid
from pathlib import Path

app = Spikard()

# Basic upload handler
@app.post("/upload")
async def upload(file: UploadFile) -> dict:
    content = file.read()
    return {"filename": file.filename, "size": len(content)}

# Complete upload handler with validation and storage
@app.post("/upload/complete")
async def upload_with_validation(file: UploadFile) -> dict:
    # Validate file size (10MB limit)
    MAX_SIZE = 10 * 1024 * 1024
    content = file.read()
    if len(content) > MAX_SIZE:
        raise ValueError(f"File size {len(content)} exceeds {MAX_SIZE} bytes")

    # Validate MIME type
    ALLOWED_TYPES = ["image/jpeg", "image/png", "image/gif", "application/pdf"]
    if file.content_type not in ALLOWED_TYPES:
        raise ValueError(f"File type {file.content_type} not allowed")

    # Prevent path traversal - sanitize filename
    safe_filename = os.path.basename(file.filename)
    unique_filename = f"{uuid.uuid4()}_{safe_filename}"

    # Save to local filesystem
    upload_dir = Path("/var/uploads")
    upload_dir.mkdir(parents=True, exist_ok=True)
    file_path = upload_dir / unique_filename

    with open(file_path, "wb") as f:
        f.write(content)

    return {
        "filename": safe_filename,
        "stored_as": unique_filename,
        "size": len(content),
        "content_type": file.content_type,
        "url": f"/files/{unique_filename}"
    }

# Upload to S3/cloud storage
@app.post("/upload/s3")
async def upload_to_s3(file: UploadFile) -> dict:
    import boto3

    s3_client = boto3.client('s3')
    bucket_name = "my-uploads-bucket"

    # Validate and sanitize
    safe_filename = os.path.basename(file.filename)
    s3_key = f"uploads/{uuid.uuid4()}/{safe_filename}"

    # Upload to S3
    s3_client.put_object(
        Bucket=bucket_name,
        Key=s3_key,
        Body=file.read(),
        ContentType=file.content_type
    )

    return {
        "filename": safe_filename,
        "s3_key": s3_key,
        "url": f"https://{bucket_name}.s3.amazonaws.com/{s3_key}"
    }
```
