use crate::error::Result;
use crate::types::{RunSummary, Snippet, SnippetStatus, ValidationResult};
use std::path::Path;

pub fn print_summary(summary: &RunSummary, show_code: bool) {
    println!();
    println!(
        "{:<60} {:<12} {:<10} {:<8} TIME",
        "SNIPPET", "LANGUAGE", "STATUS", "LEVEL"
    );
    println!("{}", "-".repeat(100));

    for result in &summary.results {
        let file_name = result
            .snippet
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("?");

        let status = match result.status {
            SnippetStatus::Pass => "PASS",
            SnippetStatus::Fail => "FAIL",
            SnippetStatus::Skip => "SKIP",
            SnippetStatus::Error => "ERROR",
            SnippetStatus::Unavailable => "N/A",
        };

        println!(
            "{:<60} {:<12} {:<10} {:<8} {}ms",
            truncate(file_name, 58),
            result.snippet.language,
            status,
            result.level,
            result.duration_ms
        );

        if matches!(result.status, SnippetStatus::Fail | SnippetStatus::Error) {
            let title_info = result
                .snippet
                .title
                .as_deref()
                .map(|title| format!(" (title: {title})"))
                .unwrap_or_default();
            println!(
                "  Source: {}:{}{}",
                result.snippet.path.display(),
                result.snippet.start_line,
                title_info
            );

            if let Some(message) = &result.message {
                let trimmed = message.trim();
                if !trimmed.is_empty() {
                    println!("  Error:");
                    for line in trimmed.lines() {
                        println!("    {line}");
                    }
                }
            }

            if show_code {
                println!("  Code:");
                for (index, line) in result.snippet.code.lines().enumerate() {
                    println!("    {:>3} | {line}", index + 1);
                }
            }

            println!();
        }
    }

    println!("{}", "-".repeat(100));
    println!(
        "Total: {}  Passed: {}  Failed: {}  Skipped: {}  Errors: {}  Unavailable: {}",
        summary.total, summary.passed, summary.failed, summary.skipped, summary.errors, summary.unavailable
    );
    println!();
}

/// Write validation results to a JSON file.
///
/// # Errors
///
/// Returns an error when serialization fails or the destination cannot be written.
pub fn write_json(results: &[ValidationResult], path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    std::fs::write(path, json)?;
    Ok(())
}

pub fn print_snippet_list(snippets: &[Snippet]) {
    println!("{:<60} {:<12} {:<8} TITLE", "FILE", "LANGUAGE", "LINE");
    println!("{}", "-".repeat(95));

    for snippet in snippets {
        let file_name = snippet.path.file_name().and_then(|name| name.to_str()).unwrap_or("?");

        println!(
            "{:<60} {:<12} {:<8} {}",
            truncate(file_name, 58),
            snippet.language,
            snippet.start_line,
            snippet.title.as_deref().unwrap_or("-")
        );
    }

    println!("{}", "-".repeat(95));
    println!("Total: {} snippets", snippets.len());
}

fn truncate(value: &str, max: usize) -> String {
    if value.len() <= max {
        value.to_string()
    } else {
        format!("{}...", &value[..max.saturating_sub(3)])
    }
}
