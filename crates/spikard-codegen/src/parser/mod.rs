//! Configuration file parsing

pub mod config;
pub mod resolver;
pub mod validator;

pub use config::Config;
pub use resolver::resolve_refs;
pub use validator::validate_config;

use crate::error::{CodegenError, Result};
use std::fs;
use std::path::Path;

/// Load and parse configuration from a file
pub fn load_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Err(CodegenError::ConfigNotFound(path.to_path_buf()));
    }

    let content = fs::read_to_string(path)?;

    // Determine file type by extension
    let config = if path.extension().and_then(|s| s.to_str()) == Some("json") {
        serde_json::from_str(&content)?
    } else {
        // Default to YAML
        serde_yaml::from_str(&content)?
    };

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_yaml_config() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
version: "1.0"
name: "test-service"
http:
  routes: []
"#
        )
        .unwrap();

        let config = load_config(file.path()).unwrap();
        assert_eq!(config.name, "test-service");
    }

    #[test]
    fn test_load_json_config() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"{{
  "version": "1.0",
  "name": "test-service",
  "http": {{
    "routes": []
  }}
}}"#
        )
        .unwrap();

        let path = file.path().with_extension("json");
        fs::copy(file.path(), &path).unwrap();

        let config = load_config(&path).unwrap();
        assert_eq!(config.name, "test-service");

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_config_not_found() {
        let result = load_config(Path::new("nonexistent.yaml"));
        assert!(matches!(result, Err(CodegenError::ConfigNotFound(_))));
    }
}
