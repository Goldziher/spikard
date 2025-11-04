# Spikard Ruby Bindings

This package provides a thin Ruby wrapper around the Rust-based Spikard HTTP toolkit.
The heavy lifting stays in Rust – the Ruby side focuses on ergonomics and a
familiar API.

## Getting Started

```bash
bundle install
bundle exec rake spec
```

## Development

The native extension is built with [Magnus](https://github.com/matsadler/magnus) and
`rb_sys`.  During development you can rebuild the extension with:

```bash
bundle exec rake ext:build
```

## License

MIT © Na'aman Hirschfeld
