# Spikard Ruby Bindings

High-performance Ruby HTTP framework powered by Rust using [Magnus](https://github.com/matsadler/magnus).

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'spikard'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install spikard
```

## Usage

### Basic Server

```ruby
require 'spikard'

# Define routes
routes = [
  {
    method: 'GET',
    path: '/',
    handler_name: 'root',
    is_async: false
  },
  {
    method: 'GET',
    path: '/hello/:name',
    handler_name: 'hello',
    is_async: false
  }
]

# Define handlers
handlers = {
  'root' => ->(request) {
    { message: 'Hello, world!' }
  },
  'hello' => ->(request) {
    name = request[:params]['name']
    { message: "Hello, #{name}!" }
  }
}

# Create app
app = Spikard.create_app(routes, handlers)

# Run server
Spikard.run_server(app, '127.0.0.1', 8000)
```

### Testing

Spikard provides an in-memory test client for testing your handlers without starting an actual server:

```ruby
require 'spikard'

# Create app
app = Spikard.create_app(routes, handlers)

# Create test client
client = Spikard.create_test_client(app)

# Make requests
response = client.get('/')
puts response.status_code  # => 200
puts response.json         # => { "message" => "Hello, world!" }

# Test with path parameters
response = client.get('/hello/Ruby')
puts response.json         # => { "message" => "Hello, Ruby!" }
```

## Architecture

The Ruby bindings use Magnus to bridge Ruby and Rust code:

- **`RubyHandler`**: Implements the `spikard_http::Handler` trait, storing Ruby Proc objects and calling them for each request
- **`TestClient`**: In-memory testing client using axum-test
- **Request/Response conversion**: Automatic conversion between Ruby hashes and JSON

## Development

### Building

Build the native extension:

```bash
export RUBY=/path/to/ruby
cargo build
```

### Testing

Run the test suite:

```bash
bundle exec rake test
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/Goldziher/spikard.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
