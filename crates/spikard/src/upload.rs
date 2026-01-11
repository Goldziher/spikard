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
    /// Create a new `UploadFile` instance.
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

        let decoded_content = if content_encoding.as_deref() == Some("base64")
            || (content_encoding.is_none() && Self::is_base64(&content))
        {
            use base64::{Engine as _, engine::general_purpose::STANDARD};
            STANDARD.decode(&content).map_or_else(|_| content.clone(), Bytes::from)
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
    #[must_use]
    pub const fn as_bytes(&self) -> &Bytes {
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
    fn creates_and_initializes_upload_file() {
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

        let file = UploadFile::new("file.bin".to_string(), Bytes::from("data"), None, None, None);
        assert_eq!(file.content_type_or_default(), "application/octet-stream");

        let content = Bytes::from("Hello, World!");
        let file = UploadFile::new("test.txt".to_string(), content, None, None, None);
        assert_eq!(file.size, Some(13));

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
    fn implements_read_and_seek_traits() {
        let mut file = UploadFile::new("test.txt".to_string(), Bytes::from("Hello, World!"), None, None, None);
        let mut buf = [0u8; 5];
        let n = file.read(&mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"Hello");

        let mut file = UploadFile::new("test.txt".to_string(), Bytes::from("Hello, World!"), None, None, None);
        file.seek(SeekFrom::Start(7)).unwrap();
        let mut buf = [0u8; 5];
        let n = file.read(&mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"World");
    }

    #[test]
    fn invalid_base64_falls_back_to_raw_content() {
        let invalid_base64 = Bytes::from("!!!invalid base64!!!");

        let file = UploadFile::new(
            "test.txt".to_string(),
            invalid_base64.clone(),
            None,
            None,
            Some("base64".to_string()),
        );

        assert_eq!(file.as_bytes(), &invalid_base64);
    }

    #[test]
    fn handles_various_filenames_and_content() {
        let file = UploadFile::new(String::new(), Bytes::from("content"), None, None, None);
        assert_eq!(file.filename, "");

        let file = UploadFile::new(
            "test-file_2024 (1).txt".to_string(),
            Bytes::from("content"),
            None,
            None,
            None,
        );
        assert_eq!(file.filename, "test-file_2024 (1).txt");

        let file = UploadFile::new("empty.txt".to_string(), Bytes::from(""), None, None, None);
        assert_eq!(file.size, Some(0));
        assert_eq!(file.as_bytes().len(), 0);

        let large_content = Bytes::from(vec![42u8; 1_000_000]);
        let file = UploadFile::new("large.bin".to_string(), large_content, None, None, None);
        assert_eq!(file.size, Some(1_000_000));
        assert_eq!(file.as_bytes().len(), 1_000_000);

        let file = UploadFile::new(
            "test.json".to_string(),
            Bytes::from("{}"),
            Some("application/json".to_string()),
            None,
            None,
        );
        assert_eq!(file.content_type_or_default(), "application/json");
    }

    #[test]
    fn read_to_string_with_utf8_content() {
        let file = UploadFile::new(
            "test.txt".to_string(),
            Bytes::from("Hello, UTF-8: 你好"),
            None,
            None,
            None,
        );
        let content = file.read_to_string().unwrap();
        assert_eq!(content, "Hello, UTF-8: 你好");
    }

    #[test]
    fn read_to_string_with_invalid_utf8() {
        let invalid_utf8 = Bytes::from(vec![0xFF, 0xFE, 0xFD]);
        let file = UploadFile::new("binary.bin".to_string(), invalid_utf8, None, None, None);
        let result = file.read_to_string();
        assert!(result.is_err());
    }

    #[test]
    fn seek_with_all_variants() {
        let mut file = UploadFile::new("test.txt".to_string(), Bytes::from("0123456789"), None, None, None);
        assert_eq!(file.seek(SeekFrom::Start(5)).unwrap(), 5);

        let mut file = UploadFile::new("test.txt".to_string(), Bytes::from("0123456789"), None, None, None);
        file.seek(SeekFrom::Start(3)).unwrap();
        assert_eq!(file.seek(SeekFrom::Current(2)).unwrap(), 5);

        let mut file = UploadFile::new("test.txt".to_string(), Bytes::from("0123456789"), None, None, None);
        assert_eq!(file.seek(SeekFrom::End(0)).unwrap(), 10);

        let mut file = UploadFile::new("test.txt".to_string(), Bytes::from("0123456789"), None, None, None);
        assert_eq!(file.seek(SeekFrom::End(-3)).unwrap(), 7);
    }

    #[test]
    fn read_partial_then_read_rest() {
        let mut file = UploadFile::new("test.txt".to_string(), Bytes::from("Hello, World!"), None, None, None);

        let mut buf1 = [0u8; 5];
        let n1 = file.read(&mut buf1).unwrap();
        assert_eq!(n1, 5);
        assert_eq!(&buf1, b"Hello");

        let mut buf2 = [0u8; 20];
        let n2 = file.read(&mut buf2).unwrap();
        assert_eq!(n2, 8);
        assert_eq!(&buf2[..8], b", World!");
    }

    #[test]
    fn size_handling_and_cloning() {
        let file = UploadFile::new("test.txt".to_string(), Bytes::from("Hello"), None, Some(1000), None);
        assert_eq!(file.size, Some(1000));

        let file = UploadFile::new("test.txt".to_string(), Bytes::from("Hello, World!"), None, None, None);
        assert_eq!(file.size, Some(13));

        let file1 = UploadFile::new(
            "test.txt".to_string(),
            Bytes::from("Hello"),
            Some("text/plain".to_string()),
            None,
            None,
        );
        let file2 = file1.clone();

        assert_eq!(file1.filename, file2.filename);
        assert_eq!(file1.content_type, file2.content_type);
        assert_eq!(file1.as_bytes(), file2.as_bytes());
    }

    #[test]
    fn base64_with_and_without_padding() {
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        let original = "A";
        let encoded = STANDARD.encode(original.as_bytes());
        assert_eq!(encoded, "QQ==");
        let file = UploadFile::new(
            "test.txt".to_string(),
            Bytes::from(encoded),
            None,
            None,
            Some("base64".to_string()),
        );
        assert_eq!(file.read_to_string().unwrap(), original);

        let original = "Hello";
        let encoded = STANDARD.encode(original.as_bytes());
        assert_eq!(encoded, "SGVsbG8=");
        let file = UploadFile::new(
            "test.txt".to_string(),
            Bytes::from(encoded),
            None,
            None,
            Some("base64".to_string()),
        );
        assert_eq!(file.read_to_string().unwrap(), original);

        let valid_base64 = "SGVsbG8gV29ybGQ=";
        let file = UploadFile::new("test.txt".to_string(), Bytes::from(valid_base64), None, None, None);
        let content = file.as_bytes();
        assert!(content.len() <= valid_base64.len());
    }

    #[test]
    fn multiple_seeks_and_reads() {
        let mut file = UploadFile::new("test.txt".to_string(), Bytes::from("0123456789"), None, None, None);

        file.seek(SeekFrom::Start(2)).unwrap();
        let mut buf1 = [0u8; 3];
        let n1 = file.read(&mut buf1).unwrap();
        assert_eq!(n1, 3);
        assert_eq!(&buf1, b"234");

        file.seek(SeekFrom::Current(2)).unwrap();
        let mut buf2 = [0u8; 2];
        let n2 = file.read(&mut buf2).unwrap();
        assert_eq!(n2, 2);
        assert_eq!(&buf2, b"78");
    }

    #[test]
    fn content_encoding_and_binary_handling() {
        let file = UploadFile::new(
            "test.txt".to_string(),
            Bytes::from("content"),
            None,
            None,
            Some("gzip".to_string()),
        );
        assert_eq!(file.content_encoding, Some("gzip".to_string()));

        let binary_data = Bytes::from(vec![0, 1, 2, 255, 254, 253]);
        let file = UploadFile::new("binary.bin".to_string(), binary_data.clone(), None, None, None);
        assert_eq!(file.as_bytes(), &binary_data);
    }
}
