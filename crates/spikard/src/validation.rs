//! JSON-RPC and request validation utilities.

/// Validates a JSON-RPC method name according to the JSON-RPC 2.0 specification.
///
/// Method names must be non-empty strings containing only alphanumeric characters,
/// dots (.), underscores (_), and hyphens (-). Method names starting with "rpc." are reserved.
///
/// # Arguments
///
/// * `name` - The method name to validate
///
/// # Returns
///
/// * `Ok(())` if the method name is valid
/// * `Err(String)` with a descriptive error message if invalid
///
/// # Examples
///
/// ```
/// use spikard::validation::validate_jsonrpc_method_name;
///
/// assert!(validate_jsonrpc_method_name("user.create").is_ok());
/// assert!(validate_jsonrpc_method_name("math.add").is_ok());
/// assert!(validate_jsonrpc_method_name("get_user_by_id").is_ok());
/// assert!(validate_jsonrpc_method_name("myMethod").is_ok());
///
/// // Invalid cases
/// assert!(validate_jsonrpc_method_name("").is_err());  // empty
/// assert!(validate_jsonrpc_method_name("user.create!").is_err());  // invalid char
/// assert!(validate_jsonrpc_method_name("user create").is_err());  // space
/// ```
pub fn validate_jsonrpc_method_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("JSON-RPC method name cannot be empty".to_string());
    }

    if name.starts_with("rpc.") {
        return Err("JSON-RPC method name cannot start with reserved prefix 'rpc.'".to_string());
    }

    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '-')
    {
        return Err(format!(
            "Invalid JSON-RPC method name '{}'. Method names must contain only alphanumeric \
             characters, dots (.), underscores (_), and hyphens (-)",
            name
        ));
    }

    if let Some(first_char) = name.chars().next()
        && (first_char == '.' || first_char == '-')
    {
        return Err(format!(
            "Invalid JSON-RPC method name '{}'. Method name cannot start with '.' or '-'",
            name
        ));
    }

    if name.ends_with('.') {
        return Err(format!(
            "Invalid JSON-RPC method name '{}'. Method name cannot end with '.'",
            name
        ));
    }

    if name.contains("..") {
        return Err(format!(
            "Invalid JSON-RPC method name '{}'. Method name cannot contain consecutive dots '..'",
            name
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_method_names() {
        assert!(validate_jsonrpc_method_name("user.create").is_ok());
        assert!(validate_jsonrpc_method_name("math.add").is_ok());
        assert!(validate_jsonrpc_method_name("get_user_by_id").is_ok());
        assert!(validate_jsonrpc_method_name("myMethod").is_ok());
        assert!(validate_jsonrpc_method_name("get").is_ok());
        assert!(validate_jsonrpc_method_name("User.Create.New").is_ok());
        assert!(validate_jsonrpc_method_name("user_create").is_ok());
        assert!(validate_jsonrpc_method_name("user-create").is_ok());
        assert!(validate_jsonrpc_method_name("a").is_ok());
        assert!(validate_jsonrpc_method_name("a1b2c3").is_ok());
    }

    #[test]
    fn test_valid_single_character() {
        assert!(validate_jsonrpc_method_name("a").is_ok());
        assert!(validate_jsonrpc_method_name("Z").is_ok());
        assert!(validate_jsonrpc_method_name("0").is_ok());
    }

    #[test]
    fn test_valid_multiple_segments() {
        assert!(validate_jsonrpc_method_name("user.profile.get").is_ok());
        assert!(validate_jsonrpc_method_name("api.v1.user.create").is_ok());
        assert!(validate_jsonrpc_method_name("a.b.c.d.e.f").is_ok());
    }

    #[test]
    fn test_valid_with_numbers() {
        assert!(validate_jsonrpc_method_name("v1.get").is_ok());
        assert!(validate_jsonrpc_method_name("method2").is_ok());
        assert!(validate_jsonrpc_method_name("test123abc").is_ok());
        assert!(validate_jsonrpc_method_name("123method").is_ok());
        assert!(validate_jsonrpc_method_name("0.method").is_ok());
    }

    #[test]
    fn test_valid_with_mixed_separators() {
        assert!(validate_jsonrpc_method_name("user_profile.get").is_ok());
        assert!(validate_jsonrpc_method_name("get-user.info").is_ok());
        assert!(validate_jsonrpc_method_name("a_b.c-d").is_ok());
    }

    #[test]
    fn test_reject_empty_string() {
        let result = validate_jsonrpc_method_name("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "JSON-RPC method name cannot be empty");
    }

    #[test]
    fn test_reject_only_whitespace() {
        let result = validate_jsonrpc_method_name("   ");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid JSON-RPC method name"));
    }

    #[test]
    fn test_reject_only_tabs() {
        let result = validate_jsonrpc_method_name("\t\t");
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_rpc_prefix() {
        assert!(validate_jsonrpc_method_name("rpc.test").is_err());
        assert!(validate_jsonrpc_method_name("rpc.").is_err());
        assert!(validate_jsonrpc_method_name("rpc.system").is_err());
    }

    #[test]
    fn test_rpc_prefix_case_sensitive() {
        assert!(validate_jsonrpc_method_name("RPC.test").is_ok());
        assert!(validate_jsonrpc_method_name("Rpc.test").is_ok());
    }

    #[test]
    fn test_reject_null_byte() {
        let result = validate_jsonrpc_method_name("method\0name");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid JSON-RPC method name"));
    }

    #[test]
    fn test_reject_newline() {
        let result = validate_jsonrpc_method_name("method\nname");
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_carriage_return() {
        let result = validate_jsonrpc_method_name("method\rname");
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_tab() {
        let result = validate_jsonrpc_method_name("method\tname");
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_form_feed() {
        let result = validate_jsonrpc_method_name("method\u{000C}name");
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_vertical_tab() {
        let result = validate_jsonrpc_method_name("method\u{000B}name");
        assert!(result.is_err());
    }

    #[test]
    fn test_reject_sql_injection_quotes() {
        assert!(validate_jsonrpc_method_name("'; DROP TABLE users--").is_err());
        assert!(validate_jsonrpc_method_name("'\" OR 1=1").is_err());
        assert!(validate_jsonrpc_method_name("user' OR 'a'='a").is_err());
    }

    #[test]
    fn test_reject_sql_injection_semicolon() {
        assert!(validate_jsonrpc_method_name("method;").is_err());
        assert!(validate_jsonrpc_method_name("select;drop").is_err());
    }

    #[test]
    fn test_reject_sql_comments() {
        assert!(validate_jsonrpc_method_name("method--").is_ok());
        assert!(validate_jsonrpc_method_name("method#comment").is_err());
    }

    #[test]
    fn test_reject_path_traversal_dots() {
        assert!(validate_jsonrpc_method_name("../../../etc/passwd").is_err());
        assert!(validate_jsonrpc_method_name("..\\..\\windows\\system32").is_err());
        assert!(validate_jsonrpc_method_name("../../etc/shadow").is_err());
    }

    #[test]
    fn test_reject_backslash() {
        assert!(validate_jsonrpc_method_name("method\\name").is_err());
        assert!(validate_jsonrpc_method_name("c:\\windows\\temp").is_err());
    }

    #[test]
    fn test_reject_forward_slash() {
        assert!(validate_jsonrpc_method_name("method/name").is_err());
        assert!(validate_jsonrpc_method_name("/etc/passwd").is_err());
    }

    #[test]
    fn test_reject_shell_pipes() {
        assert!(validate_jsonrpc_method_name("method | cat /etc/passwd").is_err());
        assert!(validate_jsonrpc_method_name("cmd1|cmd2").is_err());
    }

    #[test]
    fn test_reject_shell_ampersand() {
        assert!(validate_jsonrpc_method_name("method & rm -rf /").is_err());
        assert!(validate_jsonrpc_method_name("cmd1&cmd2").is_err());
    }

    #[test]
    fn test_reject_shell_backticks() {
        assert!(validate_jsonrpc_method_name("method`whoami`").is_err());
        assert!(validate_jsonrpc_method_name("`id`").is_err());
    }

    #[test]
    fn test_reject_shell_dollar_sign() {
        assert!(validate_jsonrpc_method_name("method$(id)").is_err());
        assert!(validate_jsonrpc_method_name("$HOME").is_err());
        assert!(validate_jsonrpc_method_name("${var}").is_err());
    }

    #[test]
    fn test_reject_shell_parentheses() {
        assert!(validate_jsonrpc_method_name("method(id)").is_err());
        assert!(validate_jsonrpc_method_name("(command)").is_err());
    }

    #[test]
    fn test_reject_special_characters() {
        assert!(validate_jsonrpc_method_name("method!").is_err());
        assert!(validate_jsonrpc_method_name("method@host").is_err());
        assert!(validate_jsonrpc_method_name("method#tag").is_err());
        assert!(validate_jsonrpc_method_name("method$").is_err());
        assert!(validate_jsonrpc_method_name("method%").is_err());
        assert!(validate_jsonrpc_method_name("method^").is_err());
        assert!(validate_jsonrpc_method_name("method&").is_err());
        assert!(validate_jsonrpc_method_name("method*").is_err());
        assert!(validate_jsonrpc_method_name("method=").is_err());
        assert!(validate_jsonrpc_method_name("method+").is_err());
        assert!(validate_jsonrpc_method_name("method[test]").is_err());
        assert!(validate_jsonrpc_method_name("method{test}").is_err());
        assert!(validate_jsonrpc_method_name("method,test").is_err());
        assert!(validate_jsonrpc_method_name("method<test>").is_err());
        assert!(validate_jsonrpc_method_name("method?").is_err());
    }

    #[test]
    fn test_reject_colon() {
        assert!(validate_jsonrpc_method_name("method:name").is_err());
        assert!(validate_jsonrpc_method_name("http://method").is_err());
    }

    #[test]
    fn test_reject_quote_characters() {
        assert!(validate_jsonrpc_method_name("method'name").is_err());
        assert!(validate_jsonrpc_method_name("method\"name").is_err());
        assert!(validate_jsonrpc_method_name("method`name").is_err());
    }

    #[test]
    fn test_reject_starts_with_dot() {
        let result = validate_jsonrpc_method_name(".user");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot start with '.' or '-'"));
    }

    #[test]
    fn test_reject_starts_with_hyphen() {
        let result = validate_jsonrpc_method_name("-user");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot start with '.' or '-'"));
    }

    #[test]
    fn test_reject_starts_with_underscore_is_ok() {
        assert!(validate_jsonrpc_method_name("_method").is_ok());
        assert!(validate_jsonrpc_method_name("__private").is_ok());
    }

    #[test]
    fn test_reject_ends_with_dot() {
        let result = validate_jsonrpc_method_name("user.");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot end with '.'"));
    }

    #[test]
    fn test_reject_ends_with_hyphen_is_ok() {
        assert!(validate_jsonrpc_method_name("user-").is_ok());
    }

    #[test]
    fn test_reject_ends_with_underscore_is_ok() {
        assert!(validate_jsonrpc_method_name("user_").is_ok());
    }

    #[test]
    fn test_reject_consecutive_dots() {
        let result = validate_jsonrpc_method_name("user..create");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("consecutive dots"));
    }

    #[test]
    fn test_reject_triple_dots() {
        assert!(validate_jsonrpc_method_name("user...create").is_err());
        assert!(validate_jsonrpc_method_name("...").is_err());
    }

    #[test]
    fn test_accept_unicode_letters() {
        assert!(validate_jsonrpc_method_name("méthod").is_ok());
        assert!(validate_jsonrpc_method_name("日本語").is_ok());
        assert!(validate_jsonrpc_method_name("用户").is_ok());
    }

    #[test]
    fn test_reject_unicode_bidi_override() {
        let bidi_override = "method\u{202E}name";
        assert!(validate_jsonrpc_method_name(bidi_override).is_err());
    }

    #[test]
    fn test_reject_unicode_zero_width_characters() {
        assert!(validate_jsonrpc_method_name("method\u{200B}name").is_err());
        assert!(validate_jsonrpc_method_name("method\u{200D}name").is_err());
        assert!(validate_jsonrpc_method_name("method\u{200C}name").is_err());
        assert!(validate_jsonrpc_method_name("method\u{00AD}name").is_err());
    }

    #[test]
    fn test_reject_unicode_control_characters() {
        assert!(validate_jsonrpc_method_name("method\u{0080}name").is_err());
        assert!(validate_jsonrpc_method_name("method\u{0085}name").is_err());
        assert!(validate_jsonrpc_method_name("method\u{009F}name").is_err());
    }

    #[test]
    fn test_very_long_method_name_allowed() {
        let long_name = "a".repeat(100);
        assert!(validate_jsonrpc_method_name(&long_name).is_ok());
    }

    #[test]
    fn test_extremely_long_method_name_with_valid_chars() {
        let very_long = format!("{}method", "x.".repeat(50));
        assert!(validate_jsonrpc_method_name(&very_long).is_ok());
    }

    #[test]
    fn test_error_message_contains_invalid_char_context() {
        let result = validate_jsonrpc_method_name("method@host");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Invalid JSON-RPC method name"));
        assert!(err.contains("alphanumeric"));
    }

    #[test]
    fn test_error_message_safe_for_display() {
        let result = validate_jsonrpc_method_name("method@host");
        let err = result.unwrap_err();
        assert!(!err.contains("\0"));
        assert!(!err.contains("\n"));
    }

    #[test]
    fn test_alternating_separators() {
        assert!(validate_jsonrpc_method_name("a.b_c-d.e_f-g").is_ok());
    }

    #[test]
    fn test_numbers_with_separators() {
        assert!(validate_jsonrpc_method_name("v1.get.user.by_id.2").is_ok());
        assert!(validate_jsonrpc_method_name("123.456.789").is_ok());
    }

    #[test]
    fn test_mixed_case_preservation() {
        assert!(validate_jsonrpc_method_name("GetUser").is_ok());
        assert!(validate_jsonrpc_method_name("getUser").is_ok());
        assert!(validate_jsonrpc_method_name("GETUSER").is_ok());
    }

    #[test]
    fn test_rpc_prefix_boundary() {
        assert!(validate_jsonrpc_method_name("rpctest").is_ok());
        assert!(validate_jsonrpc_method_name("rpc").is_ok());
        assert!(validate_jsonrpc_method_name("rpc.").is_err());
        assert!(validate_jsonrpc_method_name("rpc.internal").is_err());
    }
}
