use crate::error::Result;
use crate::types::{Language, Snippet, SnippetStatus, ValidationLevel};
use crate::validators::{SnippetValidator, run_command};
use tempfile::TempDir;

pub struct PythonValidator;

impl PythonValidator {
    fn patch_code(code: &str) -> String {
        let trimmed = code.trim();

        if trimmed.starts_with(' ') || trimmed.starts_with('\t') {
            let min_indent = trimmed
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.len() - line.trim_start().len())
                .min()
                .unwrap_or(0);

            if min_indent > 0 {
                let dedented = trimmed
                    .lines()
                    .map(|line| {
                        if line.trim().is_empty() {
                            String::new()
                        } else if line.len() > min_indent {
                            line[min_indent..].to_string()
                        } else {
                            line.trim().to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                return Self::patch_signatures(&dedented);
            }
        }

        Self::patch_signatures(code)
    }

    fn patch_signatures(code: &str) -> String {
        let lines: Vec<&str> = code.lines().collect();
        let mut output = Vec::new();
        let mut index = 0;

        while index < lines.len() {
            output.push(lines[index].to_string());
            let trimmed = lines[index].trim();
            let is_def_start =
                trimmed.starts_with("def ") || trimmed.starts_with("async def ") || trimmed.starts_with("class ");

            if is_def_start {
                let mut signature_end = index;
                let mut has_inline_body = false;

                while signature_end < lines.len() {
                    let trimmed_signature = lines[signature_end].trim();
                    if signature_end > index {
                        output.push(lines[signature_end].to_string());
                    }

                    if trimmed_signature.ends_with(':') {
                        break;
                    }

                    if let Some(arrow_pos) = trimmed_signature.find("->") {
                        let after_arrow = &trimmed_signature[arrow_pos + 2..];
                        if let Some(colon_pos) = after_arrow.find(':') {
                            let after_colon = after_arrow[colon_pos + 1..].trim();
                            if !after_colon.is_empty() {
                                has_inline_body = true;
                            }
                            break;
                        }

                        if signature_end == index {
                            let last = output.len() - 1;
                            output[last] = format!("{}:", lines[signature_end]);
                        }
                        break;
                    }

                    if trimmed_signature.contains("): ") || trimmed_signature.contains("):\t") {
                        has_inline_body = true;
                        break;
                    }

                    if trimmed_signature.ends_with(')') && signature_end > index {
                        let last = output.len() - 1;
                        output[last] = format!("{}:", output[last]);
                        break;
                    }

                    signature_end += 1;
                }

                if signature_end >= lines.len() {
                    let last = output.len() - 1;
                    if !output[last].trim().ends_with(':') {
                        output[last] = format!("{}:", output[last]);
                    }
                    let indent = lines[index].chars().take_while(|c| c.is_whitespace()).count();
                    output.push(format!("{}...", " ".repeat(indent + 4)));
                    index = signature_end;
                    continue;
                }

                if has_inline_body {
                    index = signature_end + 1;
                    continue;
                }

                let next_content = (signature_end + 1..lines.len())
                    .find(|candidate| !lines[*candidate].trim().is_empty())
                    .map(|candidate| lines[candidate]);

                let has_body = next_content.is_some_and(|line| line.starts_with(' ') || line.starts_with('\t'));
                if !has_body {
                    let last = output.len() - 1;
                    if !output[last].trim().ends_with(':') {
                        output[last] = format!("{}:", output[last]);
                    }

                    let indent = lines[index].chars().take_while(|c| c.is_whitespace()).count();
                    output.push(format!("{}...", " ".repeat(indent + 4)));
                }

                index = signature_end + 1;
                continue;
            }

            index += 1;
        }

        output.join("\n")
    }
}

impl SnippetValidator for PythonValidator {
    fn language(&self) -> Language {
        Language::Python
    }

    fn is_available(&self) -> bool {
        which::which("python3").is_ok() || which::which("python").is_ok()
    }

    fn validate(
        &self,
        snippet: &Snippet,
        level: ValidationLevel,
        timeout_secs: u64,
    ) -> Result<(SnippetStatus, Option<String>)> {
        let dir = TempDir::new()?;
        let code = Self::patch_code(&snippet.code);
        let snippet_path = dir.path().join("snippet.py");
        std::fs::write(&snippet_path, &code)?;

        let python = if which::which("python3").is_ok() {
            "python3"
        } else {
            "python"
        };
        let path = snippet_path.to_string_lossy().to_string();

        let mut command = match level {
            ValidationLevel::Syntax => {
                let checker_path = dir.path().join("check.py");
                let checker = "\
import ast, sys
try:
    with open(sys.argv[1]) as f:
        ast.parse(f.read())
except SyntaxError as e:
    print(f\"{e}\", file=sys.stderr)
    sys.exit(1)
";
                std::fs::write(&checker_path, checker)?;

                let mut command = std::process::Command::new(python);
                command.args([checker_path.to_string_lossy().as_ref(), &path]);
                command
            }
            ValidationLevel::Compile => {
                let mut command = std::process::Command::new(python);
                command.args(["-m", "py_compile", &path]);
                command
            }
            ValidationLevel::Run => {
                let mut command = std::process::Command::new(python);
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

    fn is_dependency_error(&self, output: &str) -> bool {
        output.contains("unexpected indent") || output.contains("was never closed")
    }
}

#[cfg(test)]
mod tests {
    use super::PythonValidator;

    #[test]
    fn preserves_multiline_async_signature_lines() {
        let code = r"class UserServiceHandler:
    async def CreateUsers(
        self, request_iterator
    ) -> CreateUsersResponse:
        created_users = []
        return created_users
";

        let patched = PythonValidator::patch_code(code);
        assert!(patched.contains(") -> CreateUsersResponse:"));
        assert!(patched.contains("created_users = []"));
    }
}
