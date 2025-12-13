use spikard_cli::codegen::{
    generate_openrpc_php_handler_app, generate_openrpc_python_handler_app, generate_openrpc_ruby_handler_app,
    generate_openrpc_typescript_handler_app, parse_openrpc_schema,
};
use std::path::Path;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn openrpc_generators_produce_handlers_for_example_spec() {
    let spec = parse_openrpc_schema(Path::new(
        &workspace_root().join("examples/schemas/user-api.openrpc.json"),
    ))
    .expect("parse OpenRPC example spec");

    let python = generate_openrpc_python_handler_app(&spec).expect("python generation");
    assert!(python.contains("user.getById") || python.contains("user_get_by_id"));

    let typescript = generate_openrpc_typescript_handler_app(&spec).expect("ts generation");
    assert!(typescript.contains("user.getById") || typescript.contains("userGetById"));

    let ruby = generate_openrpc_ruby_handler_app(&spec).expect("ruby generation");
    assert!(ruby.contains("user.getById") || ruby.contains("user_get_by_id"));

    let php = generate_openrpc_php_handler_app(&spec).expect("php generation");
    assert!(php.contains("user.getById") || php.contains("user_get_by_id"));
}
