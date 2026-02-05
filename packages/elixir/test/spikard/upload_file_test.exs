defmodule Spikard.UploadFileTest do
  @moduledoc """
  Tests for Spikard.UploadFile struct.

  Tests cover file metadata parsing, file access from requests,
  and basic file upload scenarios.

  Note: The "UploadFile in request" tests require Rust-side multipart
  parsing implementation to work.
  """
  use ExUnit.Case, async: true

  alias Spikard.UploadFile

  describe "UploadFile.new/4" do
    test "creates a file with all fields" do
      file = UploadFile.new("test.txt", "text/plain", 42, "file content")
      assert file.filename == "test.txt"
      assert file.content_type == "text/plain"
      assert file.size == 42
      assert file.data == "file content"
    end

    test "handles different MIME types" do
      file = UploadFile.new("image.jpg", "image/jpeg", 1024, <<0xFF, 0xD8>>)
      assert file.content_type == "image/jpeg"
      assert file.size == 1024
    end

    test "handles empty files" do
      file = UploadFile.new("empty.txt", "text/plain", 0, "")
      assert file.size == 0
      assert file.data == ""
    end

    test "handles binary file data" do
      binary_data = <<137, 80, 78, 71>>  # PNG magic bytes
      file = UploadFile.new("image.png", "image/png", 4, binary_data)
      assert file.data == binary_data
      assert file.size == 4
    end
  end

  describe "UploadFile struct fields" do
    test "struct has all required fields" do
      file = UploadFile.new("test.pdf", "application/pdf", 256, "data")
      assert Map.has_key?(file, :filename)
      assert Map.has_key?(file, :content_type)
      assert Map.has_key?(file, :size)
      assert Map.has_key?(file, :data)
    end

    test "filename is string" do
      file = UploadFile.new("document.docx", "application/vnd.openxmlformats-officedocument.wordprocessingml.document", 512, "")
      assert is_binary(file.filename)
    end

    test "content_type is string" do
      file = UploadFile.new("file.json", "application/json", 100, "{}")
      assert is_binary(file.content_type)
    end

    test "size is integer" do
      file = UploadFile.new("large.bin", "application/octet-stream", 10_000_000, "")
      assert is_integer(file.size)
    end

    test "data is binary" do
      file = UploadFile.new("text.txt", "text/plain", 11, "hello world")
      assert is_binary(file.data) or is_list(file.data)
    end
  end

  describe "UploadFile in request" do
    test "request.files returns list of files" do
      handler = fn req ->
        files = Spikard.Request.files(req)
        %{status: 200, body: %{file_count: length(files)}}
      end

      {:ok, client} = Spikard.TestClient.new(routes: [{:post, "/upload", handler}])
      {:ok, response} = Spikard.TestClient.post(client, "/upload", multipart: [{"file", "test data"}])

      assert response.status_code == 200
    end

    test "files are accessible with metadata" do
      handler = fn req ->
        files = Spikard.Request.files(req)

        case files do
          [file] ->
            %{
              status: 200,
              body: %{
                filename: file.filename,
                content_type: file.content_type,
                size: file.size
              }
            }

          _ ->
            %{status: 400, body: %{error: "Expected 1 file"}}
        end
      end

      {:ok, client} = Spikard.TestClient.new(routes: [{:post, "/upload", handler}])
      {:ok, response} = Spikard.TestClient.post(client, "/upload", multipart: [{"file", "content", filename: "test.txt"}])

      assert response.status_code == 200
      body = Spikard.TestClient.Response.json(response)
      assert body["filename"] == "test.txt"
      assert is_integer(body["size"])
    end

    test "multiple files are returned as list" do
      handler = fn req ->
        files = Spikard.Request.files(req)
        filenames = Enum.map(files, & &1.filename)

        %{status: 200, body: %{filenames: filenames, count: length(files)}}
      end

      {:ok, client} = Spikard.TestClient.new(routes: [{:post, "/upload", handler}])

      multipart = [
        {"files", "content 1", filename: "file1.txt"},
        {"files", "content 2", filename: "file2.txt"}
      ]

      {:ok, response} = Spikard.TestClient.post(client, "/upload", multipart: multipart)

      assert response.status_code == 200
      body = Spikard.TestClient.Response.json(response)
      assert body["count"] == 2
      assert Enum.member?(body["filenames"], "file1.txt")
      assert Enum.member?(body["filenames"], "file2.txt")
    end

    test "file data is binary" do
      handler = fn req ->
        files = Spikard.Request.files(req)

        case files do
          [file] ->
            %{
              status: 200,
              body: %{
                is_binary: is_binary(file.data),
                data_length: byte_size(file.data)
              }
            }

          _ ->
            %{status: 400}
        end
      end

      {:ok, client} = Spikard.TestClient.new(routes: [{:post, "/upload", handler}])
      {:ok, response} = Spikard.TestClient.post(client, "/upload", multipart: [{"file", "hello", filename: "test.txt"}])

      assert response.status_code == 200
      body = Spikard.TestClient.Response.json(response)
      assert body["is_binary"] == true
      assert body["data_length"] == 5
    end

    test "file with no filename still works" do
      handler = fn req ->
        files = Spikard.Request.files(req)

        case files do
          [file] ->
            %{status: 200, body: %{filename: file.filename}}

          _ ->
            %{status: 400}
        end
      end

      {:ok, client} = Spikard.TestClient.new(routes: [{:post, "/upload", handler}])
      {:ok, response} = Spikard.TestClient.post(client, "/upload", multipart: [{"field", "data"}])

      assert response.status_code == 200
    end

    test "empty file list when no files uploaded" do
      handler = fn req ->
        files = Spikard.Request.files(req)
        %{status: 200, body: %{count: length(files)}}
      end

      {:ok, client} = Spikard.TestClient.new(routes: [{:post, "/upload", handler}])
      {:ok, response} = Spikard.TestClient.post(client, "/upload", json: %{data: "no files"})

      assert response.status_code == 200
      body = Spikard.TestClient.Response.json(response)
      assert body["count"] == 0
    end
  end
end
