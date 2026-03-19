defmodule Spikard.GrpcTest do
  use ExUnit.Case, async: false

  alias Spikard.Grpc

  describe "service registry" do
    test "registers method definitions for Rust gRPC routing" do
      registry =
        Grpc.Service.new()
        |> Grpc.Service.register("example.EchoService", "Echo", :unary, fn request ->
          %Grpc.Response{payload: request.payload}
        end)
        |> Grpc.Service.register("example.EchoService", "EchoStream", :server_stream, fn requests ->
          requests
        end)

      assert Grpc.Service.registered?(registry, "example.EchoService", "Echo")
      assert Enum.sort(Grpc.Service.list_methods(registry, "example.EchoService")) == ["Echo", "EchoStream"]

      assert Grpc.Service.service_definitions(registry) == %{
               "example.EchoService" => %{
                 "Echo" => "unary",
                 "EchoStream" => "server_stream"
               }
             }
    end
  end

  describe "runtime integration" do
    test "serves unary and streaming gRPC methods over the Rust transport" do
      port = free_port()

      grpc =
        Grpc.Service.new()
        |> Grpc.Service.register("example.EchoService", "Echo", :unary, fn request ->
          %Grpc.Response{payload: request.payload, metadata: %{"x-test" => "unary"}}
        end)
        |> Grpc.Service.register("example.EchoService", "EchoStream", :server_stream, fn request ->
          [
            %Grpc.Response{payload: request.payload},
            %Grpc.Response{payload: request.payload <> "-again"}
          ]
        end)
        |> Grpc.Service.register("example.EchoService", "Collect", :client_stream, fn requests ->
          payload =
            requests
            |> Enum.map(& &1.payload)
            |> Enum.join("|")

          %Grpc.Response{payload: payload}
        end)
        |> Grpc.Service.register("example.EchoService", "Chat", :bidi_stream, fn requests ->
          Enum.map(requests, fn request ->
            %Grpc.Response{payload: "echo:" <> request.payload}
          end)
        end)

      assert {:ok, server} = Spikard.start(port: port, grpc: grpc)
      on_exit(fn -> Spikard.stop(server) end)

      assert {:ok, unary} = grpc_curl_request(port, "/example.EchoService/Echo", ["hello"])
      assert decode_grpc_messages(unary.body) == ["hello"]
      assert header_value(unary.headers, "grpc-status") == "0"

      assert {:ok, server_stream} = grpc_curl_request(port, "/example.EchoService/EchoStream", ["hello"])
      assert decode_grpc_messages(server_stream.body) == ["hello", "hello-again"]

      assert {:ok, client_stream} = grpc_curl_request(port, "/example.EchoService/Collect", ["one", "two", "three"])
      assert decode_grpc_messages(client_stream.body) == ["one|two|three"]
      assert header_value(client_stream.headers, "grpc-status") == "0"

      assert {:ok, bidi} = grpc_curl_request(port, "/example.EchoService/Chat", ["left", "right"])
      assert decode_grpc_messages(bidi.body) == ["echo:left", "echo:right"]
    end

    test "builds structured Elixir gRPC error tuples" do
      assert {:error, %Grpc.Error{} = error} = Grpc.Response.error("bad request", :invalid_argument)
      assert error.code == :invalid_argument
      assert error.message == "bad request"
    end
  end

  defp grpc_curl_request(port, path, payloads) do
    curl = System.find_executable("curl")

    if is_nil(curl) do
      {:error, "curl executable not found"}
    else
      tmp_dir = System.tmp_dir!()
      unique = Integer.to_string(System.unique_integer([:positive]))
      request_path = Path.join(tmp_dir, "spikard-grpc-request-#{unique}.bin")
      response_path = Path.join(tmp_dir, "spikard-grpc-response-#{unique}.bin")
      headers_path = Path.join(tmp_dir, "spikard-grpc-headers-#{unique}.txt")

      request_body =
        payloads
        |> Enum.map(&encode_grpc_message/1)
        |> IO.iodata_to_binary()

      File.write!(request_path, request_body)

      args = [
        "--silent",
        "--show-error",
        "--http2-prior-knowledge",
        "--header",
        "content-type: application/grpc",
        "--header",
        "te: trailers",
        "--data-binary",
        "@#{request_path}",
        "--dump-header",
        headers_path,
        "--output",
        response_path,
        "http://127.0.0.1:#{port}#{path}"
      ]

      result =
        case System.cmd(curl, args, stderr_to_stdout: true) do
          {_output, 0} ->
            {:ok,
             %{
               headers: File.read!(headers_path),
               body: File.read!(response_path)
             }}

          {output, status} ->
            {:error, "curl gRPC probe failed with status #{status}: #{output}"}
        end

      File.rm(request_path)
      File.rm(response_path)
      File.rm(headers_path)
      result
    end
  end

  defp encode_grpc_message(payload) when is_binary(payload) do
    <<0, byte_size(payload)::32-big, payload::binary>>
  end

  defp decode_grpc_messages(binary), do: decode_grpc_messages(binary, [])

  defp decode_grpc_messages(<<>>, acc), do: Enum.reverse(acc)

  defp decode_grpc_messages(<<0, size::32-big, payload::binary-size(size), rest::binary>>, acc) do
    decode_grpc_messages(rest, [payload | acc])
  end

  defp decode_grpc_messages(other, _acc) do
    flunk("invalid gRPC body framing: #{inspect(other)}")
  end

  defp header_value(headers, name) do
    name_downcase = String.downcase(name)

    headers
    |> String.split("\r\n")
    |> Enum.reverse()
    |> Enum.find_value(fn line ->
      case String.split(line, ":", parts: 2) do
        [key, value] ->
          if String.downcase(key) == name_downcase do
            String.trim(value)
          end

        _ ->
          nil
      end
    end)
  end

  defp free_port do
    {:ok, socket} = :gen_tcp.listen(0, [:binary, active: false])
    {:ok, {_addr, port}} = :inet.sockname(socket)
    :gen_tcp.close(socket)
    port
  end
end
