use crate::error::Result;
use crate::types::{Language, Snippet, SnippetStatus, ValidationLevel};
use crate::validators::{SnippetValidator, run_command};
use std::io::Write;
use tempfile::NamedTempFile;

pub struct PhpValidator;

impl SnippetValidator for PhpValidator {
    fn language(&self) -> Language {
        Language::Php
    }

    fn is_available(&self) -> bool {
        which::which("php").is_ok()
    }

    fn validate(
        &self,
        snippet: &Snippet,
        level: ValidationLevel,
        timeout_secs: u64,
    ) -> Result<(SnippetStatus, Option<String>)> {
        let mut temp_file = NamedTempFile::with_suffix(".php")?;
        temp_file.write_all(snippet.code.as_bytes())?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let mut command = match level {
            ValidationLevel::Syntax | ValidationLevel::Compile => {
                let mut command = std::process::Command::new("php");
                command.args(["-l", &path]);
                command
            }
            ValidationLevel::Run => {
                let mut command = std::process::Command::new("php");
                command.arg(&path);
                command
            }
        };

        let (success, output) = run_command(&mut command, timeout_secs)?;
        if success {
            Ok((SnippetStatus::Pass, None))
        } else {
            Ok((SnippetStatus::Fail, Some(output)))
        }
    }

    fn max_level(&self) -> ValidationLevel {
        ValidationLevel::Run
    }
}

