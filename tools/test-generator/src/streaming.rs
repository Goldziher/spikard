use anyhow::{Context, Result};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use spikard_codegen::openapi::{Fixture, FixtureStreamChunk, FixtureStreaming};

/// Convenience struct with precomputed streaming metadata for generators/tests
pub struct StreamingFixtureData<'a> {
    pub streaming: &'a FixtureStreaming,
    pub expected_bytes: Vec<u8>,
    pub is_text_only: bool,
}

/// Returns streaming metadata if the fixture defines a streaming response
pub fn streaming_data(fixture: &Fixture) -> Result<Option<StreamingFixtureData<'_>>> {
    let Some(streaming) = fixture.streaming.as_ref() else {
        return Ok(None);
    };

    let mut expected = Vec::new();
    let mut is_text_only = true;

    for chunk in &streaming.chunks {
        let (bytes, is_text_chunk) = chunk_bytes_with_meta(chunk)?;
        expected.extend(bytes);
        if !is_text_chunk {
            is_text_only = false;
        }
    }

    Ok(Some(StreamingFixtureData {
        streaming,
        expected_bytes: expected,
        is_text_only,
    }))
}

pub fn chunk_bytes(chunk: &FixtureStreamChunk) -> Result<Vec<u8>> {
    match chunk {
        FixtureStreamChunk::Text { value } => Ok(value.as_bytes().to_vec()),
        FixtureStreamChunk::Bytes { base64 } => BASE64_STANDARD
            .decode(base64)
            .with_context(|| format!("Invalid base64 chunk: {}", base64)),
    }
}

fn chunk_bytes_with_meta(chunk: &FixtureStreamChunk) -> Result<(Vec<u8>, bool)> {
    match chunk {
        FixtureStreamChunk::Text { .. } => Ok((chunk_bytes(chunk)?, true)),
        FixtureStreamChunk::Bytes { .. } => Ok((chunk_bytes(chunk)?, false)),
    }
}
