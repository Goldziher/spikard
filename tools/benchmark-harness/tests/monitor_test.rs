//! Unit tests for resource monitoring

use benchmark_harness::monitor::{ResourceMonitor, ResourceSample};
use std::time::Duration;

#[test]
fn test_resource_sample_creation() {
    let sample = ResourceSample {
        memory_bytes: 1024 * 1024 * 50, // 50MB
        cpu_percent: 25.5,
    };

    assert_eq!(sample.memory_bytes, 1024 * 1024 * 50);
    assert_eq!(sample.cpu_percent, 25.5);
}

#[test]
fn test_monitor_sample_own_process() {
    // Monitor our own process
    let pid = std::process::id();
    let mut monitor = ResourceMonitor::new(pid);

    // Take a sample
    let sample = monitor.sample();
    assert!(sample.is_some());

    let sample = sample.unwrap();
    // Memory should be > 0
    assert!(sample.memory_bytes > 0);
    // CPU might be 0 for a quick sample, so just check it's not negative
    assert!(sample.cpu_percent >= 0.0);
}

#[test]
fn test_monitor_sample_nonexistent_process() {
    // Use a PID that definitely doesn't exist
    let mut monitor = ResourceMonitor::new(u32::MAX);

    // Should return None for nonexistent process
    let sample = monitor.sample();
    assert!(sample.is_none());
}

#[test]
fn test_monitor_calculate_metrics_empty() {
    let monitor = ResourceMonitor::new(std::process::id());

    // No samples taken yet
    let metrics = monitor.calculate_metrics();

    // All should be 0
    assert_eq!(metrics.avg_memory_mb, 0.0);
    assert_eq!(metrics.peak_memory_mb, 0.0);
    assert_eq!(metrics.p50_memory_mb, 0.0);
    assert_eq!(metrics.p95_memory_mb, 0.0);
    assert_eq!(metrics.p99_memory_mb, 0.0);
    assert_eq!(metrics.avg_cpu_percent, 0.0);
    assert_eq!(metrics.peak_cpu_percent, 0.0);
}

#[test]
fn test_monitor_calculate_metrics_single_sample() {
    let pid = std::process::id();
    let mut monitor = ResourceMonitor::new(pid);

    // Take one sample
    monitor.sample();

    let metrics = monitor.calculate_metrics();

    // Should have some values
    assert!(metrics.avg_memory_mb >= 0.0);
    assert!(metrics.peak_memory_mb >= 0.0);
    assert!(metrics.p50_memory_mb >= 0.0);
    assert!(metrics.avg_cpu_percent >= 0.0);
}

#[test]
fn test_monitor_calculate_metrics_multiple_samples() {
    let pid = std::process::id();
    let mut monitor = ResourceMonitor::new(pid);

    // Take multiple samples
    for _ in 0..10 {
        monitor.sample();
        std::thread::sleep(Duration::from_millis(10));
    }

    let metrics = monitor.calculate_metrics();

    // Check that percentiles are ordered correctly
    assert!(metrics.p50_memory_mb <= metrics.p95_memory_mb);
    assert!(metrics.p95_memory_mb <= metrics.p99_memory_mb);
    assert!(metrics.p99_memory_mb <= metrics.peak_memory_mb);
    assert!(metrics.avg_memory_mb > 0.0);
}

#[test]
fn test_monitor_samples_accessor() {
    let pid = std::process::id();
    let mut monitor = ResourceMonitor::new(pid);

    // Initially no samples
    assert_eq!(monitor.samples().len(), 0);

    // Take some samples
    monitor.sample();
    monitor.sample();
    monitor.sample();

    // Should have 3 samples
    assert_eq!(monitor.samples().len(), 3);
}

#[tokio::test]
async fn test_monitor_start_monitoring() {
    let pid = std::process::id();
    let monitor = ResourceMonitor::new(pid);

    // Start monitoring with 50ms interval
    let handle = monitor.start_monitoring(50);

    // Let it collect some samples
    tokio::time::sleep(Duration::from_millis(300)).await;

    // Stop monitoring
    let monitor = handle.stop().await;

    // Should have collected multiple samples (at least 5 in 300ms with 50ms interval)
    assert!(monitor.samples().len() >= 5);

    // Calculate metrics
    let metrics = monitor.calculate_metrics();
    assert!(metrics.avg_memory_mb > 0.0);
}

#[tokio::test]
async fn test_monitor_percentile_calculation() {
    let pid = std::process::id();
    let monitor = ResourceMonitor::new(pid);

    // Start monitoring
    let handle = monitor.start_monitoring(20);

    // Run for a bit to collect samples
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Stop and get metrics
    let monitor = handle.stop().await;
    let metrics = monitor.calculate_metrics();

    // Verify percentile ordering
    assert!(metrics.p50_memory_mb <= metrics.p95_memory_mb);
    assert!(metrics.p95_memory_mb <= metrics.p99_memory_mb);
    assert!(metrics.avg_memory_mb > 0.0);
    assert!(metrics.peak_memory_mb >= metrics.p99_memory_mb);

    // CPU percentiles
    assert!(metrics.avg_cpu_percent >= 0.0);
    assert!(metrics.peak_cpu_percent >= metrics.avg_cpu_percent);
}

#[tokio::test]
async fn test_monitor_handle_drop() {
    let pid = std::process::id();
    let monitor = ResourceMonitor::new(pid);

    // Start monitoring
    let handle = monitor.start_monitoring(50);

    // Let it run briefly
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Drop the handle without calling stop
    drop(handle);

    // Should not panic or cause issues
}

#[tokio::test]
async fn test_monitor_stops_on_process_exit() {
    // This test monitors a nonexistent process
    // The monitoring loop should exit when it can't sample
    let monitor = ResourceMonitor::new(u32::MAX);

    let handle = monitor.start_monitoring(50);

    // Wait a bit - the monitor should exit quickly
    tokio::time::sleep(Duration::from_millis(200)).await;

    let monitor = handle.stop().await;

    // Should have no samples since process doesn't exist
    assert_eq!(monitor.samples().len(), 0);
}

#[test]
fn test_percentile_edge_cases() {
    let pid = std::process::id();
    let mut monitor = ResourceMonitor::new(pid);

    // Single sample - all percentiles should be equal
    monitor.sample();
    let metrics = monitor.calculate_metrics();

    assert_eq!(metrics.p50_memory_mb, metrics.p95_memory_mb);
    assert_eq!(metrics.p95_memory_mb, metrics.p99_memory_mb);
    assert_eq!(metrics.p99_memory_mb, metrics.peak_memory_mb);
}
