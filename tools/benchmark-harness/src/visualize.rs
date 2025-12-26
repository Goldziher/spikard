use std::path::{Path, PathBuf};

use tokio::process::Command;

use crate::error::{Error, Result};
use crate::schema::aggregate::AggregatedBenchmarkResults;

#[derive(Debug, Clone)]
pub enum ChartType {
    Throughput,
    Latency,
    ValidationOverhead,
    Resources,
    All,
}

impl ChartType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "throughput" => Some(Self::Throughput),
            "latency" => Some(Self::Latency),
            "validation" | "validation-overhead" => Some(Self::ValidationOverhead),
            "resources" => Some(Self::Resources),
            "all" => Some(Self::All),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Throughput => "throughput",
            Self::Latency => "latency",
            Self::ValidationOverhead => "validation",
            Self::Resources => "resources",
            Self::All => "all",
        }
    }
}

pub struct VisualizeRunner {
    input_path: PathBuf,
    output_dir: PathBuf,
    charts: Vec<ChartType>,
    title: Option<String>,
    python_path: Option<PathBuf>,
}

impl VisualizeRunner {
    pub fn new(
        input_path: PathBuf,
        output_dir: PathBuf,
        charts: Vec<ChartType>,
        title: Option<String>,
        python_path: Option<PathBuf>,
    ) -> Self {
        Self {
            input_path,
            output_dir,
            charts,
            title,
            python_path,
        }
    }

    pub async fn run(&self) -> Result<()> {
        println!("🔍 Locating Python visualization script...");
        let script_path = self.find_script()?;
        println!("✅ Found script at {:?}", script_path);

        println!("📖 Validating input file...");
        let aggregated = self.validate_input()?;
        println!(
            "✅ Valid input with {} frameworks",
            aggregated.frameworks.len()
        );

        println!("📁 Creating output directory...");
        tokio::fs::create_dir_all(&self.output_dir)
            .await
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to create output directory: {}", e)))?;
        println!("✅ Output directory ready");

        println!("📊 Generating charts...");
        self.run_visualizer(&script_path).await?;
        println!("✅ Charts generated successfully!");

        println!();
        println!("Output:");
        println!("  Directory: {:?}", self.output_dir);
        println!("  Charts: {}", self.charts_arg());

        Ok(())
    }

    fn find_script(&self) -> Result<PathBuf> {
        let possible_paths = vec![
            PathBuf::from("tools/benchmark-harness/visualize_benchmarks.py"),
            PathBuf::from("visualize_benchmarks.py"),
            PathBuf::from("../visualize_benchmarks.py"),
        ];

        for path in possible_paths {
            if path.exists() {
                return Ok(path);
            }
        }

        Err(Error::BenchmarkFailed(
            "Could not find visualize_benchmarks.py in any of the expected locations. \
             Expected: tools/benchmark-harness/visualize_benchmarks.py"
                .to_string(),
        ))
    }

    fn validate_input(&self) -> Result<AggregatedBenchmarkResults> {
        let content = std::fs::read_to_string(&self.input_path)
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to read input file: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to parse aggregated results JSON: {}", e)))
    }

    async fn run_visualizer(&self, script_path: &Path) -> Result<()> {
        let python = self
            .python_path
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("python3");

        let mut cmd = Command::new(python);
        cmd.arg(script_path)
            .arg("--input")
            .arg(&self.input_path)
            .arg("--output")
            .arg(&self.output_dir)
            .arg("--charts")
            .arg(self.charts_arg());

        if let Some(ref title) = self.title {
            cmd.arg("--title").arg(title);
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to execute Python script: {}", e)))?;

        if !output.status.success() {
            eprintln!("Python script stderr:");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(Error::BenchmarkFailed(format!(
                "Visualization script failed with exit code: {:?}",
                output.status.code()
            )));
        }

        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }

        Ok(())
    }

    fn charts_arg(&self) -> String {
        self.charts
            .iter()
            .map(|c| c.as_str())
            .collect::<Vec<_>>()
            .join(",")
    }
}

pub fn parse_chart_types(charts_str: &str) -> Result<Vec<ChartType>> {
    let chart_names: Vec<&str> = charts_str.split(',').map(|s| s.trim()).collect();

    if chart_names.iter().any(|&name| name == "all") {
        return Ok(vec![ChartType::All]);
    }

    let mut types = Vec::new();
    for name in chart_names {
        match ChartType::from_str(name) {
            Some(chart_type) => types.push(chart_type),
            None => {
                return Err(Error::BenchmarkFailed(format!(
                    "Unknown chart type: '{}'. Valid types: throughput, latency, validation, resources, all",
                    name
                )))
            }
        }
    }

    if types.is_empty() {
        types.push(ChartType::All);
    }

    Ok(types)
}
