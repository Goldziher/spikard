use std::path::{Path, PathBuf};
use std::str::FromStr;

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
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Throughput => "throughput",
            Self::Latency => "latency",
            Self::ValidationOverhead => "validation",
            Self::Resources => "resources",
            Self::All => "all",
        }
    }
}

impl FromStr for ChartType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "throughput" => Ok(Self::Throughput),
            "latency" => Ok(Self::Latency),
            "validation" | "validation-overhead" => Ok(Self::ValidationOverhead),
            "resources" => Ok(Self::Resources),
            "all" => Ok(Self::All),
            _ => Err(format!("Unknown chart type: '{s}'")),
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
    #[must_use]
    pub const fn new(
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

    /// Run the visualization process.
    ///
    /// # Errors
    /// Returns an error if the visualization script is not found, input is invalid, or the script fails.
    pub async fn run(&self) -> Result<()> {
        println!("🔍 Locating Python visualization script...");
        let script_path = Self::find_script()?;
        println!("✅ Found script at {}", script_path.display());

        println!("📖 Validating input file...");
        let aggregated = self.validate_input()?;
        println!("✅ Valid input with {} frameworks", aggregated.frameworks.len());

        println!("📁 Creating output directory...");
        tokio::fs::create_dir_all(&self.output_dir)
            .await
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to create output directory: {e}")))?;
        println!("✅ Output directory ready");

        println!("📊 Generating charts...");
        self.run_visualizer(&script_path).await?;
        println!("✅ Charts generated successfully!");

        println!();
        println!("Output:");
        println!("  Directory: {}", self.output_dir.display());
        println!("  Charts: {}", self.charts_arg());

        Ok(())
    }

    /// Find the visualization script.
    ///
    /// # Errors
    /// Returns an error if the script cannot be found.
    fn find_script() -> Result<PathBuf> {
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
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to read input file: {e}")))?;

        serde_json::from_str(&content)
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to parse aggregated results JSON: {e}")))
    }

    async fn run_visualizer(&self, script_path: &Path) -> Result<()> {
        let python = self.python_path.as_ref().and_then(|p| p.to_str()).unwrap_or("python3");

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
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to execute Python script: {e}")))?;

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
        self.charts.iter().map(ChartType::as_str).collect::<Vec<_>>().join(",")
    }
}

/// Parse comma-separated chart type strings into `ChartType` enums.
///
/// # Errors
/// Returns an error if an unknown chart type is encountered.
pub fn parse_chart_types(charts_str: &str) -> Result<Vec<ChartType>> {
    let chart_names: Vec<&str> = charts_str.split(',').map(str::trim).collect();

    if chart_names.contains(&"all") {
        return Ok(vec![ChartType::All]);
    }

    let mut types = Vec::new();
    for name in chart_names {
        match name.parse::<ChartType>() {
            Ok(chart_type) => types.push(chart_type),
            Err(_) => {
                return Err(Error::BenchmarkFailed(format!(
                    "Unknown chart type: '{name}'. Valid types: throughput, latency, validation, resources, all"
                )));
            }
        }
    }

    if types.is_empty() {
        types.push(ChartType::All);
    }

    Ok(types)
}
