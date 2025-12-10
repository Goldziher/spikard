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

    // Check for reserved "rpc." prefix
    if name.starts_with("rpc.") {
        return Err("JSON-RPC method name cannot start with reserved prefix 'rpc.'".to_string());
    }

    // Validate characters: alphanumeric, dots, underscores, and hyphens
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

    // Cannot start with a dot or hyphen
    if let Some(first_char) = name.chars().next() {
        if first_char == '.' || first_char == '-' {
            return Err(format!(
                "Invalid JSON-RPC method name '{}'. Method name cannot start with '.' or '-'",
                name
            ));
        }
    }

    // Cannot end with a dot
    if name.ends_with('.') {
        return Err(format!(
            "Invalid JSON-RPC method name '{}'. Method name cannot end with '.'",
            name
        ));
    }

    // Ensure no consecutive dots
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
    fn test_invalid_method_names() {
        assert!(validate_jsonrpc_method_name("").is_err()); // empty
        assert!(validate_jsonrpc_method_name("rpc.test").is_err()); // reserved prefix
        assert!(validate_jsonrpc_method_name("user.create!").is_err()); // invalid char
        assert!(validate_jsonrpc_method_name("user create").is_err()); // space
        assert!(validate_jsonrpc_method_name("user@domain").is_err()); // @ symbol
        assert!(validate_jsonrpc_method_name(".user").is_err()); // starts with dot
        assert!(validate_jsonrpc_method_name("-user").is_err()); // starts with hyphen
        assert!(validate_jsonrpc_method_name("user.").is_err()); // ends with dot
        assert!(validate_jsonrpc_method_name("user..create").is_err()); // consecutive dots
    }
}
