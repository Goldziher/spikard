use crate::error::Result;
use crate::parser;
use crate::types::{Language, Snippet, SnippetAnnotation};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Discover snippets beneath the provided directories.
///
/// # Errors
///
/// Returns an error when a source file cannot be parsed into snippet blocks.
pub fn discover_snippets(dirs: &[PathBuf], language_filter: Option<&[Language]>) -> Result<Vec<Snippet>> {
    let mut snippets = Vec::new();

    for dir in dirs {
        if !dir.exists() {
            continue;
        }

        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let path = entry.path();
            let file_snippets = extract_snippets_from_file(path, dir)?;

            for snippet in file_snippets {
                if let Some(filter) = language_filter
                    && !filter.contains(&snippet.language)
                {
                    continue;
                }

                snippets.push(snippet);
            }
        }
    }

    snippets.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.block_index.cmp(&right.block_index))
    });
    Ok(snippets)
}

fn extract_snippets_from_file(path: &Path, base_dir: &Path) -> Result<Vec<Snippet>> {
    let blocks = parser::parse_code_blocks(path)?;
    let dir_language = infer_language_from_path(path, base_dir);
    let mut snippets = Vec::new();

    for (index, block) in blocks.into_iter().enumerate() {
        let language = {
            let from_fence = Language::from_fence_tag(&block.lang);
            if from_fence == Language::Unknown {
                dir_language.unwrap_or(Language::Unknown)
            } else {
                from_fence
            }
        };

        if language == Language::Unknown {
            continue;
        }

        let annotation = block.preceding_comment.as_deref().and_then(parse_annotation);

        snippets.push(Snippet {
            path: path.to_path_buf(),
            language,
            title: block.title,
            code: block.code,
            start_line: block.start_line,
            block_index: index,
            annotation,
        });
    }

    Ok(snippets)
}

fn infer_language_from_path(path: &Path, base_dir: &Path) -> Option<Language> {
    let relative = path.strip_prefix(base_dir).ok()?;
    for component in relative.components() {
        let dir_name = component.as_os_str().to_str()?;
        let language = Language::from_dir_name(dir_name);
        if language != Language::Unknown {
            return Some(language);
        }
    }

    None
}

fn parse_annotation(comment: &str) -> Option<SnippetAnnotation> {
    let inner = comment.trim().strip_prefix("<!--")?.strip_suffix("-->")?.trim();

    match inner {
        "snippet:skip" => Some(SnippetAnnotation::Skip),
        "snippet:compile-only" => Some(SnippetAnnotation::CompileOnly),
        "snippet:syntax-only" => Some(SnippetAnnotation::SyntaxOnly),
        _ => None,
    }
}

#[must_use]
pub fn count_by_language(snippets: &[Snippet]) -> Vec<(Language, usize)> {
    let mut counts: HashMap<Language, usize> = HashMap::new();
    for snippet in snippets {
        *counts.entry(snippet.language).or_default() += 1;
    }

    let mut result: Vec<_> = counts.into_iter().collect();
    result.sort_by_key(|(language, _)| language.to_string());
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn parses_annotations() {
        assert_eq!(parse_annotation("<!-- snippet:skip -->"), Some(SnippetAnnotation::Skip));
        assert_eq!(
            parse_annotation("<!-- snippet:compile-only -->"),
            Some(SnippetAnnotation::CompileOnly)
        );
        assert_eq!(
            parse_annotation("<!-- snippet:syntax-only -->"),
            Some(SnippetAnnotation::SyntaxOnly)
        );
    }

    #[test]
    fn infers_language_from_nested_snippet_path() {
        let base = Path::new("/repo/docs");
        let path = Path::new("/repo/docs/snippets/python/example.md");
        assert_eq!(infer_language_from_path(path, base), Some(Language::Python));
    }

    #[test]
    fn does_not_infer_language_from_non_language_directories() {
        let base = Path::new("/repo/docs");
        let path = Path::new("/repo/docs/cli/usage.md");
        assert_eq!(infer_language_from_path(path, base), None);
    }
}
