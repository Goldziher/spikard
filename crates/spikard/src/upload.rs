//! File upload handling for multipart/form-data requests.
//!
//! This module provides the `UploadFile` struct for handling file uploads,
//! designed to provide zero-copy access to uploaded file content with
//! automatic base64 decoding support.

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::io::{self, Cursor, Read, Seek, SeekFrom};

/// Represents an uploaded file from multipart/form-data requests.
///
/// This struct provides efficient access to file content with automatic
/// base64 decoding and implements standard I/O traits for compatibility.
///
/// # Example
///
/// ```rust
/// use spikard::UploadFile;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct UploadRequest {
///     file: UploadFile,
///     description: String,
/// }
///
/// // In a handler:
/// // let body: UploadRequest = ctx.json()?;
/// // let content = body.file.as_bytes();
/// // let filename = &body.file.filename;
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadFile {
    /// Original filename from the client
    pub filename: String,

    /// MIME type of the uploaded file
    #[serde(rename = "content_type")]
    pub content_type: Option<String>,

    /// Size of the file in bytes
    pub size: Option<usize>,

    /// File content (may be base64 encoded)
    #[serde(skip)]
    content: Bytes,

    /// Content encoding type
    #[serde(rename = "content_encoding")]
    content_encoding: Option<String>,

    /// Internal cursor for Read/Seek operations
    #[serde(skip)]
    cursor: Cursor<Bytes>,
}

impl UploadFile {
    /// Create a new UploadFile instance.
    ///
    /// # Arguments
    ///
    /// * `filename` - Original filename from the client
    /// * `content` - File contents (raw or base64 encoded)
    /// * `content_type` - MIME type (defaults to "application/octet-stream")
    /// * `size` - File size in bytes (computed from content if not provided)
    /// * `content_encoding` - Encoding type (e.g., "base64")
    pub fn new(
        filename: String,
        content: impl Into<Bytes>,
        content_type: Option<String>,
        size: Option<usize>,
        content_encoding: Option<String>,
    ) -> Self {
        let content: Bytes = content.into();

        // Decode content if base64 encoded
        let decoded_content = if content_encoding.as_deref() == Some("base64")
            || (content_encoding.is_none() && Self::is_base64(&content))
        {
            use base64::{Engine as _, engine::general_purpose::STANDARD};
            STANDARD
                .decode(&content)
                .map(Bytes::from)
                .unwrap_or_else(|_| content.clone())
        } else {
            content
        };

        let size = size.or_else(|| Some(decoded_content.len()));
        let cursor = Cursor::new(decoded_content.clone());

        Self {
            filename,
            content_type,
            size,
            content: decoded_content,
            content_encoding,
            cursor,
        }
    }

    /// Get the raw file content as bytes.
    ///
    /// This provides zero-copy access to the underlying buffer.
    pub fn as_bytes(&self) -> &Bytes {
        &self.content
    }

    /// Read the file content as a UTF-8 string.
    ///
    /// # Errors
    ///
    /// Returns an error if the content is not valid UTF-8.
    pub fn read_to_string(&self) -> io::Result<String> {
        String::from_utf8(self.content.to_vec()).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }

    /// Get the content type, defaulting to "application/octet-stream".
    pub fn content_type_or_default(&self) -> &str {
        self.content_type.as_deref().unwrap_or("application/octet-stream")
    }

    /// Check if bytes appear to be base64 encoded.
    fn is_base64(bytes: &[u8]) -> bool {
        // Simple heuristic: check if bytes match base64 pattern
        bytes
            .iter()
            .all(|&b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'=')
            && !bytes.is_empty()
    }
}

impl Read for UploadFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.cursor.read(buf)
    }
}

impl Seek for UploadFile {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.cursor.seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_upload_file_with_all_fields() {
        let content = Bytes::from("Hello, World!");
        let file = UploadFile::new(
            "test.txt".to_string(),
            content.clone(),
            Some("text/plain".to_string()),
            Some(13),
            None,
        );

        assert_eq!(file.filename, "test.txt");
        assert_eq!(file.content_type, Some("text/plain".to_string()));
        assert_eq!(file.size, Some(13));
        assert_eq!(file.as_bytes(), &content);
    }

    #[test]
    fn defaults_content_type() {
        let file = UploadFile::new("file.bin".to_string(), Bytes::from("data"), None, None, None);

        assert_eq!(file.content_type_or_default(), "application/octet-stream");
    }

    #[test]
    fn computes_size_from_content() {
        let content = Bytes::from("Hello, World!");
        let file = UploadFile::new("test.txt".to_string(), content, None, None, None);

        assert_eq!(file.size, Some(13));
    }

    #[test]
    fn reads_as_string() {
        let file = UploadFile::new("test.txt".to_string(), Bytes::from("Hello, World!"), None, None, None);

        assert_eq!(file.read_to_string().unwrap(), "Hello, World!");
    }

    #[test]
    fn decodes_base64_content() {
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        let original = "Hello, World!";
        let encoded = STANDARD.encode(original.as_bytes());

        let file = UploadFile::new(
            "test.txt".to_string(),
            Bytes::from(encoded),
            None,
            None,
            Some("base64".to_string()),
        );

        assert_eq!(file.read_to_string().unwrap(), original);
    }

    #[test]
    fn implements_read_trait() {
        let mut file = UploadFile::new("test.txt".to_string(), Bytes::from("Hello, World!"), None, None, None);

        let mut buf = [0u8; 5];
        let n = file.read(&mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"Hello");
    }

    #[test]
    fn implements_seek_trait() {
        let mut file = UploadFile::new("test.txt".to_string(), Bytes::from("Hello, World!"), None, None, None);

        // Seek to position 7
        file.seek(SeekFrom::Start(7)).unwrap();

        let mut buf = [0u8; 5];
        let n = file.read(&mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"World");
    }
}
