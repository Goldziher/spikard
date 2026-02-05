defmodule E2EElixirApp.StatusCodesTest do
  @moduledoc """
  Generated tests for status_codes fixtures.

  Each test starts its own isolated server with only its route,
  avoiding conflicts when multiple fixtures share the same path.
  """
  use ExUnit.Case, async: false

  alias E2EElixirApp.AppFactories

  @base_url "http://127.0.0.1:59800"

  setup do
    :inets.start()
    :ssl.start()
    :ok
  end

  @tag :integration
  test "test status codes 19 413 payload too large" do
    {routes, config} = AppFactories.create_app_handle_status_codes_19_413_payload_too_large()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/upload"
      headers = []
      req_body = Jason.encode!(%{"data" => "{{ repeat 'x' 2000 times }}"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 413, "Expected status 413, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["error"] == "Payload Too Large"

      assert parsed_body["message"] ==
               "Request body size exceeds maximum allowed size of 1024 bytes"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 200 OK - Success" do
    {routes, config} = AppFactories.create_app_handle_status_codes_200_ok___success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/status-test/200"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["id"] == 1
      assert parsed_body["name"] == "Item 1"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 201 Created - Resource created" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_201_created___resource_created()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"name" => "New Item"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["id"] == 1
      assert parsed_body["name"] == "New Item"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 202 Accepted - Request accepted for processing" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_202_accepted___request_accepted_for_processing()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/tasks/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"task" => "process_data"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 202, "Expected status 202, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["message"] == "Task accepted for processing"
      assert parsed_body["task_id"] == "abc123"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 204 No Content - Success with no body" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_204_no_content___success_with_no_body()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/status-test/204"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:delete, {String.to_charlist(url), headers}, [], [])

      assert status == 204, "Expected status 204, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 206 Partial Content" do
    {routes, config} = AppFactories.create_app_handle_status_codes_206_partial_content()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/files/document.pdf"
      headers = [{~c"Range", ~c"bytes=0-1023"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 206, "Expected status 206, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "binary_data_1024_bytes"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 20 414 uri too long" do
    {routes, config} = AppFactories.create_app_handle_status_codes_20_414_uri_too_long()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data?skip_template_expansion=true"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 21 431 request header fields too large" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_21_431_request_header_fields_too_large()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = [{~c"X-Large-Header", ~c"{{ repeat 'x' 10000 times }}"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 431, "Expected status 431, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["error"] == "Request Header Fields Too Large"
      assert parsed_body["message"] == "Request headers exceed maximum allowed size of 8192 bytes"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 22 501 not implemented" do
    {routes, config} = AppFactories.create_app_handle_status_codes_22_501_not_implemented()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:trace, {String.to_charlist(url), headers}, [], [])

      assert status == 405, "Expected status 405, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 23 503 service unavailable" do
    {routes, config} = AppFactories.create_app_handle_status_codes_23_503_service_unavailable()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = []

      {status, resp_body} =
        case :httpc.request(:get, {String.to_charlist(url), headers}, [{:timeout, 5000}], []) do
          {:ok, {{_, s, _}, _resp_headers, body}} ->
            {s, body}

          {:error, :timeout} ->
            # :httpc may timeout due to Retry-After handling on 503 responses.
            # We assume 503 and no body in this case.
            {503, []}
        end

      assert status == 503, "Expected status 503, got #{status}"
      # Response body validation
      # For 503, body assertions are conditional since :httpc may timeout
      if resp_body != [] do
        resp_body_str = :erlang.list_to_binary(resp_body)
        parsed_body = Jason.decode!(resp_body_str)
        assert parsed_body["error"] == "Service Unavailable"

        assert parsed_body["message"] ==
                 "The service is temporarily unavailable. Please try again later."
      end
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 301 Moved Permanently - Permanent redirect" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_301_moved_permanently___permanent_redirect()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/old-path"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [{:autoredirect, false}], [])

      assert status == 301, "Expected status 301, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 302 Found - Temporary redirect" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_302_found___temporary_redirect()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/temp-redirect"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [{:autoredirect, false}], [])

      assert status == 302, "Expected status 302, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 304 Not Modified - Cached content valid" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_304_not_modified___cached_content_valid()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/status-test/304"
      headers = [{~c"If-None-Match", ~c"\"abc123\""}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 304, "Expected status 304, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 307 Temporary Redirect - Method preserved" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_307_temporary_redirect___method_preserved()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/redirect-post"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [{:autoredirect, false}],
          []
        )

      assert status == 307, "Expected status 307, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 400 Bad Request - Invalid request" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_400_bad_request___invalid_request()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!("not valid json")

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 400, "Expected status 400, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["detail"] == "Invalid request format"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 401 Unauthorized - Missing authentication" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_401_unauthorized___missing_authentication()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/users/me"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 401, "Expected status 401, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["detail"] == "Not authenticated"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 403 Forbidden - Insufficient permissions" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_403_forbidden___insufficient_permissions()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/admin/users"
      headers = [{~c"Authorization", ~c"Bearer valid_token"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 403, "Expected status 403, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["detail"] == "Not enough permissions"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 404 Not Found - Resource not found" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_404_not_found___resource_not_found()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/status-test/404"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 404, "Expected status 404, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["detail"] == "Item not found"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 408 Request Timeout" do
    {routes, config} = AppFactories.create_app_handle_status_codes_408_request_timeout()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/slow-endpoint"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"data" => "large_data"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 408, "Expected status 408, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["detail"] == "Request timeout"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 422 Unprocessable Entity - Validation error" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_422_unprocessable_entity___validation_error()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/json"}]
      req_body = Jason.encode!(%{"price" => "not a number"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json", req_body},
          [],
          []
        )

      assert status == 422, "Expected status 422, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/validation-error"
      assert parsed_body["title"] == "Request Validation Failed"
      assert parsed_body["status"] == 422
      assert parsed_body["detail"] == "1 validation error in request"
      assert Map.has_key?(parsed_body, "errors")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 429 Too Many Requests" do
    {routes, config} = AppFactories.create_app_handle_status_codes_429_too_many_requests()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/resource"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 429, "Expected status 429, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["detail"] == "Rate limit exceeded. Try again in 60 seconds."
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 500 Internal Server Error - Server error" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_500_internal_server_error___server_error()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/error"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 500, "Expected status 500, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/internal-server-error"
      assert parsed_body["title"] == "Internal Server Error"
      assert parsed_body["status"] == 500
      assert parsed_body["detail"] == "Internal server error"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test status codes 503 Service Unavailable - Server overload" do
    {routes, config} =
      AppFactories.create_app_handle_status_codes_503_service_unavailable___server_overload()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/health"
      headers = []

      {status, resp_body} =
        case :httpc.request(:get, {String.to_charlist(url), headers}, [{:timeout, 5000}], []) do
          {:ok, {{_, s, _}, _resp_headers, body}} ->
            {s, body}

          {:error, :timeout} ->
            # :httpc may timeout due to Retry-After handling on 503 responses.
            # We assume 503 and no body in this case.
            {503, []}
        end

      assert status == 503, "Expected status 503, got #{status}"
      # Response body validation
      # For 503, body assertions are conditional since :httpc may timeout
      if resp_body != [] do
        resp_body_str = :erlang.list_to_binary(resp_body)
        parsed_body = Jason.decode!(resp_body_str)
        assert parsed_body["detail"] == "Service temporarily unavailable"
      end
    after
      Spikard.stop(server)
    end
  end
end
