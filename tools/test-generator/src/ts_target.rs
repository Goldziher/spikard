//! Shared configuration for TypeScript-based e2e targets.

/// Runtime environment for TypeScript targets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Runtime {
    /// Node.js runtime with native NAPI bindings
    Node,
    /// Deno runtime with WASM bindings
    Deno,
    /// Cloudflare Workers runtime with WASM bindings
    CloudflareWorkers,
    /// Node.js runtime with WASM bindings (for comparison testing)
    NodeWasm,
}

#[derive(Debug, Clone, Copy)]
pub struct TypeScriptTarget {
    /// The npm package path used when importing the bindings (e.g. `spikard` or `spikard-wasm/node`).
    pub binding_package: &'static str,
    /// The npm package that must appear in devDependencies.
    pub dependency_package: &'static str,
    /// Package name for the generated e2e project (e.g. `spikard-e2e-node`).
    pub e2e_package_name: &'static str,
    /// Runtime environment
    pub runtime: Runtime,
}

pub const NODE_TARGET: TypeScriptTarget = TypeScriptTarget {
    binding_package: "spikard",
    dependency_package: "spikard",
    e2e_package_name: "spikard-e2e-node",
    runtime: Runtime::Node,
};

pub const WASM_TARGET: TypeScriptTarget = TypeScriptTarget {
    binding_package: "spikard-wasm/node",
    dependency_package: "spikard-wasm",
    e2e_package_name: "spikard-e2e-wasm",
    runtime: Runtime::NodeWasm,
};

pub const DENO_TARGET: TypeScriptTarget = TypeScriptTarget {
    binding_package: "@spikard/wasm",
    dependency_package: "@spikard/wasm",
    e2e_package_name: "spikard-e2e-deno",
    runtime: Runtime::Deno,
};

pub const CLOUDFLARE_TARGET: TypeScriptTarget = TypeScriptTarget {
    binding_package: "@spikard/wasm",
    dependency_package: "@spikard/wasm",
    e2e_package_name: "spikard-e2e-cloudflare",
    runtime: Runtime::CloudflareWorkers,
};
