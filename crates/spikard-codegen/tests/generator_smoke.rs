use spikard_codegen::{GenerateOptions, Generator, Protocol, Target};
use std::fs;

#[test]
fn generator_from_file_validates_and_generates_stub_outputs() {
    let dir = tempfile::tempdir().expect("tempdir");
    let config_path = dir.path().join("spikard.yaml");
    let out_dir = dir.path().join("out");

    fs::write(
        &config_path,
        r#"
version: "1.0"
name: "smoke-service"
http:
  routes:
    - path: "/health"
      method: "GET"
      handler: "handlers.health"
"#,
    )
    .unwrap();

    let generator = Generator::from_file(&config_path).expect("from_file");
    generator.validate().expect("validate");

    generator.generate(Target::Python, &out_dir).expect("generate python");
    generator.generate(Target::TypeScript, &out_dir).expect("generate ts");
    generator.generate(Target::Rust, &out_dir).expect("generate rust");

    let openapi = generator.generate_openapi().expect("openapi");
    assert_eq!(openapi["openapi"], "3.1.0");

    let opts = GenerateOptions::default();
    assert_eq!(opts.target, Target::Python);
    assert_eq!(opts.protocols, None);

    assert_eq!(Protocol::Http as u8, Protocol::Http as u8);
}
