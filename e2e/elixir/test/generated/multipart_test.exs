defmodule E2EElixirApp.MultipartTest do
  @moduledoc """
  Generated tests for multipart fixtures.

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
  test "test multipart 17 file magic number png success" do
    {routes, config} = AppFactories.create_app_handle_multipart_17_file_magic_number_png_success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/upload"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"image\"; filename=\"test.png\"\r\nContent-Type: image/png\r\n\r\n\r\n"
               ]

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

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart 18 file magic number jpeg success" do
    {routes, config} =
      AppFactories.create_app_handle_multipart_18_file_magic_number_jpeg_success()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/upload"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"image\"; filename=\"test.jpg\"\r\nContent-Type: image/jpeg\r\n\r\n\r\n"
               ]

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

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart 19 file mime spoofing png as jpeg" do
    {routes, config} =
      AppFactories.create_app_handle_multipart_19_file_mime_spoofing_png_as_jpeg()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/upload"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"image\"; filename=\"fake.jpg\"\r\nContent-Type: image/jpeg\r\n\r\n\r\n"
               ]

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
  test "test multipart 20 file mime spoofing jpeg as png" do
    {routes, config} =
      AppFactories.create_app_handle_multipart_20_file_mime_spoofing_jpeg_as_png()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/upload"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"image\"; filename=\"fake.png\"\r\nContent-Type: image/png\r\n\r\n\r\n"
               ]

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
  test "test multipart 21 file pdf magic number success" do
    {routes, config} = AppFactories.create_app_handle_multipart_21_file_pdf_magic_number_success()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/upload"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"document\"; filename=\"test.pdf\"\r\nContent-Type: application/pdf\r\n\r\n\r\n"
               ]

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

      assert status == 201, "Expected status 201, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart 22 file empty buffer" do
    {routes, config} = AppFactories.create_app_handle_multipart_22_file_empty_buffer()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/upload"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"empty.txt\"\r\nContent-Type: text/plain\r\n\r\n\r\n"
               ]

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
  test "test multipart Content-Type validation - invalid type" do
    {routes, config} =
      AppFactories.create_app_handle_multipart_content_type_validation___invalid_type()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/files/images-only"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"script.sh\"\r\nContent-Type: application/x-sh\r\n\r\n#!/bin/bash\necho hello\r\n"
               ]

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

      assert status == 422, "Expected status 422, got #{status}"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart Empty file upload" do
    {routes, config} = AppFactories.create_app_handle_multipart_empty_file_upload()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/files/upload"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"empty.txt\"\r\nContent-Type: text/plain\r\n\r\n\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["filename"] == "empty.txt"
      assert parsed_body["size"] == 0
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart File list upload (array of files)" do
    {routes, config} =
      AppFactories.create_app_handle_multipart_file_list_upload__array_of_files_()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/files/list"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"file1.txt\"\r\nContent-Type: text/plain\r\n\r\ncontent of file 1\r\n"
               ]

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"file2.txt\"\r\nContent-Type: text/plain\r\n\r\ncontent of file 2\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "filenames")
      assert parsed_body["total_size"] == 35
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart File size validation - too large" do
    {routes, config} = AppFactories.create_app_handle_multipart_file_size_validation___too_large()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/files/validated"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"large.txt\"\r\nContent-Type: text/plain\r\n\r\nx\r\n"
               ]

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

      assert status == 413, "Expected status 413, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["detail"] == "File too large. Maximum size is 1MB"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart File upload with custom headers" do
    {routes, config} = AppFactories.create_app_handle_multipart_file_upload_with_custom_headers()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"test2\"; filename=\"test2.txt\"\r\nContent-Type: text/plain\r\n\r\n<file2 content>\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "test2")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart File upload without filename" do
    {routes, config} = AppFactories.create_app_handle_multipart_file_upload_without_filename()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"test1\"\r\nContent-Type: application/octet-stream\r\n\r\n<file1 content>\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["test1"] == "<file1 content>"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart Form data without files" do
    {routes, config} = AppFactories.create_app_handle_multipart_form_data_without_files()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               ["--#{boundary}\r\nContent-Disposition: form-data; name=\"some\"\r\n\r\ndata\r\n"]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["some"] == "data"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart Image file upload" do
    {routes, config} = AppFactories.create_app_handle_multipart_image_file_upload()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/files/image"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"image\"; filename=\"photo.jpg\"\r\nContent-Type: image/jpeg\r\n\r\nfake_jpeg_content_here\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["filename"] == "photo.jpg"
      assert parsed_body["content_type"] == "image/jpeg"
      assert parsed_body["size"] == 22
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart Mixed files and form data" do
    {routes, config} = AppFactories.create_app_handle_multipart_mixed_files_and_form_data()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               ["--#{boundary}\r\nContent-Disposition: form-data; name=\"age\"\r\n\r\n25\r\n"]

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"active\"\r\n\r\ntrue\r\n"
               ]

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"username\"\r\n\r\ntestuser\r\n"
               ]

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"upload.txt\"\r\nContent-Type: text/plain\r\n\r\nfile data here\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "file")
      assert parsed_body["username"] == "testuser"
      assert parsed_body["age"] == "25"
      assert parsed_body["active"] == "true"
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart Multiple file uploads" do
    {routes, config} = AppFactories.create_app_handle_multipart_multiple_file_uploads()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"test1\"; filename=\"test1.txt\"\r\nContent-Type: text/plain\r\n\r\n<file1 content>\r\n"
               ]

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"test2\"; filename=\"test2.txt\"\r\nContent-Type: text/plain\r\n\r\n<file2 content>\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "test1")
      assert Map.has_key?(parsed_body, "test2")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart Multiple values for same field name" do
    {routes, config} =
      AppFactories.create_app_handle_multipart_multiple_values_for_same_field_name()

    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"tags\"\r\n\r\n[\"python\",\"rust\",\"web\"]\r\n"
               ]

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"file1.txt\"\r\nContent-Type: text/plain\r\n\r\nfirst file\r\n"
               ]

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"files\"; filename=\"file2.txt\"\r\nContent-Type: text/plain\r\n\r\nsecond file\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "files")
      assert Map.has_key?(parsed_body, "tags")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart Optional file upload - missing" do
    {routes, config} = AppFactories.create_app_handle_multipart_optional_file_upload___missing()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/files/optional"

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "file")
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart Optional file upload - provided" do
    {routes, config} = AppFactories.create_app_handle_multipart_optional_file_upload___provided()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/files/optional"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"optional.txt\"\r\nContent-Type: text/plain\r\n\r\noptional file content\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["filename"] == "optional.txt"
      assert parsed_body["content_type"] == "text/plain"
      assert parsed_body["size"] == 21
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart PDF file upload" do
    {routes, config} = AppFactories.create_app_handle_multipart_pdf_file_upload()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/files/document"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"document\"; filename=\"report.pdf\"\r\nContent-Type: application/pdf\r\n\r\nfake_pdf_content\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert parsed_body["filename"] == "report.pdf"
      assert parsed_body["content_type"] == "application/pdf"
      assert parsed_body["size"] == 16
    after
      Spikard.stop(server)
    end
  end

  @tag :integration
  test "test multipart Required file upload - missing" do
    {routes, config} = AppFactories.create_app_handle_multipart_required_file_upload___missing()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/files/required"

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
  test "test multipart Simple file upload" do
    {routes, config} = AppFactories.create_app_handle_multipart_simple_file_upload()
    start_opts = [port: 59800, host: "127.0.0.1", routes: routes]
    start_opts = if config == %{}, do: start_opts, else: start_opts ++ [config: config]
    {:ok, server} = Spikard.start(start_opts)

    try do
      url = @base_url <> "/"

      {boundary, req_body} =
        (fn ->
           boundary = "----ElixirFormBoundary#{:erlang.unique_integer([:positive])}"
           parts = []

           parts =
             parts ++
               [
                 "--#{boundary}\r\nContent-Disposition: form-data; name=\"test\"; filename=\"test.txt\"\r\nContent-Type: text/plain\r\n\r\n<file content>\r\n"
               ]

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

      assert status == 200, "Expected status 200, got #{status}"
      # Response body validation
      resp_body_str = :erlang.list_to_binary(resp_body)
      parsed_body = Jason.decode!(resp_body_str)
      assert Map.has_key?(parsed_body, "test")
    after
      Spikard.stop(server)
    end
  end
end
