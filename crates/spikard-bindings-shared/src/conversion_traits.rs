//! Language-agnostic conversion interfaces

use std::any::Any;

/// Trait for converting from language-specific types to Rust types
pub trait FromLanguage: Sized {
    /// The error type for conversion failures
    type Error: std::fmt::Display;

    /// Convert from a language-specific value
    ///
    /// # Errors
    ///
    /// Returns an error if the value cannot be converted to the expected type.
    fn from_any(value: &(dyn Any + Send + Sync)) -> Result<Self, Self::Error>;
}

/// Trait for converting from Rust types to language-specific types
pub trait ToLanguage {
    /// The error type for conversion failures
    type Error: std::fmt::Display;

    /// Convert to a language-specific value
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails.
    fn to_any(&self) -> Result<Box<dyn Any + Send + Sync>, Self::Error>;
}

/// Trait for converting to/from JSON values
pub trait JsonConvertible: Sized {
    /// The error type for conversion failures
    type Error: std::fmt::Display;

    /// Convert from a JSON value
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON value is not valid for the target type.
    fn from_json(value: serde_json::Value) -> Result<Self, Self::Error>;

    /// Convert to a JSON value
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails.
    fn to_json(&self) -> Result<serde_json::Value, Self::Error>;
}

/// Default JSON conversion error
#[derive(Debug)]
pub struct JsonConversionError(pub String);

impl std::fmt::Display for JsonConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSON conversion error: {}", self.0)
    }
}

/// Default implementation for JSON values
impl JsonConvertible for serde_json::Value {
    type Error = JsonConversionError;

    fn from_json(value: serde_json::Value) -> Result<Self, Self::Error> {
        Ok(value)
    }

    fn to_json(&self) -> Result<serde_json::Value, Self::Error> {
        Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Debug)]
    struct TestType {
        value: i32,
    }

    impl FromLanguage for TestType {
        type Error = String;

        fn from_any(value: &(dyn Any + Send + Sync)) -> Result<Self, Self::Error> {
            value
                .downcast_ref::<i32>()
                .map(|&v| Self { value: v })
                .ok_or_else(|| "Invalid type".to_string())
        }
    }

    impl ToLanguage for TestType {
        type Error = String;

        fn to_any(&self) -> Result<Box<dyn Any + Send + Sync>, Self::Error> {
            Ok(Box::new(self.value))
        }
    }

    #[test]
    fn test_json_conversion_error_display() {
        let err = JsonConversionError("test error".to_string());
        assert_eq!(err.to_string(), "JSON conversion error: test error");
    }

    #[test]
    fn test_json_value_from_json() {
        let input = json!({ "key": "value" });
        let result = serde_json::Value::from_json(input.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_json_value_to_json() {
        let input = json!({ "key": "value" });
        let result = serde_json::Value::to_json(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), input);
    }

    #[test]
    fn test_json_value_roundtrip() {
        let original = json!({
            "string": "test",
            "number": 42,
            "float": 3.2,
            "bool": true,
            "null": null,
            "array": [1, 2, 3],
            "object": { "nested": "value" }
        });

        let converted = serde_json::Value::from_json(original.clone()).unwrap();
        let back = converted.to_json().unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn test_from_language_trait() {
        let any_value: Box<dyn Any + Send + Sync> = Box::new(42i32);
        let result = TestType::from_any(&*any_value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, 42);
    }

    #[test]
    fn test_from_language_wrong_type() {
        let any_value: Box<dyn Any + Send + Sync> = Box::new("string");
        let result = TestType::from_any(&*any_value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid type");
    }

    #[test]
    fn test_to_language_trait() {
        let test_obj = TestType { value: 123 };
        let result = test_obj.to_any();
        assert!(result.is_ok());

        let any_box = result.unwrap();
        let downcast = any_box.downcast_ref::<i32>();
        assert_eq!(*downcast.unwrap(), 123);
    }

    #[test]
    fn test_json_null_conversion() {
        let null_value = serde_json::Value::Null;
        let result = serde_json::Value::from_json(null_value);
        assert!(result.is_ok());
        assert!(result.unwrap().is_null());
    }

    #[test]
    fn test_json_array_conversion() {
        let array = json!([1, 2, 3, 4, 5]);
        let result = serde_json::Value::from_json(array);
        assert!(result.is_ok());
        let converted = result.unwrap();
        assert!(converted.is_array());
        assert_eq!(converted.as_array().unwrap().len(), 5);
    }

    #[test]
    fn test_json_nested_object_conversion() {
        let nested = json!({
            "level1": {
                "level2": {
                    "level3": {
                        "value": "deep"
                    }
                }
            }
        });

        let result = serde_json::Value::from_json(nested);
        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted["level1"]["level2"]["level3"]["value"], "deep");
    }

    #[test]
    fn test_json_conversion_error_description() {
        let err = JsonConversionError("Custom message".to_string());
        assert!(err.to_string().contains("Custom message"));
    }
}
