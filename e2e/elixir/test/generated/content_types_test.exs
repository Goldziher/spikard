defmodule E2EElixirApp.ContentTypesTest do
  @moduledoc """
  Generated tests for content_types fixtures.

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
  test "test content types 13 json with charset utf16" do
    {routes, config} = AppFactories.create_app_handle_content_types_13_json_with_charset_utf16()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = [{~c"Content-Type", ~c"application/json; charset=utf-16"}]
      req_body = Jason.encode!(%{"value" => "test"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/json; charset=utf-16", req_body},
          [],
          []
        )

      assert status == 415, "Expected status 415, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unsupported-charset"
      assert parsed_body["title"] == "Unsupported Charset"
      assert parsed_body["status"] == 415

      assert parsed_body["detail"] ==
               "Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported."
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types 14 content type case insensitive" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_14_content_type_case_insensitive()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = [{~c"Content-Type", ~c"APPLICATION/JSON"}]
      req_body = Jason.encode!(%{"name" => "test"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"APPLICATION/JSON", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["name"] == "test"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types 15 multipart boundary required" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_15_multipart_boundary_required()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/upload"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []
           body = Enum.join(parts, "") <> "--#{boundary}--\r\n"
           {boundary, body}
         end).()

      headers = [{~c"Content-Type", ~c"multipart/form-data; boundary=#{boundary}"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"multipart/form-data; boundary=#{boundary}",
           req_body},
          [],
          []
        )

      assert status == 400, "Expected status 400, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["error"] == "multipart/form-data requires 'boundary' parameter"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types 16 text plain not accepted" do
    {routes, config} = AppFactories.create_app_handle_content_types_16_text_plain_not_accepted()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = [{~c"Content-Type", ~c"text/plain"}]
      req_body = Jason.encode!("{\"data\": \"value\"}")

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"text/plain", req_body},
          [],
          []
        )

      assert status == 415, "Expected status 415, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unsupported-media-type"
      assert parsed_body["title"] == "Unsupported Media Type"
      assert parsed_body["status"] == 415
      assert parsed_body["detail"] == "Unsupported media type"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types 17 vendor json accepted" do
    {routes, config} = AppFactories.create_app_handle_content_types_17_vendor_json_accepted()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/api/v1/resource"
      headers = [{~c"Content-Type", ~c"application/vnd.api+json"}]
      req_body = Jason.encode!(%{"data" => "value"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/vnd.api+json", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["data"] == "value"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types 18 content type with multiple params" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_18_content_type_with_multiple_params()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = [{~c"Content-Type", ~c"application/json; charset=utf-8; boundary=something"}]
      req_body = Jason.encode!(%{"value" => "test"})

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers,
           ~c"application/json; charset=utf-8; boundary=something", req_body},
          [],
          []
        )

      assert status == 201, "Expected status 201, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["value"] == "test"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types 19 missing content type default json" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_19_missing_content_type_default_json()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = []
      req_body = Jason.encode!(%{"name" => "test"})

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
      assert parsed_body["name"] == "test"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types 20 content length mismatch" do
    {routes, config} = AppFactories.create_app_handle_content_types_20_content_length_mismatch()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/data"
      headers = [{~c"Content-Type", ~c"application/json"}, {~c"Content-Length", ~c"100"}]
      req_body = Jason.encode!(%{"value" => "short"})

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
      assert parsed_body["type"] == "https://spikard.dev/errors/content-length-mismatch"
      assert parsed_body["title"] == "Content-Length header mismatch"
      assert parsed_body["status"] == 400
      assert parsed_body["detail"] == "Content-Length header does not match actual body size"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types 415 Unsupported Media Type" do
    {routes, config} = AppFactories.create_app_handle_content_types_415_unsupported_media_type()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/"
      headers = [{~c"Content-Type", ~c"application/xml"}]
      req_body = Jason.encode!("<?xml version=\"1.0\"?><item><name>Item</name></item>")

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(
          :post,
          {String.to_charlist(url), headers, ~c"application/xml", req_body},
          [],
          []
        )

      assert status == 415, "Expected status 415, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["type"] == "https://spikard.dev/errors/unsupported-media-type"
      assert parsed_body["title"] == "Unsupported Media Type"
      assert parsed_body["status"] == 415
      assert parsed_body["detail"] == "Unsupported media type"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types Binary response - application/octet-stream" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_binary_response___application_octet_stream()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/download/file.bin"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "binary_data_placeholder"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types CSV response - text/csv" do
    {routes, config} = AppFactories.create_app_handle_content_types_csv_response___text_csv()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/export/data.csv"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "id,name,price\n1,Item A,10.0\n2,Item B,20.0"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types Content negotiation - Accept header" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_content_negotiation___accept_header()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/accept-test/1"
      headers = [{~c"Accept", ~c"application/json"}]

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["id"] == 1
      assert parsed_body["name"] == "Item"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types HTML response - text/html" do
    {routes, config} = AppFactories.create_app_handle_content_types_html_response___text_html()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/html"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "<html><body><h1>Hello</h1></body></html>"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types JPEG image response - image/jpeg" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_jpeg_image_response___image_jpeg()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/images/photo.jpg"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "jpeg_binary_data"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types JSON response - application/json" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_json_response___application_json()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/json"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["name"] == "Item"
      assert parsed_body["price"] == 42.0
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types JSON with UTF-8 charset" do
    {routes, config} = AppFactories.create_app_handle_content_types_json_with_utf_8_charset()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/items/unicode"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["name"] == "Café"
      assert parsed_body["emoji"] == "☕"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types PDF response - application/pdf" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_pdf_response___application_pdf()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/download/document.pdf"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "pdf_binary_data"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types PNG image response - image/png" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_png_image_response___image_png()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/images/logo.png"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "png_binary_data"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types Plain text response - text/plain" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_plain_text_response___text_plain()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/text"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      assert resp_body_str == "Hello, World!"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test content types XML response - application/xml" do
    {routes, config} =
      AppFactories.create_app_handle_content_types_xml_response___application_xml()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/xml"
      headers = []

      {:ok, {{_, status, _}, _resp_headers, resp_body}} =
        :httpc.request(:get, {String.to_charlist(url), headers}, [], [])

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)

      assert resp_body_str ==
               "<?xml version=\"1.0\"?><item><name>Item</name><price>42.0</price></item>"
    after
      Spikard.stop(server)
    end
  end
end
