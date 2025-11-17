use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Result;
use spikard_cli::codegen::{
    CodegenEngine, CodegenOutcome, CodegenRequest, CodegenTargetKind, DtoConfig, NodeDtoStyle, PythonDtoStyle,
    RubyDtoStyle, SchemaKind, TargetLanguage, generate_from_openapi,
};
use tempfile::tempdir;

const SIMPLE_OPENAPI: &str = r##"
openapi: 3.1.0
info:
  title: Example API
  version: "1.0.0"
paths:
  /hello:
    get:
      operationId: helloWorld
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
      description: Example greeting payload
      properties:
        message:
          type: string
        count:
          type: integer
          nullable: true
      required:
        - message
"##;

const SIMPLE_ASYNCAPI: &str = r##"
asyncapi: "3.0.0"
info:
  title: Chat API
  version: "1.0.0"
servers:
  primary:
    host: ws.example.com
    protocol: ws
channels:
  /chat:
    messages:
      chatEvent:
        payload:
          type: object
          properties:
            type:
              const: chatEvent
            body:
              type: string
          required:
            - type
            - body
"##;

fn write_temp_file(dir: &Path, name: &str, contents: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, contents).expect("failed to write test fixture");
    path
}

#[test]
fn python_dataclass_generation_emits_dataclasses() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let mut dto = DtoConfig::default();
    dto.python = PythonDtoStyle::Dataclass;

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto, None)?;
    assert!(code.contains("@dataclass"), "expected dataclass annotation");
    assert!(code.contains("class HelloResponse"), "expected response class");
    assert!(code.contains("slots=True"), "dataclasses should enable slots");
    assert_python_class_executes("HelloResponse", &code)?;
    Ok(())
}

#[test]
fn python_msgspec_generation_emits_structs() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let mut dto = DtoConfig::default();
    dto.python = PythonDtoStyle::Msgspec;

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto, None)?;
    assert!(
        code.contains("class HelloResponse(msgspec.Struct)"),
        "msgspec struct not generated"
    );
    assert!(code.contains("import msgspec"), "msgspec import missing");
    assert_python_class_executes("HelloResponse", &code)?;
    Ok(())
}

#[test]
fn python_nullable_properties_emit_optional_union() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let mut dto = DtoConfig::default();
    dto.python = PythonDtoStyle::Dataclass;

    let code = generate_from_openapi(&schema_path, TargetLanguage::Python, &dto, None)?;
    assert!(
        code.contains("count: int | None = None"),
        "expected nullable optional dataclass field"
    );
    Ok(())
}

#[test]
fn node_generation_uses_zod_schemas() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let mut dto = DtoConfig::default();
    dto.node = NodeDtoStyle::Zod;

    let code = generate_from_openapi(&schema_path, TargetLanguage::TypeScript, &dto, None)?;
    assert!(
        code.contains("import { z } from \"zod\""),
        "expected Zod import in generated code"
    );
    assert!(
        code.contains("export const HelloResponseSchema = z.object"),
        "expected inferred schema"
    );
    Ok(())
}

#[test]
fn typescript_nullable_properties_emit_nullable_optional_schemas() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let mut dto = DtoConfig::default();
    dto.node = NodeDtoStyle::Zod;

    let code = generate_from_openapi(&schema_path, TargetLanguage::TypeScript, &dto, None)?;
    assert!(
        code.contains("\tcount: z.number().int().nullable().optional(),"),
        "expected nullable + optional zod chain"
    );
    Ok(())
}

#[test]
fn ruby_generation_uses_dry_structs() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "openapi.yaml", SIMPLE_OPENAPI);

    let mut dto = DtoConfig::default();
    dto.ruby = RubyDtoStyle::DrySchema;

    let code = generate_from_openapi(&schema_path, TargetLanguage::Ruby, &dto, None)?;
    assert!(
        code.contains("class HelloResponse < Dry::Struct"),
        "expected Dry::Struct model"
    );
    Ok(())
}

#[test]
fn asyncapi_fixture_generation_creates_files() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "asyncapi.yaml", SIMPLE_ASYNCAPI);
    let fixtures_dir = dir.path().join("fixtures");

    let request = CodegenRequest {
        schema_path: schema_path.clone(),
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncFixtures {
            output: fixtures_dir.clone(),
        },
        dto: None,
    };

    let files = match CodegenEngine::execute(request)? {
        CodegenOutcome::Files(files) => files,
        CodegenOutcome::InMemory(_) => panic!("fixture generation should emit files"),
    };

    assert!(!files.is_empty(), "no fixtures generated");
    for asset in files {
        assert!(asset.path.exists(), "missing fixture {}", asset.path.display());
    }

    Ok(())
}

#[test]
fn asyncapi_test_app_generation_writes_python_handler() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "asyncapi.yaml", SIMPLE_ASYNCAPI);
    let output = dir.path().join("app.py");

    let request = CodegenRequest {
        schema_path,
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncTestApp {
            language: TargetLanguage::Python,
            output: output.clone(),
        },
        dto: None,
    };

    let files = match CodegenEngine::execute(request)? {
        CodegenOutcome::Files(files) => files,
        CodegenOutcome::InMemory(_) => panic!("test app generation should emit files"),
    };

    assert_eq!(files.len(), 1, "expected single asset");
    assert!(output.exists(), "test app file missing");
    let contents = fs::read_to_string(output)?;
    assert!(
        contents.contains("async def handle_websocket"),
        "expected websocket handler in generated app"
    );
    compile_python_file(&contents)?;

    Ok(())
}

#[test]
fn asyncapi_test_app_generation_writes_node_handler() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "asyncapi.yaml", SIMPLE_ASYNCAPI);
    let output = dir.path().join("app.ts");

    let request = CodegenRequest {
        schema_path,
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncTestApp {
            language: TargetLanguage::TypeScript,
            output: output.clone(),
        },
        dto: None,
    };

    let files = match CodegenEngine::execute(request)? {
        CodegenOutcome::Files(files) => files,
        CodegenOutcome::InMemory(_) => panic!("test app generation should emit files"),
    };

    assert_eq!(files.len(), 1, "expected single asset");
    assert!(output.exists(), "Node test app file missing");
    let contents = fs::read_to_string(output)?;
    assert!(
        contents.contains("async function handleWebSocket"),
        "expected websocket handler in Node app"
    );

    Ok(())
}

#[test]
fn asyncapi_test_app_generation_writes_ruby_handler() -> Result<()> {
    let dir = tempdir()?;
    let schema_path = write_temp_file(dir.path(), "asyncapi.yaml", SIMPLE_ASYNCAPI);
    let output = dir.path().join("app.rb");

    let request = CodegenRequest {
        schema_path,
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncTestApp {
            language: TargetLanguage::Ruby,
            output: output.clone(),
        },
        dto: None,
    };

    let files = match CodegenEngine::execute(request)? {
        CodegenOutcome::Files(files) => files,
        CodegenOutcome::InMemory(_) => panic!("test app generation should emit files"),
    };

    assert_eq!(files.len(), 1, "expected single asset");
    assert!(output.exists(), "Ruby test app file missing");
    let contents = fs::read_to_string(output)?;
    assert!(
        contents.contains("def handle_websocket"),
        "expected websocket handler in Ruby app"
    );
    assert!(contents.contains("Faye::WebSocket"), "expected Faye WebSocket usage");

    Ok(())
}

fn assert_python_class_executes(class_name: &str, code: &str) -> Result<()> {
    let dir = tempdir()?;
    let stub_dir = create_python_stub_dir(dir.path())?;
    let module_path = dir.path().join("generated_app.py");
    fs::write(&module_path, code)?;

    let script = format!(
        r#"
import importlib.util
import sys
spec = importlib.util.spec_from_file_location("generated_app", r"{path}")
module = importlib.util.module_from_spec(spec)
sys.modules["generated_app"] = module
spec.loader.exec_module(module)
instance = getattr(module, "{class_name}")(message="hello")
assert instance.message == "hello"
"#,
        path = module_path.display(),
        class_name = class_name
    );

    let pythonpath = format!("{}:{}", stub_dir.display(), pythonpath_env());
    let status = Command::new("python3")
        .env("PYTHONPATH", pythonpath)
        .arg("-c")
        .arg(script)
        .status()
        .expect("failed to run python");

    assert!(status.success(), "python execution failed");
    Ok(())
}

fn compile_python_file(code: &str) -> Result<()> {
    let dir = tempdir()?;
    let stub_dir = create_python_stub_dir(dir.path())?;
    let module_path = dir.path().join("async_app.py");
    fs::write(&module_path, code)?;
    let pythonpath = format!("{}:{}", stub_dir.display(), pythonpath_env());
    let status = Command::new("python3")
        .env("PYTHONPATH", pythonpath)
        .arg("-m")
        .arg("py_compile")
        .arg(&module_path)
        .status()
        .expect("failed to run python");
    assert!(status.success(), "python compilation failed");
    Ok(())
}

fn pythonpath_env() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/python")
        .canonicalize()
        .expect("failed to resolve python package path");
    path.to_string_lossy().into_owned()
}

fn create_python_stub_dir(base: &Path) -> Result<PathBuf> {
    let root = base.join("py_stubs");
    let spikard_dir = root.join("spikard");
    fs::create_dir_all(&spikard_dir)?;
    fs::write(
        spikard_dir.join("__init__.py"),
        r#"
class _Param:
    def __class_getitem__(cls, item):
        return cls

Body = Path = Query = _Param

class Request:
    ...

class Spikard:
    def __call__(self, *args, **kwargs):
        pass

    def route(self, *args, **kwargs):
        def decorator(fn):
            return fn
        return decorator

def route(*args, **kwargs):
    def decorator(fn):
        return fn
    return decorator
"#,
    )?;

    fs::write(
        root.join("_spikard.py"),
        r#"
class Response:
    ...

class StreamingResponse:
    ...
"#,
    )?;

    Ok(root)
}
