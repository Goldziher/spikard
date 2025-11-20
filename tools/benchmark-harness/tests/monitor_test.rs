//! Unit tests for resource monitoring

use benchmark_harness::monitor::{ResourceMonitor, ResourceSample};
use std::time::Duration;

#[test]
fn test_resource_sample_creation() {
    let sample = ResourceSample {
        memory_bytes: 1024 * 1024 * 50,
        cpu_percent: 25.5,
    };

    assert_eq!(sample.memory_bytes, 1024 * 1024 * 50);
    assert_eq!(sample.cpu_percent, 25.5);
}

#[test]
fn test_monitor_sample_own_process() {
    let pid = std::process::id();
    let mut monitor = ResourceMonitor::new(pid);

    let sample = monitor.sample();
    assert!(sample.is_some());

    let sample = sample.unwrap();
    assert!(sample.memory_bytes > 0);
    assert!(sample.cpu_percent >= 0.0);
}

#[test]
fn test_monitor_sample_nonexistent_process() {
    let mut monitor = ResourceMonitor::new(u32::MAX);

    let sample = monitor.sample();
    assert!(sample.is_none());
}

#[test]
fn test_monitor_calculate_metrics_empty() {
    let monitor = ResourceMonitor::new(std::process::id());

    let metrics = monitor.calculate_metrics();

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

    monitor.sample();

    let metrics = monitor.calculate_metrics();

    assert!(metrics.avg_memory_mb >= 0.0);
    assert!(metrics.peak_memory_mb >= 0.0);
    assert!(metrics.p50_memory_mb >= 0.0);
    assert!(metrics.avg_cpu_percent >= 0.0);
}

#[test]
fn test_monitor_calculate_metrics_multiple_samples() {
    let pid = std::process::id();
    let mut monitor = ResourceMonitor::new(pid);

    for _ in 0..10 {
        monitor.sample();
        std::thread::sleep(Duration::from_millis(10));
    }

    let metrics = monitor.calculate_metrics();

    assert!(metrics.p50_memory_mb <= metrics.p95_memory_mb);
    assert!(metrics.p95_memory_mb <= metrics.p99_memory_mb);
    assert!(metrics.p99_memory_mb <= metrics.peak_memory_mb);
    assert!(metrics.avg_memory_mb > 0.0);
}

#[test]
fn test_monitor_samples_accessor() {
    let pid = std::process::id();
    let mut monitor = ResourceMonitor::new(pid);

    assert_eq!(monitor.samples().len(), 0);

    monitor.sample();
    monitor.sample();
    monitor.sample();

    assert_eq!(monitor.samples().len(), 3);
}

#[tokio::test]
async fn test_monitor_start_monitoring() {
    let pid = std::process::id();
    let monitor = ResourceMonitor::new(pid);

    let handle = monitor.start_monitoring(50);

    tokio::time::sleep(Duration::from_millis(300)).await;

    let monitor = handle.stop().await;

    assert!(monitor.samples().len() >= 5);

    let metrics = monitor.calculate_metrics();
    assert!(metrics.avg_memory_mb > 0.0);
}

#[tokio::test]
async fn test_monitor_percentile_calculation() {
    let pid = std::process::id();
    let monitor = ResourceMonitor::new(pid);

    let handle = monitor.start_monitoring(20);

    tokio::time::sleep(Duration::from_millis(200)).await;

    let monitor = handle.stop().await;
    let metrics = monitor.calculate_metrics();

    assert!(metrics.p50_memory_mb <= metrics.p95_memory_mb);
    assert!(metrics.p95_memory_mb <= metrics.p99_memory_mb);
    assert!(metrics.avg_memory_mb > 0.0);
    assert!(metrics.peak_memory_mb >= metrics.p99_memory_mb);

    assert!(metrics.avg_cpu_percent >= 0.0);
    assert!(metrics.peak_cpu_percent >= metrics.avg_cpu_percent);
}

#[tokio::test]
async fn test_monitor_handle_drop() {
    let pid = std::process::id();
    let monitor = ResourceMonitor::new(pid);

    let handle = monitor.start_monitoring(50);

    tokio::time::sleep(Duration::from_millis(100)).await;

    drop(handle);
}

#[tokio::test]
async fn test_monitor_stops_on_process_exit() {
    let monitor = ResourceMonitor::new(u32::MAX);

    let handle = monitor.start_monitoring(50);

    tokio::time::sleep(Duration::from_millis(200)).await;

    let monitor = handle.stop().await;

    assert_eq!(monitor.samples().len(), 0);
}

#[test]
fn test_percentile_edge_cases() {
    let pid = std::process::id();
    let mut monitor = ResourceMonitor::new(pid);

    monitor.sample();
    let metrics = monitor.calculate_metrics();

    assert_eq!(metrics.p50_memory_mb, metrics.p95_memory_mb);
    assert_eq!(metrics.p95_memory_mb, metrics.p99_memory_mb);
    assert_eq!(metrics.p99_memory_mb, metrics.peak_memory_mb);
}
