use crate::types::Language;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub lang: String,
    pub title: Option<String>,
    pub code: String,
    pub start_line: usize,
    pub preceding_comment: Option<String>,
}

#[must_use]
pub fn extract_fenced_blocks(content: &str) -> Vec<CodeBlock> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("```") {
            if rest.is_empty() || rest.starts_with('`') {
                index += 1;
                continue;
            }

            let (lang, title) = parse_fence_info(rest);
            if lang.is_empty() {
                index += 1;
                continue;
            }

            let preceding_comment = if index > 0 {
                let previous = lines[index - 1].trim();
                if previous.starts_with("<!--") && previous.ends_with("-->") {
                    Some(previous.to_string())
                } else {
                    None
                }
            } else {
                None
            };

            let start_line = index + 1;
            let mut code_lines = Vec::new();
            index += 1;

            while index < lines.len() {
                let closing = lines[index].trim();
                if closing == "```" || (closing.starts_with("```") && closing.chars().skip(3).all(|c| c == '`')) {
                    break;
                }

                code_lines.push(lines[index]);
                index += 1;
            }

            let code = code_lines.join("\n");
            if !code.trim().is_empty() {
                blocks.push(CodeBlock {
                    lang,
                    title,
                    code,
                    start_line,
                    preceding_comment,
                });
            }
        }

        index += 1;
    }

    blocks
}

pub fn parse_code_blocks(path: &Path) -> crate::error::Result<Vec<CodeBlock>> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::InvalidData => return Ok(Vec::new()),
        Err(err) => {
            return Err(crate::error::Error::Parse {
                path: path.to_path_buf(),
                reason: err.to_string(),
            });
        }
    };

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_lowercase();

    if extension == "md" || extension == "markdown" {
        return Ok(extract_fenced_blocks(&content));
    }

    let fenced = extract_fenced_blocks(&content);
    if !fenced.is_empty() {
        return Ok(fenced);
    }

    let language = Language::from_extension(&extension);
    if language == Language::Unknown {
        return Ok(Vec::new());
    }

    Ok(vec![CodeBlock {
        lang: language.to_string(),
        title: path.file_name().and_then(|name| name.to_str()).map(str::to_string),
        code: content,
        start_line: 1,
        preceding_comment: None,
    }])
}

fn parse_fence_info(info: &str) -> (String, Option<String>) {
    let trimmed = info.trim();
    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let lang = parts.next().unwrap_or_default().to_string();
    let rest = parts.next().unwrap_or_default();
    let title = parse_title_attr(rest);
    (lang, title)
}

fn parse_title_attr(attrs: &str) -> Option<String> {
    let trimmed = attrs.trim();

    if let Some(after) = trimmed.strip_prefix("title=") {
        let after = after.trim();

        if let Some(stripped) = after.strip_prefix('"') {
            let end = stripped.find('"')?;
            return Some(stripped[..end].to_string());
        }

        if let Some(stripped) = after.strip_prefix('\'') {
            let end = stripped.find('\'')?;
            return Some(stripped[..end].to_string());
        }

        let value: String = after.chars().take_while(|c| !c.is_whitespace()).collect();
        if !value.is_empty() {
            return Some(value);
        }
    }

    for part in trimmed.split_whitespace() {
        if let Some(after) = part.strip_prefix("title=") {
            let value = after.trim_matches(|c| c == '"' || c == '\'');
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_single_block() {
        let markdown = r#"
```rust title="example"
fn main() {}
```
"#;

        let blocks = extract_fenced_blocks(markdown);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].lang, "rust");
        assert_eq!(blocks[0].title.as_deref(), Some("example"));
    }

    #[test]
    fn preserves_annotations() {
        let markdown = r#"
<!-- snippet:skip -->
```python
print("hello")
```
"#;

        let blocks = extract_fenced_blocks(markdown);
        assert_eq!(blocks[0].preceding_comment.as_deref(), Some("<!-- snippet:skip -->"));
    }
}
