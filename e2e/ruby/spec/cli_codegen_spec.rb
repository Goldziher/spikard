require 'spec_helper'
require "json"
require "open3"
require "pathname"
require "tempfile"

ROOT = Pathname.new(__dir__).join("..", "..", "..").expand_path
CLI_MANIFEST = ROOT.join("crates", "spikard-cli", "Cargo.toml")

OPENAPI_SPEC = <<~YAML
  openapi: 3.1.0
  info:
    title: Ruby DTO Smoke
    version: "1.0.0"
  paths:
    /hello:
      get:
        operationId: sayHello
        responses:
          "200":
            description: OK
            content:
              application/json:
                schema:
                  $ref: "#/components/schemas/HelloResponse"
  components:
    schemas:
      HelloResponse:
        type: object
        properties:
          message:
            type: string
        required:
          - message
YAML

ASYNCAPI_SPEC = <<~YAML
  asyncapi: "3.0.0"
  info:
    title: Ruby AsyncAPI Smoke
    version: "1.0.0"
  servers:
    ws:
      host: chat.example.com
      protocol: ws
  channels:
    /chat:
      messages:
        chatMessage:
          payload:
            type: object
            properties:
              type:
                const: chatMessage
              body:
                type: string
            required:
              - type
              - body
YAML

RSpec.describe "spikard-cli DTO generation (ruby)" do
  def run_cli(*args)
    cmd = ["cargo", "run", "--manifest-path", CLI_MANIFEST.to_s, "--", *args]
    stdout, stderr, status = Open3.capture3(*cmd, chdir: ROOT.to_s)
    raise "CLI failed: #{stderr}" unless status.success?
    stdout
  end

  it "generates Dry::Struct models for OpenAPI specs" do
    Dir.mktmpdir do |dir|
      spec_path = File.join(dir, "openapi.yaml")
      output_path = File.join(dir, "app.rb")
      File.write(spec_path, OPENAPI_SPEC)

      run_cli("generate", spec_path, "--lang", "ruby", "--dto", "dry-schema", "--output", output_path)

      contents = File.read(output_path)
      expect(contents).to include("class HelloResponse < Dry::Struct")
      syntax_check = system("ruby", "-c", output_path)
      expect(syntax_check).to be true
    end
  end

  it "generates AsyncAPI WebSocket handler" do
    Dir.mktmpdir do |dir|
      spec_path = File.join(dir, "asyncapi.yaml")
      output_path = File.join(dir, "ws_app.rb")
      File.write(spec_path, ASYNCAPI_SPEC)

      run_cli(
        "generate-asyncapi",
        spec_path,
        "test-app",
        "--lang",
        "ruby",
        "--dto",
        "dry-schema",
        "--output",
        output_path
      )

      contents = File.read(output_path)
      expect(contents).to include("def handle_websocket")
      expect(system("ruby", "-c", output_path)).to be true
    end
  end

  it "generates AsyncAPI handler scaffolding" do
    Dir.mktmpdir do |dir|
      spec_path = File.join(dir, "asyncapi.yaml")
      output_path = File.join(dir, "handlers.rb")
      File.write(spec_path, ASYNCAPI_SPEC)

      run_cli(
        "generate-asyncapi",
        spec_path,
        "handlers",
        "--lang",
        "ruby",
        "--dto",
        "dry-schema",
        "--output",
        output_path
      )

      expect(File.read(output_path)).to include("app.websocket")
      expect(system("ruby", "-c", output_path)).to be true
    end
  end
end
