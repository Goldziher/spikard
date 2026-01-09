#![cfg(feature = "di")]

use http::Request;
use spikard_core::RequestData;
use spikard_core::di::{Dependency, DependencyError, ResolvedDependencies};
use std::any::Any;
use std::sync::Arc;

struct DummyDependency;

impl Dependency for DummyDependency {
    fn resolve(
        &self,
        _request: &Request<()>,
        _request_data: &RequestData,
        _resolved: &ResolvedDependencies,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Arc<dyn Any + Send + Sync>, DependencyError>> + Send + '_>,
    > {
        Box::pin(async move { Ok(Arc::new(()) as Arc<dyn Any + Send + Sync>) })
    }

    fn key(&self) -> &'static str {
        "dummy"
    }

    fn depends_on(&self) -> Vec<String> {
        Vec::new()
    }
}

#[test]
fn dependency_defaults_are_false() {
    let dep = DummyDependency;
    assert!(!dep.cacheable());
    assert!(!dep.singleton());
}
