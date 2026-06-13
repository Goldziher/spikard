# frozen_string_literal: true

def on_request(&block)
  # Called before any other processing for each inbound request.
  @lifecycle_hooks.push(["on_request", block])
  self
end

def pre_validation(&block)
  # Called after parsing but before parameter validation.
  @lifecycle_hooks.push(["pre_validation", block])
  self
end

def pre_handler(&block)
  # Called after validation but before invoking the route handler.
  @lifecycle_hooks.push(["pre_handler", block])
  self
end

def on_response(&block)
  # Called after the handler returns but before the response is serialized.
  @lifecycle_hooks.push(["on_response", block])
  self
end

def on_error(&block)
  # Called when a handler returns an error.
  @lifecycle_hooks.push(["on_error", block])
  self
end

def websocket(path, &block)
  # Register a WebSocket upgrade handler at the given path.
  @registrations.push(["websocket", [path], block])
  self
end

def sse(path, &block)
  # Register an SSE event producer at the given path.
  @registrations.push(["sse", [path], block])
  self
end
