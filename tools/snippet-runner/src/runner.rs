use crate::error::Result;
use crate::types::{RunSummary, Snippet, SnippetAnnotation, SnippetStatus, ValidationLevel, ValidationResult};
use crate::validators::ValidatorRegistry;
use rayon::prelude::*;
use std::time::Instant;

pub struct RunnerConfig {
    pub level: ValidationLevel,
    pub parallelism: usize,
    pub timeout_secs: u64,
    pub fail_fast: bool,
}

impl Default for RunnerConfig {
    fn default() -> Self {
        Self {
            level: ValidationLevel::Syntax,
            parallelism: available_parallelism(),
            timeout_secs: 30,
            fail_fast: false,
        }
    }
}

fn available_parallelism() -> usize {
    std::thread::available_parallelism().map_or(4, std::num::NonZeroUsize::get)
}

pub fn run_validation(snippets: &[Snippet], registry: &ValidatorRegistry, config: &RunnerConfig) -> Result<RunSummary> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.parallelism)
        .build()
        .map_err(|err| crate::error::Error::Other(format!("failed to build thread pool: {err}")))?;

    let fail_fast = config.fail_fast;
    let results: Vec<ValidationResult> = pool.install(|| {
        if fail_fast {
            let mut results = Vec::with_capacity(snippets.len());
            for snippet in snippets {
                let result = validate_one(snippet, registry, config);
                let should_stop = matches!(result.status, SnippetStatus::Fail | SnippetStatus::Error);
                results.push(result);
                if should_stop {
                    break;
                }
            }
            results
        } else {
            snippets
                .par_iter()
                .map(|snippet| validate_one(snippet, registry, config))
                .collect()
        }
    });

    Ok(RunSummary::from_results(results))
}

fn validate_one(snippet: &Snippet, registry: &ValidatorRegistry, config: &RunnerConfig) -> ValidationResult {
    if let Some(annotation) = &snippet.annotation {
        match annotation {
            SnippetAnnotation::Skip => {
                return ValidationResult {
                    snippet: snippet.clone(),
                    status: SnippetStatus::Skip,
                    level: config.level,
                    message: Some("skipped via annotation".to_string()),
                    duration_ms: 0,
                };
            }
            SnippetAnnotation::SyntaxOnly if config.level > ValidationLevel::Syntax => {
                return ValidationResult {
                    snippet: snippet.clone(),
                    status: SnippetStatus::Skip,
                    level: config.level,
                    message: Some("annotation limits to syntax-only".to_string()),
                    duration_ms: 0,
                };
            }
            SnippetAnnotation::CompileOnly if config.level > ValidationLevel::Compile => {
                return ValidationResult {
                    snippet: snippet.clone(),
                    status: SnippetStatus::Skip,
                    level: config.level,
                    message: Some("annotation limits to compile-only".to_string()),
                    duration_ms: 0,
                };
            }
            _ => {}
        }
    }

    let Some(validator) = registry.get(snippet.language) else {
        return ValidationResult {
            snippet: snippet.clone(),
            status: SnippetStatus::Unavailable,
            level: config.level,
            message: Some(format!("no validator for {}", snippet.language)),
            duration_ms: 0,
        };
    };

    if !validator.is_available() {
        return ValidationResult {
            snippet: snippet.clone(),
            status: SnippetStatus::Unavailable,
            level: config.level,
            message: Some(format!("{} toolchain not found", snippet.language)),
            duration_ms: 0,
        };
    }

    let effective_level = config.level.min(validator.max_level());
    let start = Instant::now();
    let (mut status, message) = match validator.validate(snippet, effective_level, config.timeout_secs) {
        Ok((status, message)) => (status, message),
        Err(err) => (SnippetStatus::Error, Some(err.to_string())),
    };
    let duration_ms = start.elapsed().as_millis() as u64;

    if status == SnippetStatus::Fail
        && effective_level == ValidationLevel::Syntax
        && let Some(error_output) = &message
        && validator.is_dependency_error(error_output)
    {
        status = SnippetStatus::Pass;
    }

    ValidationResult {
        snippet: snippet.clone(),
        status,
        level: effective_level,
        message,
        duration_ms,
    }
}
