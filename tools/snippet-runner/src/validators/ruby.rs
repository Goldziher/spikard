use crate::error::Result;
use crate::types::{Language, Snippet, SnippetStatus, ValidationLevel};
use crate::validators::{SnippetValidator, run_command};
use std::io::Write;
use tempfile::NamedTempFile;

pub struct RubyValidator;

impl SnippetValidator for RubyValidator {
    fn language(&self) -> Language {
        Language::Ruby
    }

    fn is_available(&self) -> bool {
        which::which("ruby").is_ok()
    }

    fn validate(
        &self,
        snippet: &Snippet,
        level: ValidationLevel,
        timeout_secs: u64,
    ) -> Result<(SnippetStatus, Option<String>)> {
        if is_api_signature(snippet.code.trim()) {
            return Ok((SnippetStatus::Pass, None));
        }

        let mut temp_file = NamedTempFile::with_suffix(".rb")?;
        temp_file.write_all(snippet.code.as_bytes())?;
        temp_file.flush()?;

        let path = temp_file.path().to_string_lossy().to_string();
        let mut command = match level {
            ValidationLevel::Syntax | ValidationLevel::Compile => {
                let mut command = std::process::Command::new("ruby");
                command.args(["-c", &path]);
                command
            }
            ValidationLevel::Run => {
                let mut command = std::process::Command::new("ruby");
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

fn is_api_signature(code: &str) -> bool {
    code.lines().count() <= 3 && code.contains(" -> ")
}
