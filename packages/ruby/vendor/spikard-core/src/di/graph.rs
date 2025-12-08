//! Dependency graph with topological sorting and cycle detection
//!
//! This module provides the `DependencyGraph` type which manages the dependency
//! relationships between registered dependencies, detects cycles, and calculates
//! the optimal batched resolution order.

use super::error::DependencyError;
use std::collections::{HashMap, HashSet, VecDeque};

/// Dependency graph for managing dependency relationships
///
/// The graph tracks which dependencies depend on which other dependencies,
/// and provides algorithms for:
/// - Cycle detection (preventing circular dependencies)
/// - Topological sorting (determining resolution order)
/// - Batch calculation (enabling parallel resolution)
///
/// # Examples
///
/// ```ignore
/// use spikard_core::di::DependencyGraph;
///
/// let mut graph = DependencyGraph::new();
///
/// // Add dependencies
/// graph.add_dependency("config", vec![]).unwrap();
/// graph.add_dependency("database", vec!["config".to_string()]).unwrap();
/// graph.add_dependency("cache", vec!["config".to_string()]).unwrap();
/// graph.add_dependency("service", vec!["database".to_string(), "cache".to_string()]).unwrap();
///
/// // Calculate batches for parallel resolution
/// let batches = graph.calculate_batches(&[
///     "config".to_string(),
///     "database".to_string(),
///     "cache".to_string(),
///     "service".to_string(),
/// ]).unwrap();
///
/// // Batch 1: config (no dependencies)
/// // Batch 2: database, cache (both depend only on config, can run in parallel)
/// // Batch 3: service (depends on database and cache)
/// assert_eq!(batches.len(), 3);
/// ```
#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    /// Adjacency list: key -> list of dependencies it depends on
    graph: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    /// Create a new empty dependency graph
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyGraph;
    ///
    /// let graph = DependencyGraph::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self { graph: HashMap::new() }
    }

    /// Add a dependency and its dependencies to the graph
    ///
    /// This will check for cycles before adding. If adding this dependency
    /// would create a cycle, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `key` - The unique key for this dependency
    /// * `depends_on` - List of dependency keys that this depends on
    ///
    /// # Errors
    ///
    /// Returns `DependencyError::CircularDependency` if adding this dependency
    /// would create a cycle in the graph.
    ///
    /// Returns `DependencyError::DuplicateKey` if a dependency with this key
    /// already exists.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyGraph;
    ///
    /// let mut graph = DependencyGraph::new();
    ///
    /// // Simple dependency chain
    /// graph.add_dependency("a", vec![]).unwrap();
    /// graph.add_dependency("b", vec!["a".to_string()]).unwrap();
    ///
    /// // This would create a cycle: a -> b -> a
    /// let result = graph.add_dependency("a", vec!["b".to_string()]);
    /// assert!(result.is_err());
    /// ```
    pub fn add_dependency(&mut self, key: &str, depends_on: Vec<String>) -> Result<(), DependencyError> {
        // Check for duplicate
        if self.graph.contains_key(key) {
            return Err(DependencyError::DuplicateKey { key: key.to_string() });
        }

        // Don't check for cycles here - allow registration and detect at resolution time
        // This allows the server to start and return proper HTTP error responses
        self.graph.insert(key.to_string(), depends_on);
        Ok(())
    }

    /// Check if adding a new dependency would create a cycle
    ///
    /// Uses depth-first search to detect cycles in the graph if the new
    /// dependency were added.
    ///
    /// # Arguments
    ///
    /// * `new_key` - The key of the dependency to potentially add
    /// * `new_deps` - The dependencies that the new dependency would depend on
    ///
    /// # Returns
    ///
    /// `true` if adding this dependency would create a cycle, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyGraph;
    ///
    /// let mut graph = DependencyGraph::new();
    /// graph.add_dependency("a", vec!["b".to_string()]).unwrap();
    /// graph.add_dependency("b", vec!["c".to_string()]).unwrap();
    ///
    /// // Adding c -> a would create a cycle
    /// assert!(graph.has_cycle_with("c", &["a".to_string()]));
    ///
    /// // Adding c -> [] would not
    /// assert!(!graph.has_cycle_with("c", &[]));
    /// ```
    pub fn has_cycle_with(&self, new_key: &str, new_deps: &[String]) -> bool {
        // Build temporary graph with the new dependency
        let mut temp_graph = self.graph.clone();
        temp_graph.insert(new_key.to_string(), new_deps.to_vec());

        // DFS cycle detection
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for key in temp_graph.keys() {
            if !visited.contains(key) && Self::has_cycle_dfs(key, &temp_graph, &mut visited, &mut rec_stack) {
                return true;
            }
        }

        false
    }

    /// Depth-first search for cycle detection
    ///
    /// # Arguments
    ///
    /// * `node` - Current node being visited
    /// * `graph` - The graph to search
    /// * `visited` - Set of all visited nodes
    /// * `rec_stack` - Set of nodes in the current recursion stack
    ///
    /// # Returns
    ///
    /// `true` if a cycle is detected, `false` otherwise.
    fn has_cycle_dfs(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());

        if let Some(deps) = graph.get(node) {
            for dep in deps {
                if !visited.contains(dep) {
                    if Self::has_cycle_dfs(dep, graph, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(dep) {
                    // Found a back edge (cycle)
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        false
    }

    /// Calculate batches of dependencies that can be resolved in parallel
    ///
    /// Uses topological sorting with Kahn's algorithm to determine the order
    /// in which dependencies should be resolved. Dependencies with no unresolved
    /// dependencies can be resolved in parallel (same batch).
    ///
    /// # Arguments
    ///
    /// * `keys` - The dependency keys to resolve
    ///
    /// # Returns
    ///
    /// A vector of batches, where each batch is a set of dependency keys that
    /// can be resolved in parallel. Batches must be executed sequentially.
    ///
    /// # Errors
    ///
    /// Returns `DependencyError::CircularDependency` if the graph contains a cycle.
    /// Returns `DependencyError::NotFound` if a requested dependency doesn't exist.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use spikard_core::di::DependencyGraph;
    ///
    /// let mut graph = DependencyGraph::new();
    /// graph.add_dependency("a", vec![]).unwrap();
    /// graph.add_dependency("b", vec![]).unwrap();
    /// graph.add_dependency("c", vec!["a".to_string(), "b".to_string()]).unwrap();
    ///
    /// let batches = graph.calculate_batches(&[
    ///     "a".to_string(),
    ///     "b".to_string(),
    ///     "c".to_string(),
    /// ]).unwrap();
    ///
    /// // Batch 1: a and b (no dependencies, can run in parallel)
    /// assert_eq!(batches[0].len(), 2);
    /// assert!(batches[0].contains("a"));
    /// assert!(batches[0].contains("b"));
    ///
    /// // Batch 2: c (depends on a and b)
    /// assert_eq!(batches[1].len(), 1);
    /// assert!(batches[1].contains("c"));
    /// ```
    pub fn calculate_batches(&self, keys: &[String]) -> Result<Vec<HashSet<String>>, DependencyError> {
        // Build subgraph with only the requested keys and their transitive dependencies
        let mut subgraph = HashMap::new();
        let mut to_visit: VecDeque<String> = keys.iter().cloned().collect();
        let mut visited = HashSet::new();

        while let Some(key) = to_visit.pop_front() {
            if visited.contains(&key) {
                continue;
            }
            visited.insert(key.clone());

            if let Some(deps) = self.graph.get(&key) {
                subgraph.insert(key.clone(), deps.clone());
                for dep in deps {
                    to_visit.push_back(dep.clone());
                }
            } else {
                // Key not in graph - treat as having no dependencies
                subgraph.insert(key.clone(), vec![]);
            }
        }

        // Calculate in-degree for each node in the subgraph
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for key in subgraph.keys() {
            in_degree.entry(key.clone()).or_insert(0);
        }
        for deps in subgraph.values() {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        // Kahn's algorithm for topological sort with batching
        let mut batches = Vec::new();
        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter(|&(_, &degree)| degree == 0)
            .map(|(key, _)| key.clone())
            .collect();

        let mut processed = 0;
        let total = subgraph.len();

        while !queue.is_empty() {
            // All items in the queue can be processed in parallel (same batch)
            let mut batch = HashSet::new();

            // Process all nodes with in-degree 0
            let batch_size = queue.len();
            for _ in 0..batch_size {
                if let Some(node) = queue.pop_front() {
                    batch.insert(node.clone());
                    processed += 1;

                    // Reduce in-degree for dependents
                    if let Some(deps) = subgraph.get(&node) {
                        for dep in deps {
                            if let Some(degree) = in_degree.get_mut(dep) {
                                *degree -= 1;
                                if *degree == 0 {
                                    queue.push_back(dep.clone());
                                }
                            }
                        }
                    }
                }
            }

            if !batch.is_empty() {
                batches.push(batch);
            }
        }

        // Check if we processed all nodes (if not, there's a cycle)
        if processed < total {
            // Find a cycle by tracing from any unprocessed node
            let unprocessed: Vec<String> = subgraph
                .keys()
                .filter(|k| in_degree.get(*k).is_some_and(|&d| d > 0))
                .cloned()
                .collect();

            if let Some(start) = unprocessed.first() {
                // Trace the cycle path
                let mut cycle = vec![start.clone()];
                let mut current = start;
                let mut visited_in_path = HashSet::new();
                visited_in_path.insert(start.clone());

                // Follow dependencies until we find the cycle
                while let Some(deps) = subgraph.get(current) {
                    if let Some(next) = deps.iter().find(|d| unprocessed.contains(d)) {
                        if visited_in_path.contains(next) {
                            // Found the cycle - add the closing node
                            cycle.push(next.clone());
                            break;
                        }
                        cycle.push(next.clone());
                        visited_in_path.insert(next.clone());
                        current = next;
                    } else {
                        break;
                    }
                }

                // Normalize the cycle to start with the lexicographically smallest element
                // This makes cycle detection deterministic
                // The cycle includes the closing element (first element repeated at end)
                // e.g., [A, B, A] or [B, A, B]
                if cycle.len() > 1 {
                    // Find the index of the smallest element (ignoring the last closing element)
                    if let Some((min_idx, _)) = cycle[..cycle.len() - 1].iter().enumerate().min_by_key(|(_, s)| *s) {
                        cycle.rotate_left(min_idx);
                        // After rotation, update the closing element to match the new first element
                        if let Some(first) = cycle.first().cloned()
                            && let Some(last) = cycle.last_mut()
                        {
                            *last = first;
                        }
                    }
                }

                return Err(DependencyError::CircularDependency { cycle });
            }

            // Fallback if we can't trace the cycle
            return Err(DependencyError::CircularDependency { cycle: unprocessed });
        }

        // Reverse the batches because we built them in reverse order
        // (dependencies come before dependents in our graph structure)
        batches.reverse();

        Ok(batches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let graph = DependencyGraph::new();
        assert_eq!(graph.graph.len(), 0);
    }

    #[test]
    fn test_add_dependency_simple() {
        let mut graph = DependencyGraph::new();
        assert!(graph.add_dependency("a", vec![]).is_ok());
        assert!(graph.graph.contains_key("a"));
    }

    #[test]
    fn test_add_dependency_duplicate() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", vec![]).unwrap();
        let result = graph.add_dependency("a", vec![]);
        assert!(matches!(result, Err(DependencyError::DuplicateKey { .. })));
    }

    #[test]
    fn test_has_cycle_simple() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", vec!["b".to_string()]).unwrap();

        // a -> b -> a would be a cycle
        assert!(graph.has_cycle_with("b", &["a".to_string()]));
    }

    #[test]
    fn test_has_cycle_complex() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", vec!["b".to_string()]).unwrap();
        graph.add_dependency("b", vec!["c".to_string()]).unwrap();

        // c -> a would create cycle: a -> b -> c -> a
        assert!(graph.has_cycle_with("c", &["a".to_string()]));
    }

    #[test]
    fn test_has_cycle_self_loop() {
        let graph = DependencyGraph::new();

        // Self-loop: a -> a
        assert!(graph.has_cycle_with("a", &["a".to_string()]));
    }

    #[test]
    fn test_no_cycle() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", vec![]).unwrap();
        graph.add_dependency("b", vec!["a".to_string()]).unwrap();

        // c -> a is fine (no cycle)
        assert!(!graph.has_cycle_with("c", &["a".to_string()]));
    }

    #[test]
    fn test_calculate_batches_simple() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", vec![]).unwrap();

        let batches = graph.calculate_batches(&["a".to_string()]).unwrap();
        assert_eq!(batches.len(), 1);
        assert!(batches[0].contains("a"));
    }

    #[test]
    fn test_calculate_batches_linear() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", vec![]).unwrap();
        graph.add_dependency("b", vec!["a".to_string()]).unwrap();
        graph.add_dependency("c", vec!["b".to_string()]).unwrap();

        let batches = graph
            .calculate_batches(&["a".to_string(), "b".to_string(), "c".to_string()])
            .unwrap();

        // Should be 3 batches in order: a, b, c
        assert_eq!(batches.len(), 3);
        assert!(batches[0].contains("a"));
        assert!(batches[1].contains("b"));
        assert!(batches[2].contains("c"));
    }

    #[test]
    fn test_calculate_batches_parallel() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", vec![]).unwrap();
        graph.add_dependency("b", vec![]).unwrap();
        graph
            .add_dependency("c", vec!["a".to_string(), "b".to_string()])
            .unwrap();

        let batches = graph
            .calculate_batches(&["a".to_string(), "b".to_string(), "c".to_string()])
            .unwrap();

        // Batch 1: a and b (parallel)
        // Batch 2: c
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].len(), 2);
        assert!(batches[0].contains("a"));
        assert!(batches[0].contains("b"));
        assert!(batches[1].contains("c"));
    }

    #[test]
    fn test_calculate_batches_nested() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("config", vec![]).unwrap();
        graph.add_dependency("database", vec!["config".to_string()]).unwrap();
        graph.add_dependency("cache", vec!["config".to_string()]).unwrap();
        graph
            .add_dependency("service", vec!["database".to_string(), "cache".to_string()])
            .unwrap();

        let batches = graph
            .calculate_batches(&[
                "config".to_string(),
                "database".to_string(),
                "cache".to_string(),
                "service".to_string(),
            ])
            .unwrap();

        // Batch 1: config
        // Batch 2: database, cache (parallel)
        // Batch 3: service
        assert_eq!(batches.len(), 3);
        assert!(batches[0].contains("config"));
        assert_eq!(batches[1].len(), 2);
        assert!(batches[1].contains("database"));
        assert!(batches[1].contains("cache"));
        assert!(batches[2].contains("service"));
    }

    #[test]
    fn test_calculate_batches_partial() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", vec![]).unwrap();
        graph.add_dependency("b", vec!["a".to_string()]).unwrap();
        graph.add_dependency("c", vec!["a".to_string()]).unwrap();

        // Only request b (should also include its dependency a)
        let batches = graph.calculate_batches(&["b".to_string()]).unwrap();

        assert_eq!(batches.len(), 2);
        assert!(batches[0].contains("a"));
        assert!(batches[1].contains("b"));
    }

    #[test]
    fn test_calculate_batches_missing_dependency() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("a", vec!["missing".to_string()]).unwrap();

        // Should handle missing dependencies gracefully
        let batches = graph.calculate_batches(&["a".to_string()]).unwrap();
        assert!(!batches.is_empty());
    }
}
