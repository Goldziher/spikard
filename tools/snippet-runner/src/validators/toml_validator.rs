use crate::error::Result;
use crate::types::{Language, Snippet, SnippetStatus, ValidationLevel};
use crate::validators::SnippetValidator;

pub struct TomlValidator;

impl SnippetValidator for TomlValidator {
    fn language(&self) -> Language {
        Language::Toml
    }

    fn is_available(&self) -> bool {
        true
    }

    fn validate(
        &self,
        snippet: &Snippet,
        _level: ValidationLevel,
        _timeout_secs: u64,
    ) -> Result<(SnippetStatus, Option<String>)> {
        match snippet.code.parse::<toml::Table>() {
            Ok(_) => Ok((SnippetStatus::Pass, None)),
            Err(err) => Ok((SnippetStatus::Fail, Some(err.to_string()))),
        }
    }

    fn max_level(&self) -> ValidationLevel {
        ValidationLevel::Syntax
    }
}
