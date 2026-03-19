pub mod bash;
pub mod elixir;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod toml_validator;
pub mod typescript;

use crate::error::Result;
use crate::types::{Language, Snippet, SnippetStatus, ValidationLevel};
use std::collections::HashMap;
use std::io::Read;

pub trait SnippetValidator: Send + Sync {
    fn language(&self) -> Language;
    fn is_available(&self) -> bool;
    /// Validate a snippet at the requested level.
    ///
    /// # Errors
    ///
    /// Returns an error when the validator cannot execute its underlying toolchain.
    fn validate(
        &self,
        snippet: &Snippet,
        level: ValidationLevel,
        timeout_secs: u64,
    ) -> Result<(SnippetStatus, Option<String>)>;
    fn max_level(&self) -> ValidationLevel;

    fn is_dependency_error(&self, _error_output: &str) -> bool {
        false
    }
}

pub struct ValidatorRegistry {
    validators: HashMap<Language, Box<dyn SnippetValidator>>,
}

impl ValidatorRegistry {
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Self {
            validators: HashMap::new(),
        };

        registry.register(Box::new(rust::RustValidator));
        registry.register(Box::new(python::PythonValidator));
        registry.register(Box::new(typescript::TypeScriptValidator));
        registry.register(Box::new(php::PhpValidator));
        registry.register(Box::new(ruby::RubyValidator));
        registry.register(Box::new(elixir::ElixirValidator));
        registry.register(Box::new(bash::BashValidator));
        registry.register(Box::new(toml_validator::TomlValidator));

        registry
    }

    fn register(&mut self, validator: Box<dyn SnippetValidator>) {
        self.validators.insert(validator.language(), validator);
    }

    #[must_use]
    pub fn get(&self, language: Language) -> Option<&dyn SnippetValidator> {
        self.validators.get(&language).map(Box::as_ref)
    }
}

impl Default for ValidatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

fn strip_ansi_codes(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            if matches!(chars.next(), Some('[')) {
                for next in chars.by_ref() {
                    if next == 'm' {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Run a child process with a timeout and capture combined stdout/stderr.
///
/// # Errors
///
/// Returns an error when the child process cannot be spawned, waited on, or times out.
pub fn run_command(command: &mut std::process::Command, timeout_secs: u64) -> Result<(bool, String)> {
    let mut child = command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|err| crate::error::Error::Other(format!("spawn failed: {err}")))?;

    let timeout = std::time::Duration::from_secs(timeout_secs);
    match child.wait_timeout(timeout) {
        Ok(Some(status)) => {
            let mut output = String::new();

            if let Some(mut stdout) = child.stdout.take() {
                let _ = stdout.read_to_string(&mut output);
            }

            if let Some(mut stderr) = child.stderr.take() {
                let _ = stderr.read_to_string(&mut output);
            }

            Ok((status.success(), strip_ansi_codes(&output)))
        }
        Ok(None) => {
            let _ = child.kill();
            let _ = child.wait();
            Err(crate::error::Error::Timeout {
                command: format!("{command:?}"),
                timeout_secs,
            })
        }
        Err(err) => Err(crate::error::Error::Other(format!("wait failed: {err}"))),
    }
}

trait WaitTimeout {
    fn wait_timeout(&mut self, timeout: std::time::Duration) -> std::io::Result<Option<std::process::ExitStatus>>;
}

impl WaitTimeout for std::process::Child {
    fn wait_timeout(&mut self, timeout: std::time::Duration) -> std::io::Result<Option<std::process::ExitStatus>> {
        let start = std::time::Instant::now();
        let poll_interval = std::time::Duration::from_millis(50);

        loop {
            if let Some(status) = self.try_wait()? {
                return Ok(Some(status));
            }

            if start.elapsed() >= timeout {
                return Ok(None);
            }

            std::thread::sleep(poll_interval);
        }
    }
}
