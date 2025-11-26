use ext_php_rs::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

/// PHP-visible Request container (placeholder).
#[php_class(name = "Spikard\\Internal\\Request")]
pub struct PhpRequest {
    method: String,
    path: String,
    body: Value,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    query: HashMap<String, Vec<String>>,
}

#[php_impl]
impl PhpRequest {
    #[constructor]
    pub fn __construct(
        method: String,
        path: String,
        body: Option<Value>,
        headers: Option<HashMap<String, String>>,
        cookies: Option<HashMap<String, String>>,
        query: Option<HashMap<String, Vec<String>>>,
    ) -> Self {
        Self {
            method,
            path,
            body: body.unwrap_or(Value::Null),
            headers: headers.unwrap_or_default(),
            cookies: cookies.unwrap_or_default(),
            query: query.unwrap_or_default(),
        }
    }

    #[getter]
    pub fn method(&self) -> &str {
        &self.method
    }

    #[getter]
    pub fn path(&self) -> &str {
        &self.path
    }

    #[getter]
    pub fn body(&self) -> &Value {
        &self.body
    }

    #[getter]
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    #[getter]
    pub fn cookies(&self) -> &HashMap<String, String> {
        &self.cookies
    }

    #[getter]
    pub fn query(&self) -> &HashMap<String, Vec<String>> {
        &self.query
    }
}
