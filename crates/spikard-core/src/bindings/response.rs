use crate::CompressionConfig;
use brotli::CompressorWriter;
use flate2::Compression;
use flate2::write::GzEncoder;
use std::collections::HashMap;
use std::io::Write;

/// Minimal response container shared by bindings.
#[derive(Clone, Debug)]
pub struct RawResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl RawResponse {
    /// Construct a new response.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(status: u16, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        Self { status, headers, body }
    }

    /// Apply compression filters if the response qualifies.
    pub fn apply_compression(&mut self, request_headers: &HashMap<String, String>, compression: &CompressionConfig) {
        if self.body.is_empty() || self.status == 206 {
            return;
        }
        if self
            .headers
            .keys()
            .any(|key| key.eq_ignore_ascii_case("content-encoding"))
        {
            return;
        }
        if self.body.len() < compression.min_size {
            return;
        }

        let accept_encoding = header_value(request_headers, "Accept-Encoding").map(str::to_ascii_lowercase);
        let accepts_brotli = accept_encoding.as_ref().is_some_and(|value| value.contains("br"));
        if compression.brotli && accepts_brotli && self.try_compress_brotli(compression) {
            return;
        }

        let accepts_gzip = accept_encoding.as_ref().is_some_and(|value| value.contains("gzip"));
        if compression.gzip && accepts_gzip {
            self.try_compress_gzip(compression);
        }
    }

    fn try_compress_brotli(&mut self, compression: &CompressionConfig) -> bool {
        let quality = compression.quality.min(11);
        let mut writer = CompressorWriter::new(Vec::new(), 4096, quality, 22);
        if writer.write_all(&self.body).is_err() || writer.flush().is_err() {
            return false;
        }
        let compressed = writer.into_inner();
        if compressed.is_empty() {
            return false;
        }
        self.finalize_encoded_body("br", compressed);
        true
    }

    fn try_compress_gzip(&mut self, compression: &CompressionConfig) -> bool {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::new(compression.quality));
        if encoder.write_all(&self.body).is_err() {
            return false;
        }
        let compressed = encoder.finish().unwrap_or_else(|_| Vec::new());
        if compressed.is_empty() {
            return false;
        }
        self.finalize_encoded_body("gzip", compressed);
        true
    }

    fn finalize_encoded_body(&mut self, encoding: &str, compressed: Vec<u8>) {
        self.body = compressed;
        self.headers
            .insert("content-encoding".to_string(), encoding.to_string());
        self.headers.insert("vary".to_string(), "Accept-Encoding".to_string());
        self.headers
            .insert("content-length".to_string(), self.body.len().to_string());
    }
}

fn header_value<'a>(headers: &'a HashMap<String, String>, name: &str) -> Option<&'a str> {
    headers.iter().find_map(|(key, value)| {
        if key.eq_ignore_ascii_case(name) {
            Some(value.as_str())
        } else {
            None
        }
    })
}

/// Pre-rendered static asset produced by the CLI bundler.
#[derive(Clone, Debug)]
pub struct StaticAsset {
    pub route: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl StaticAsset {
    /// Build a response snapshot if the incoming request targets this asset.
    #[must_use]
    pub fn serve(&self, method: &str, normalized_path: &str) -> Option<RawResponse> {
        if !method.eq_ignore_ascii_case("GET") && !method.eq_ignore_ascii_case("HEAD") {
            return None;
        }
        if self.route != normalized_path {
            return None;
        }

        let mut headers = self.headers.clone();
        headers
            .entry("content-length".to_string())
            .or_insert_with(|| self.body.len().to_string());
        let body = if method.eq_ignore_ascii_case("HEAD") {
            Vec::new()
        } else {
            self.body.clone()
        };

        Some(RawResponse::new(200, headers, body))
    }
}
