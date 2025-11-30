//! Shared configuration for TypeScript-based e2e targets.

#[derive(Debug, Clone, Copy)]
pub struct TypeScriptTarget {
    /// The npm package path used when importing the bindings (e.g. `spikard` or `spikard-wasm/node`).
    pub binding_package: &'static str,
    /// The npm package that must appear in devDependencies.
    pub dependency_package: &'static str,
    /// Package name for the generated e2e project (e.g. `spikard-e2e-node`).
    pub e2e_package_name: &'static str,
}

pub const NODE_TARGET: TypeScriptTarget = TypeScriptTarget {
    binding_package: "spikard",
    dependency_package: "spikard",
    e2e_package_name: "spikard-e2e-node",
};

pub const WASM_TARGET: TypeScriptTarget = TypeScriptTarget {
    binding_package: "spikard-wasm/node",
    dependency_package: "spikard-wasm",
    e2e_package_name: "spikard-e2e-wasm",
};
