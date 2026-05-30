#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type Method;
        fn to_string(&self) -> String;
    }

    extern "Rust" {
        #[swift_bridge(swift_name = "methodFromJson")]
        fn method_from_json(json: String) -> Result<Method, String>;
        #[swift_bridge(swift_name = "securitySchemeInfoFromJson")]
        fn security_scheme_info_from_json(json: String) -> Result<String, String>;
    }
}
