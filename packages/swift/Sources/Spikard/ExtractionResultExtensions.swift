import RustBridge

// MARK: - Property-access ergonomics for e2e tests
//
// This file provides computed-property aliases for methods on swift-bridge-generated types,
// allowing callers to write `result.mimeType` rather than `result.mimeType()`.
// These extensions are especially useful in e2e test assertions where the alef
// fixture generator emits property-access syntax.
//
// Although these are primarily for test convenience, they are part of the public API
// and can be used in production code for more ergonomic access to extraction results.

extension RustBridge.UploadFileRef {
    /// Computed-property alias for `read_to_string()` method.
    public var read_to_string: String {
        self.read_to_string().toString()
    }

    /// Computed-property alias for `content_type_or_default()` method.
    public var content_type_or_default: String {
        self.content_type_or_default().toString()
    }
}

// UploadFileRefMut and UploadFile inherit the extensions automatically

extension RustBridge.CorsConfigRef {
    /// Computed-property alias for `allowed_methods_joined()` method.
    public var allowed_methods_joined: String {
        self.allowed_methods_joined().toString()
    }

    /// Computed-property alias for `allowed_headers_joined()` method.
    public var allowed_headers_joined: String {
        self.allowed_headers_joined().toString()
    }
}

// CorsConfigRefMut and CorsConfig inherit the extensions automatically

extension RustBridge.ProblemDetailsRef {
    /// Computed-property alias for `to_json()` method.
    public var to_json: String {
        self.to_json().toString()
    }

    /// Computed-property alias for `to_json_pretty()` method.
    public var to_json_pretty: String {
        self.to_json_pretty().toString()
    }
}

// ProblemDetailsRefMut and ProblemDetails inherit the extensions automatically

extension RustBridge.ResponseSnapshotRef {
    /// Computed-property alias for `text()` method.
    public var text: String {
        self.text().toString()
    }
}

// ResponseSnapshotRefMut and ResponseSnapshot inherit the extensions automatically
