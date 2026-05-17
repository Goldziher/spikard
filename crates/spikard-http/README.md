# spikard-http

High-performance HTTP server for Spikard with a tower-http middleware stack (compression, rate limiting, timeouts, auth, CORS, request IDs), the `Handler` trait used by every language binding, and lifecycle hooks (`onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`).

Most users should depend on [`spikard`](https://crates.io/crates/spikard) instead — it re-exports the HTTP runtime with sensible defaults.

For framework documentation, see the [Spikard repository](https://github.com/Goldziher/spikard).

## License

MIT — see [LICENSE](https://github.com/Goldziher/spikard/blob/main/LICENSE).
