//! PHP Method enum registration.
//!
//! Since ext-php-rs doesn't natively support backed enum registration,
//! this module exposes the HTTP Method as a PHP class with constants.
//! The constants match the backing values: GET = "Get", POST = "Post", etc.

use ext_php_rs::prelude::*;

/// Method HTTP verb enum exposed as PHP class constants.
/// Implements all standard HTTP methods as class constants.
///
/// Usage in PHP:
/// ```php
/// $method = constant("Spikard\\Php\\Method::GET");  // Returns "Get"
/// $method = \Spikard\Php\Method::GET;               // Also works
/// ```
#[php_class]
#[php(name = "Spikard\\Php\\Method")]
pub struct MethodEnum;

#[php_impl]
impl MethodEnum {
    /// GET HTTP method
    #[php(const, name = "GET")]
    pub const GET: &str = "Get";

    /// POST HTTP method
    #[php(const, name = "POST")]
    pub const POST: &str = "Post";

    /// PUT HTTP method
    #[php(const, name = "PUT")]
    pub const PUT: &str = "Put";

    /// PATCH HTTP method
    #[php(const, name = "PATCH")]
    pub const PATCH: &str = "Patch";

    /// DELETE HTTP method
    #[php(const, name = "DELETE")]
    pub const DELETE: &str = "Delete";

    /// HEAD HTTP method
    #[php(const, name = "HEAD")]
    pub const HEAD: &str = "Head";

    /// OPTIONS HTTP method
    #[php(const, name = "OPTIONS")]
    pub const OPTIONS: &str = "Options";

    /// CONNECT HTTP method
    #[php(const, name = "CONNECT")]
    pub const CONNECT: &str = "Connect";

    /// TRACE HTTP method
    #[php(const, name = "TRACE")]
    pub const TRACE: &str = "Trace";
}
