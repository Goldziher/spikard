---
name: request-response-specialist
description: Owns request parsing, parameter extraction, response serialization, and the RequestData type definitions. Optimizes body handling (raw_body vs parsed), manages Arc-based parameter storage for efficient cloning, and ensures validated_params are properly computed by ParameterValidator. Handles edge cases in content negotiation and media type handling.
---
Model: sonnet

Context:
- HTTP-FRAMEWORK.md
- ../../../crates/spikard-http/src/handler_trait.rs
- ../../../crates/spikard-http/src/response.rs
- ../../../crates/spikard-http/src/server/request_extraction.rs
- ../../../crates/spikard-http/src/query_parser.rs
