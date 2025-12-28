//! Shared utilities and helpers for GraphQL code generators.

use std::collections::HashMap;

#[allow(dead_code)]
pub fn map_graphql_type_to_language(graphql_type: &str, language: &str, is_nullable: bool) -> String {
    let base_type = match (graphql_type, language) {
        ("String", "python") => "str",
        ("String", "typescript") => "string",
        ("String", _) => "String",
        ("Int", "python") => "int",
        ("Int", "typescript") => "number",
        ("Int", "rust") => "i32",
        ("Int", _) => "int",
        ("Float", "python") => "float",
        ("Float", "typescript") => "number",
        ("Float", "rust") => "f64",
        ("Float", _) => "float",
        ("Boolean", "python") => "bool",
        ("Boolean", "typescript") => "boolean",
        ("Boolean", _) => "bool",
        ("ID", _) => "string",
        (custom, _) => custom,
    };

    if is_nullable {
        match language {
            "python" => format!("Optional[{}]", base_type),
            "typescript" => format!("{} | null", base_type),
            "rust" => format!("Option<{}>", base_type),
            "php" => format!("?{}", base_type),
            _ => base_type.to_string(),
        }
    } else {
        base_type.to_string()
    }
}

#[allow(dead_code)]
pub fn generate_field_docs(field_description: &str, comment_style: &str) -> String {
    match comment_style {
        "python" => format!("    \"\"\"{}\"\"\"", field_description),
        "typescript" | "javascript" => format!("  /** {} */", field_description),
        "rust" => format!("    /// {}", field_description),
        "ruby" => format!("  # {}", field_description),
        "php" => format!("    /** {} */", field_description),
        _ => format!("  // {}", field_description),
    }
}

#[allow(dead_code)]
pub fn to_camel_case(s: &str) -> String {
    let parts: Vec<&str> = s.split('_').collect();
    if parts.is_empty() {
        return String::new();
    }
    let mut result = parts[0].to_string();
    for part in &parts[1..] {
        if !part.is_empty() {
            result.push_str(&part[0..1].to_uppercase());
            if part.len() > 1 {
                result.push_str(&part[1..]);
            }
        }
    }
    result
}

#[allow(dead_code)]
pub fn to_pascal_case(s: &str) -> String {
    let parts: Vec<&str> = s.split(|c: char| !c.is_alphanumeric()).collect();
    parts
        .into_iter()
        .filter(|p| !p.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn indent(code: &str, spaces: usize) -> String {
    let indent_str = " ".repeat(spaces);
    code.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{}{}", indent_str, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn sanitize_identifier(name: &str) -> String {
    let mut ident: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    ident = ident.trim_matches('_').to_string();
    while ident.contains("__") {
        ident = ident.replace("__", "_");
    }
    if ident.is_empty() {
        "field".to_string()
    } else if ident.chars().next().unwrap().is_ascii_digit() {
        format!("_{}", ident)
    } else {
        ident
    }
}

pub fn sanitize_typescript_identifier(name: &str) -> String {
    let identifier = sanitize_identifier(name);
    let parts: Vec<&str> = identifier.split('_').collect();
    if parts.is_empty() {
        return "field".to_string();
    }
    let mut result = parts[0].to_string();
    for part in &parts[1..] {
        if !part.is_empty() {
            result.push_str(&part[0..1].to_uppercase());
            if part.len() > 1 {
                result.push_str(&part[1..]);
            }
        }
    }
    result
}

#[allow(dead_code)]
pub fn format_description(description: &str, max_width: usize) -> String {
    let words: Vec<&str> = description.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in words {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines.join("\n")
}

#[allow(dead_code)]
pub fn escape_string(s: &str, for_language: &str) -> String {
    match for_language {
        "php" => s
            .replace('\\', "\\\\")
            .replace('\'', "\\'")
            .replace('\n', "\\n")
            .replace('\t', "\\t")
            .replace('\r', "\\r"),
        _ => s
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\t', "\\t")
            .replace('\r', "\\r"),
    }
}

#[allow(dead_code)]
pub struct TypeNameCache {
    cache: HashMap<String, String>,
}

impl TypeNameCache {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    #[allow(dead_code)]
    pub fn get_or_create(&mut self, graphql_name: &str, _language: &str) -> String {
        if let Some(cached) = self.cache.get(graphql_name) {
            return cached.clone();
        }
        let converted = to_pascal_case(graphql_name);
        self.cache.insert(graphql_name.to_string(), converted.clone());
        converted
    }
}

impl Default for TypeNameCache {
    fn default() -> Self {
        Self::new()
    }
}
