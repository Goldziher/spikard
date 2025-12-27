# frozen_string_literal: true

module Spikard
  # Wrapper for Native::TestClient that provides convenient HTTP methods
  class TestClient
    def initialize(app)
      @client = Native::TestClient.new(app.to_routes_json, app.handlers)
    end

    def get(path, headers = {})
      request('GET', path, headers, nil)
    end

    def post(path, headers = {}, body = nil)
      request('POST', path, headers, body)
    end

    def put(path, headers = {}, body = nil)
      request('PUT', path, headers, body)
    end

    def delete(path, headers = {})
      request('DELETE', path, headers, nil)
    end

    def patch(path, headers = {}, body = nil)
      request('PATCH', path, headers, body)
    end

    def head(path, headers = {})
      request('HEAD', path, headers, nil)
    end

    def options(path, headers = {})
      request('OPTIONS', path, headers, nil)
    end

    def graphql(query, variables = nil, operation_name = nil)
      native_response = @client.graphql(query, variables, operation_name)
      Response.new(native_response)
    end

    def graphql_with_status(query, variables = nil, operation_name = nil)
      status, native_response = @client.graphql_with_status(query, variables, operation_name)
      [status, Response.new(native_response)]
    end

    private

    def request(method, path, headers, body)
      # Call the native request method and wrap in Response
      native_response = @client.request(method, path, {
        headers: headers,
        body: body
      })
      Response.new(native_response)
    end
  end
end
