//! Case conversion utilities for codegen.
//!
//! Provides unified case conversion functions used across all code generators
//! (Python, Ruby, PHP, TypeScript, Rust). Handles edge cases like consecutive
//! uppercase letters (acronyms) and preserves leading/trailing underscores.

/// Convert string to snake_case.
///
/// Converts camelCase, PascalCase, and other formats to snake_case by inserting
/// underscores before uppercase letters and converting them to lowercase.
///
/// Edge cases:
/// - Consecutive uppercase letters (acronyms) like "HTTPServer" → "http_server"
/// - Leading/trailing underscores are preserved
/// - Already snake_case strings pass through unchanged
///
/// # Examples
///
/// ```
/// use spikard_cli::codegen::common::case_conversion::to_snake_case;
/// assert_eq!(to_snake_case("user"), "user");
/// assert_eq!(to_snake_case("getUser"), "get_user");
/// assert_eq!(to_snake_case("createUserProfile"), "create_user_profile");
/// assert_eq!(to_snake_case("HTTPServer"), "http_server");
/// assert_eq!(to_snake_case("GraphQLType"), "graph_ql_type"); // Splits on each uppercase
/// assert_eq!(to_snake_case("_id"), "_id");
/// assert_eq!(to_snake_case("id_"), "id_");
/// ```
pub fn to_snake_case(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            // Check if we should add an underscore before this uppercase letter
            let should_add_underscore = if i == 0 {
                // Never add underscore at start, unless original starts with it
                false
            } else if result.ends_with('_') {
                // Already have underscore, don't duplicate
                false
            } else {
                // Add underscore if:
                // 1. Previous char is lowercase (transition from lower to upper)
                // 2. Previous char is digit (transition from digit to upper)
                // 3. OR this is end of acronym (current is upper, next is lower)
                let prev_is_lower = chars[i - 1].is_lowercase();
                let prev_is_digit = chars[i - 1].is_numeric();
                let next_is_lower = (i + 1 < chars.len()) && chars[i + 1].is_lowercase();

                prev_is_lower || prev_is_digit || (i > 0 && chars[i - 1].is_uppercase() && next_is_lower)
            };

            if should_add_underscore {
                result.push('_');
            }
            result.push_str(&ch.to_lowercase().to_string());
        } else {
            result.push(ch);
        }
    }

    result
}

/// Convert string to camelCase.
///
/// Converts snake_case and other formats to camelCase by capitalizing the first
/// letter of each word (except the first word) and removing separators.
///
/// Edge cases:
/// - First word stays lowercase
/// - Consecutive separators are treated as single separator
/// - Leading/trailing separators produce leading/trailing underscores
///
/// # Examples
///
/// ```
/// use spikard_cli::codegen::common::case_conversion::to_camel_case;
/// assert_eq!(to_camel_case("user"), "user");
/// assert_eq!(to_camel_case("get_user"), "getUser");
/// assert_eq!(to_camel_case("create_user_profile"), "createUserProfile");
/// assert_eq!(to_camel_case("_id"), "_id");
/// assert_eq!(to_camel_case("id_"), "id_");
/// ```
pub fn to_camel_case(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let parts: Vec<&str> = s.split('_').collect();
    if parts.is_empty() {
        return String::new();
    }

    // Preserve leading and trailing underscores
    let has_leading_underscore = s.starts_with('_');
    let has_trailing_underscore = s.ends_with('_');

    let mut result = if has_leading_underscore {
        String::from("_")
    } else {
        String::new()
    };

    // Collect non-empty parts
    let non_empty_parts: Vec<&str> = parts.iter().filter(|p| !p.is_empty()).copied().collect();

    if non_empty_parts.is_empty() {
        // If all parts were empty (e.g., "___"), preserve underscores
        if has_trailing_underscore {
            result.push('_');
        }
        return result;
    }

    // Add first part as-is (lowercase)
    result.push_str(non_empty_parts[0]);

    // Capitalize subsequent parts
    for part in &non_empty_parts[1..] {
        if let Some(first_char) = part.chars().next() {
            result.push_str(&first_char.to_uppercase().to_string());
            result.push_str(&part[first_char.len_utf8()..]);
        }
    }

    if has_trailing_underscore {
        result.push('_');
    }

    result
}

/// Convert string to PascalCase.
///
/// Converts snake_case and other formats to PascalCase by capitalizing the first
/// letter of every word and removing separators. First word is also capitalized.
///
/// Edge cases:
/// - All words are capitalized (unlike camelCase)
/// - Non-alphanumeric characters are treated as separators
/// - Leading/trailing separators are removed
///
/// # Examples
///
/// ```
/// use spikard_cli::codegen::common::case_conversion::to_pascal_case;
/// assert_eq!(to_pascal_case("user"), "User");
/// assert_eq!(to_pascal_case("get_user"), "GetUser");
/// assert_eq!(to_pascal_case("create_user_profile"), "CreateUserProfile");
/// assert_eq!(to_pascal_case("http_server"), "HttpServer");
/// assert_eq!(to_pascal_case("graphql-type"), "GraphqlType");
/// ```
pub fn to_pascal_case(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    // Split on non-alphanumeric characters
    let parts: Vec<&str> = s.split(|c: char| !c.is_alphanumeric()).collect();

    parts
        .into_iter()
        .filter(|p| !p.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut result = first.to_uppercase().collect::<String>();
                    result.push_str(chars.as_str());
                    result
                }
            }
        })
        .collect()
}

/// Convert string to kebab-case.
///
/// Converts camelCase, PascalCase, and other formats to kebab-case by inserting
/// hyphens before uppercase letters and converting them to lowercase.
///
/// Edge cases:
/// - Consecutive uppercase letters (acronyms) like "HTTPServer" → "http-server"
/// - Leading/trailing hyphens are removed
/// - Already kebab-case strings pass through unchanged
///
/// # Examples
///
/// ```
/// use spikard_cli::codegen::common::case_conversion::to_kebab_case;
/// assert_eq!(to_kebab_case("user"), "user");
/// assert_eq!(to_kebab_case("getUser"), "get-user");
/// assert_eq!(to_kebab_case("createUserProfile"), "create-user-profile");
/// assert_eq!(to_kebab_case("HTTPServer"), "http-server");
/// assert_eq!(to_kebab_case("GraphQLType"), "graph-ql-type"); // Splits on each uppercase
/// ```
pub fn to_kebab_case(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            // Check if we should add a hyphen before this uppercase letter
            let should_add_hyphen = if i == 0 {
                false
            } else if result.ends_with('-') {
                false
            } else {
                let prev_is_lower = chars[i - 1].is_lowercase();
                let prev_is_digit = chars[i - 1].is_numeric();
                let next_is_lower = (i + 1 < chars.len()) && chars[i + 1].is_lowercase();

                prev_is_lower || prev_is_digit || (i > 0 && chars[i - 1].is_uppercase() && next_is_lower)
            };

            if should_add_hyphen {
                result.push('-');
            }
            result.push_str(&ch.to_lowercase().to_string());
        } else if ch == '_' {
            // Convert underscores to hyphens
            if !result.ends_with('-') {
                result.push('-');
            }
        } else {
            result.push(ch);
        }
    }

    result.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============================================================================
    // to_snake_case tests
    // ============================================================================

    #[test]
    fn test_to_snake_case_simple() {
        assert_eq!(to_snake_case("user"), "user");
        assert_eq!(to_snake_case("name"), "name");
        assert_eq!(to_snake_case("id"), "id");
    }

    #[test]
    fn test_to_snake_case_camel_case() {
        assert_eq!(to_snake_case("getUser"), "get_user");
        assert_eq!(to_snake_case("userName"), "user_name");
        assert_eq!(to_snake_case("userId"), "user_id");
    }

    #[test]
    fn test_to_snake_case_pascal_case() {
        assert_eq!(to_snake_case("GetUser"), "get_user");
        assert_eq!(to_snake_case("UserName"), "user_name");
        assert_eq!(to_snake_case("CreateUserProfile"), "create_user_profile");
    }

    #[test]
    fn test_to_snake_case_acronyms() {
        assert_eq!(to_snake_case("HTTPServer"), "http_server");
        assert_eq!(to_snake_case("GraphQLType"), "graph_ql_type"); // QL is separate acronym
        assert_eq!(to_snake_case("XMLHttpRequest"), "xml_http_request");
        assert_eq!(to_snake_case("IOError"), "io_error");
        assert_eq!(to_snake_case("URLPath"), "url_path");
    }

    #[test]
    fn test_to_snake_case_consecutive_caps() {
        assert_eq!(to_snake_case("ID"), "id");
        assert_eq!(to_snake_case("HTTPSConnection"), "https_connection");
        assert_eq!(to_snake_case("JSONData"), "json_data");
    }

    #[test]
    fn test_to_snake_case_leading_underscore() {
        assert_eq!(to_snake_case("_id"), "_id");
        assert_eq!(to_snake_case("_private"), "_private");
        assert_eq!(to_snake_case("_getUser"), "_get_user");
    }

    #[test]
    fn test_to_snake_case_trailing_underscore() {
        assert_eq!(to_snake_case("id_"), "id_");
        assert_eq!(to_snake_case("name_"), "name_");
        assert_eq!(to_snake_case("getUserName_"), "get_user_name_");
    }

    #[test]
    fn test_to_snake_case_already_snake_case() {
        assert_eq!(to_snake_case("get_user"), "get_user");
        assert_eq!(to_snake_case("create_user_profile"), "create_user_profile");
        assert_eq!(to_snake_case("http_server"), "http_server");
    }

    #[test]
    fn test_to_snake_case_mixed_separators() {
        assert_eq!(to_snake_case("getUser_Name"), "get_user_name");
        assert_eq!(to_snake_case("_private_field_"), "_private_field_");
    }

    #[test]
    fn test_to_snake_case_numbers() {
        assert_eq!(to_snake_case("user123"), "user123");
        assert_eq!(to_snake_case("getUser123"), "get_user123");
        assert_eq!(to_snake_case("User123Name"), "user123_name");
    }

    #[test]
    fn test_to_snake_case_empty() {
        assert_eq!(to_snake_case(""), "");
    }

    #[test]
    fn test_to_snake_case_single_char() {
        assert_eq!(to_snake_case("a"), "a");
        assert_eq!(to_snake_case("A"), "a");
        assert_eq!(to_snake_case("_"), "_");
    }

    // ============================================================================
    // to_camel_case tests
    // ============================================================================

    #[test]
    fn test_to_camel_case_simple() {
        assert_eq!(to_camel_case("user"), "user");
        assert_eq!(to_camel_case("name"), "name");
        assert_eq!(to_camel_case("id"), "id");
    }

    #[test]
    fn test_to_camel_case_snake_case() {
        assert_eq!(to_camel_case("get_user"), "getUser");
        assert_eq!(to_camel_case("user_name"), "userName");
        assert_eq!(to_camel_case("user_id"), "userId");
    }

    #[test]
    fn test_to_camel_case_multiple_words() {
        assert_eq!(to_camel_case("create_user_profile"), "createUserProfile");
        assert_eq!(to_camel_case("get_user_by_id"), "getUserById");
        assert_eq!(to_camel_case("http_server_config"), "httpServerConfig");
    }

    #[test]
    fn test_to_camel_case_pascal_case_input() {
        assert_eq!(to_camel_case("GetUser"), "GetUser"); // First word not lowercase
        assert_eq!(to_camel_case("UserName"), "UserName");
    }

    #[test]
    fn test_to_camel_case_leading_underscore() {
        assert_eq!(to_camel_case("_id"), "_id");
        assert_eq!(to_camel_case("_get_user"), "_getUser");
        assert_eq!(to_camel_case("_private"), "_private");
    }

    #[test]
    fn test_to_camel_case_trailing_underscore() {
        assert_eq!(to_camel_case("id_"), "id_");
        assert_eq!(to_camel_case("get_user_"), "getUser_");
    }

    #[test]
    fn test_to_camel_case_consecutive_separators() {
        assert_eq!(to_camel_case("get__user"), "getUser");
        assert_eq!(to_camel_case("user___name"), "userName");
    }

    #[test]
    fn test_to_camel_case_numbers() {
        assert_eq!(to_camel_case("user_123"), "user123");
        assert_eq!(to_camel_case("get_user_123"), "getUser123");
    }

    #[test]
    fn test_to_camel_case_empty() {
        assert_eq!(to_camel_case(""), "");
    }

    #[test]
    fn test_to_camel_case_single_char() {
        assert_eq!(to_camel_case("a"), "a");
        assert_eq!(to_camel_case("_"), "__"); // Underscore alone treated as lead+trail underscore
    }

    // ============================================================================
    // to_pascal_case tests
    // ============================================================================

    #[test]
    fn test_to_pascal_case_simple() {
        assert_eq!(to_pascal_case("user"), "User");
        assert_eq!(to_pascal_case("name"), "Name");
        assert_eq!(to_pascal_case("id"), "Id");
    }

    #[test]
    fn test_to_pascal_case_snake_case() {
        assert_eq!(to_pascal_case("get_user"), "GetUser");
        assert_eq!(to_pascal_case("user_name"), "UserName");
        assert_eq!(to_pascal_case("create_user_profile"), "CreateUserProfile");
    }

    #[test]
    fn test_to_pascal_case_camel_case() {
        assert_eq!(to_pascal_case("getUser"), "GetUser");
        assert_eq!(to_pascal_case("userName"), "UserName");
        assert_eq!(to_pascal_case("createUserProfile"), "CreateUserProfile");
    }

    #[test]
    fn test_to_pascal_case_kebab_case() {
        assert_eq!(to_pascal_case("get-user"), "GetUser");
        assert_eq!(to_pascal_case("user-name"), "UserName");
        assert_eq!(to_pascal_case("http-server"), "HttpServer");
    }

    #[test]
    fn test_to_pascal_case_mixed_separators() {
        assert_eq!(to_pascal_case("get_user-name"), "GetUserName");
        assert_eq!(to_pascal_case("user-name_id"), "UserNameId");
    }

    #[test]
    fn test_to_pascal_case_numbers() {
        assert_eq!(to_pascal_case("user_123"), "User123");
        assert_eq!(to_pascal_case("get_user_123"), "GetUser123");
        assert_eq!(to_pascal_case("user123name"), "User123name");
    }

    #[test]
    fn test_to_pascal_case_leading_trailing_separators() {
        assert_eq!(to_pascal_case("_user"), "User"); // Leading separator removed
        assert_eq!(to_pascal_case("user_"), "User"); // Trailing separator removed
        assert_eq!(to_pascal_case("_user_"), "User");
    }

    #[test]
    fn test_to_pascal_case_empty() {
        assert_eq!(to_pascal_case(""), "");
    }

    #[test]
    fn test_to_pascal_case_single_char() {
        assert_eq!(to_pascal_case("a"), "A");
        assert_eq!(to_pascal_case("_"), "");
    }

    #[test]
    fn test_to_pascal_case_already_pascal() {
        assert_eq!(to_pascal_case("GetUser"), "GetUser");
        assert_eq!(to_pascal_case("UserName"), "UserName");
        assert_eq!(to_pascal_case("CreateUserProfile"), "CreateUserProfile");
    }

    // ============================================================================
    // to_kebab_case tests
    // ============================================================================

    #[test]
    fn test_to_kebab_case_simple() {
        assert_eq!(to_kebab_case("user"), "user");
        assert_eq!(to_kebab_case("name"), "name");
        assert_eq!(to_kebab_case("id"), "id");
    }

    #[test]
    fn test_to_kebab_case_camel_case() {
        assert_eq!(to_kebab_case("getUser"), "get-user");
        assert_eq!(to_kebab_case("userName"), "user-name");
        assert_eq!(to_kebab_case("createUserProfile"), "create-user-profile");
    }

    #[test]
    fn test_to_kebab_case_pascal_case() {
        assert_eq!(to_kebab_case("GetUser"), "get-user");
        assert_eq!(to_kebab_case("UserName"), "user-name");
        assert_eq!(to_kebab_case("CreateUserProfile"), "create-user-profile");
    }

    #[test]
    fn test_to_kebab_case_snake_case() {
        assert_eq!(to_kebab_case("get_user"), "get-user");
        assert_eq!(to_kebab_case("user_name"), "user-name");
        assert_eq!(to_kebab_case("http_server"), "http-server");
    }

    #[test]
    fn test_to_kebab_case_acronyms() {
        assert_eq!(to_kebab_case("HTTPServer"), "http-server");
        assert_eq!(to_kebab_case("GraphQLType"), "graph-ql-type"); // QL is separate acronym
        assert_eq!(to_kebab_case("XMLHttpRequest"), "xml-http-request");
    }

    #[test]
    fn test_to_kebab_case_already_kebab_case() {
        assert_eq!(to_kebab_case("get-user"), "get-user");
        assert_eq!(to_kebab_case("user-name"), "user-name");
        assert_eq!(to_kebab_case("http-server"), "http-server");
    }

    #[test]
    fn test_to_kebab_case_numbers() {
        assert_eq!(to_kebab_case("user123"), "user123");
        assert_eq!(to_kebab_case("getUser123"), "get-user123");
        assert_eq!(to_kebab_case("User123Name"), "user123-name");
    }

    #[test]
    fn test_to_kebab_case_leading_trailing_hyphens() {
        // Leading/trailing hyphens should be trimmed
        assert_eq!(to_kebab_case("getUser-"), "get-user");
        assert_eq!(to_kebab_case("-getUser"), "get-user");
        assert_eq!(to_kebab_case("-getUser-"), "get-user");
    }

    #[test]
    fn test_to_kebab_case_empty() {
        assert_eq!(to_kebab_case(""), "");
    }

    #[test]
    fn test_to_kebab_case_single_char() {
        assert_eq!(to_kebab_case("a"), "a");
        assert_eq!(to_kebab_case("A"), "a");
    }

    // ============================================================================
    // Cross-function consistency tests
    // ============================================================================

    #[test]
    fn test_round_trip_snake_to_camel_to_snake() {
        let original = "get_user_profile";
        let camel = to_camel_case(original);
        let back = to_snake_case(&camel);
        assert_eq!(back, original);
    }

    #[test]
    fn test_round_trip_snake_to_pascal_to_snake() {
        let original = "get_user_profile";
        let pascal = to_pascal_case(original);
        let back = to_snake_case(&pascal);
        assert_eq!(back, original);
    }

    #[test]
    fn test_acronym_consistency() {
        // All converters should handle acronyms consistently
        assert_eq!(to_snake_case("HTTPServer"), "http_server");
        assert_eq!(to_camel_case("http_server"), "httpServer");
        assert_eq!(to_pascal_case("http_server"), "HttpServer");
        assert_eq!(to_kebab_case("HTTPServer"), "http-server");
    }

    #[test]
    fn test_graphql_type_consistency() {
        let graphql = "GraphQLType";
        assert_eq!(to_snake_case(graphql), "graph_ql_type"); // QL is separate acronym
        assert_eq!(to_camel_case("graph_ql_type"), "graphQlType");
        assert_eq!(to_pascal_case("graph_ql_type"), "GraphQlType");
        assert_eq!(to_kebab_case(graphql), "graph-ql-type");
    }

    #[test]
    fn test_real_world_field_names() {
        // Common field names from APIs
        assert_eq!(to_snake_case("userId"), "user_id");
        assert_eq!(to_snake_case("firstName"), "first_name");
        assert_eq!(to_snake_case("lastName"), "last_name");
        assert_eq!(to_snake_case("createdAt"), "created_at");
        assert_eq!(to_snake_case("updatedAt"), "updated_at");

        assert_eq!(to_pascal_case("user_id"), "UserId");
        assert_eq!(to_pascal_case("first_name"), "FirstName");
        assert_eq!(to_pascal_case("created_at"), "CreatedAt");
    }

    #[test]
    fn test_edge_case_empty_parts() {
        // These shouldn't panic
        assert_eq!(to_camel_case("__"), "__");
        assert_eq!(to_pascal_case("__"), "");
        assert_eq!(to_snake_case("__"), "__");
        assert_eq!(to_kebab_case("__"), "");
    }
}
