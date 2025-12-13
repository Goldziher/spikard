use spikard_cli::codegen::{
    CodegenEngine, CodegenOutcome, CodegenRequest, CodegenTargetKind, SchemaKind, TargetLanguage,
    generate_openrpc_php_handler_app, generate_openrpc_python_handler_app, generate_openrpc_ruby_handler_app,
    generate_openrpc_typescript_handler_app, parse_openrpc_schema,
};
use std::fs;
use tempfile::{Builder, tempdir};

fn write_temp_spec(contents: &str, suffix: &str) -> tempfile::NamedTempFile {
    let mut file = Builder::new().suffix(suffix).tempfile().expect("tempfile");
    std::io::Write::write_all(&mut file, contents.as_bytes()).expect("write spec");
    file
}

#[test]
fn parse_openrpc_schema_supports_yaml_and_json() {
    let yaml = r#"
openrpc: "1.3.2"
info:
  title: "Test API"
  version: "0.1.0"
methods:
  - name: "math.add"
    params:
      - name: "x"
        required: true
        schema:
          type: integer
      - name: "y"
        required: true
        schema:
          type: integer
    result:
      name: "sum"
      schema:
        type: integer
"#;
    let yaml_file = write_temp_spec(yaml, ".yaml");
    let yaml_spec = parse_openrpc_schema(yaml_file.path()).expect("parse yaml");
    assert_eq!(yaml_spec.openrpc, "1.3.2");
    assert_eq!(yaml_spec.methods.len(), 1);
    assert_eq!(yaml_spec.methods[0].name, "math.add");

    let json = r#"
{
  "openrpc": "1.3.2",
  "info": { "title": "Test API", "version": "0.1.0" },
  "methods": [
    {
      "name": "math.add",
      "params": [
        { "name": "x", "required": true, "schema": { "type": "integer" } },
        { "name": "y", "required": true, "schema": { "type": "integer" } }
      ],
      "result": { "name": "sum", "schema": { "type": "integer" } }
    }
  ]
}
"#;
    let json_file = write_temp_spec(json, ".json");
    let json_spec = parse_openrpc_schema(json_file.path()).expect("parse json");
    assert_eq!(json_spec.methods[0].name, "math.add");
}

#[test]
fn openrpc_generators_include_method_names() {
    let yaml = r#"
openrpc: "1.3.2"
info:
  title: "Test API"
  version: "0.1.0"
methods:
  - name: "math.add"
    params: []
    result:
      name: "ok"
      schema:
        type: object
"#;
    let file = write_temp_spec(yaml, ".yaml");
    let spec = parse_openrpc_schema(file.path()).expect("parse spec");

    let python = generate_openrpc_python_handler_app(&spec).expect("python generator");
    assert!(python.contains("math.add"));

    let ts = generate_openrpc_typescript_handler_app(&spec).expect("ts generator");
    assert!(ts.contains("math.add"));

    let ruby = generate_openrpc_ruby_handler_app(&spec).expect("ruby generator");
    assert!(ruby.contains("math.add"));

    let php = generate_openrpc_php_handler_app(&spec).expect("php generator");
    assert!(php.contains("math.add"));
}

#[test]
fn codegen_engine_writes_openrpc_handlers_to_disk() {
    let yaml = r#"
openrpc: "1.3.2"
info:
  title: "Test API"
  version: "0.1.0"
methods:
  - name: "math.add"
    params: []
    result:
      name: "ok"
      schema:
        type: object
"#;
    let spec_file = write_temp_spec(yaml, ".yaml");
    let out_dir = tempdir().expect("tempdir");
    let output = out_dir.path().join("nested").join("handlers.py");

    let request = CodegenRequest {
        schema_path: spec_file.path().to_path_buf(),
        schema_kind: SchemaKind::OpenRpc,
        target: CodegenTargetKind::JsonRpcHandlers {
            language: TargetLanguage::Python,
            output: output.clone(),
        },
        dto: None,
    };

    let outcome = CodegenEngine::execute(request).expect("engine execute");
    match outcome {
        CodegenOutcome::Files(files) => {
            assert_eq!(files.len(), 1);
            assert_eq!(files[0].path, output);
        }
        CodegenOutcome::InMemory(_) => panic!("expected file output"),
    }

    let contents = fs::read_to_string(&output).expect("read output");
    assert!(contents.contains("math.add"));
}
