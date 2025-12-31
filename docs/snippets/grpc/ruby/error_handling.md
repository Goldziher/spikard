```ruby
class UserServiceHandler < Spikard::Grpc::Handler
  def handle_request(request)
    case request.method_name
    when 'GetUser'
      get_user(request)
    else
      # Return error response
      Spikard::Grpc::Response.error(
        "Method not implemented: #{request.method_name}"
      )
    end
  rescue ArgumentError => e
    # Invalid argument error
    Spikard::Grpc::Response.error(e.message, { 'grpc-status' => 'INVALID_ARGUMENT' })
  rescue SecurityError => e
    # Authentication error
    Spikard::Grpc::Response.error(e.message, { 'grpc-status' => 'UNAUTHENTICATED' })
  rescue StandardError => e
    # Internal error
    Spikard::Grpc::Response.error("Internal error: #{e.message}")
  end
end
```
