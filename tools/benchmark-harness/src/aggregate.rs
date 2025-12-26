use std::path::{Path, PathBuf};

use tokio::process::Command;

use crate::error::{Error, Result};

use crate::schema::{
    aggregate::{
        AggregatedBenchmarkResults, AggregationMetadata, AggregationSummary, ArtifactInfo,
        FrameworkResult,
    },
    profile::ProfileResult,
};

pub struct AggregateRunner {
    run_id: Option<String>,
    workflow: String,
    download_dir: PathBuf,
    keep_artifacts: bool,
}

impl AggregateRunner {
    pub fn new(
        run_id: Option<String>,
        workflow: String,
        download_dir: PathBuf,
        keep_artifacts: bool,
    ) -> Self {
        Self {
            run_id,
            workflow,
            download_dir,
            keep_artifacts,
        }
    }

    pub async fn run(&self, output: &Path) -> Result<()> {
        println!("🔍 Determining workflow run ID...");
        let run_id = self.resolve_run_id().await?;
        println!("✅ Using run ID: {}", run_id);

        println!("📥 Downloading artifacts from GitHub Actions...");
        let artifacts = self.download_artifacts(&run_id).await?;
        println!("✅ Downloaded {} artifacts", artifacts.len());

        println!("📊 Parsing profile results...");
        let (frameworks, artifact_infos) = self.parse_artifacts(&artifacts)?;
        println!(
            "✅ Parsed {} frameworks ({} successful, {} failed)",
            artifact_infos.len(),
            frameworks.len(),
            artifact_infos.iter().filter(|a| !a.downloaded).count()
        );

        println!("🔄 Building aggregated results...");
        let aggregated = self
            .build_aggregated_result(&run_id, frameworks, artifact_infos)
            .await?;

        println!("💾 Writing aggregated results to {:?}...", output);
        if let Some(parent) = output.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let json = serde_json::to_string_pretty(&aggregated)?;
        tokio::fs::write(output, json).await?;

        if !self.keep_artifacts {
            println!("🧹 Cleaning up downloaded artifacts...");
            tokio::fs::remove_dir_all(&self.download_dir).await.ok();
        }

        println!("✅ Aggregation complete!");
        println!();
        println!("Summary:");
        println!("  Total frameworks: {}", aggregated.summary.total_frameworks);
        println!("  Completed: {}", aggregated.summary.completed);
        println!("  Failed: {}", aggregated.summary.failed);
        println!("  Total requests: {}", aggregated.summary.total_requests);
        println!(
            "  Total duration: {:.1}s",
            aggregated.summary.total_duration_secs
        );

        Ok(())
    }

    async fn resolve_run_id(&self) -> Result<String> {
        if let Some(ref run_id) = self.run_id {
            return Ok(run_id.clone());
        }

        let output = Command::new("gh")
            .args([
                "run",
                "list",
                "--workflow",
                &self.workflow,
                "--status",
                "success",
                "--limit",
                "1",
                "--json",
                "databaseId",
                "--jq",
                ".[0].databaseId",
            ])
            .output()
            .await
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to execute gh CLI (is it installed?): {}", e)))?;

        if !output.status.success() {
            return Err(Error::BenchmarkFailed(format!(
                "Failed to query workflow runs: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let run_id = String::from_utf8(output.stdout)
            .map_err(|e| Error::BenchmarkFailed(format!("Invalid UTF-8 in gh output: {}", e)))?
            .trim()
            .to_string();

        if run_id.is_empty() || run_id == "null" {
            return Err(Error::BenchmarkFailed(format!(
                "No successful workflow runs found for '{}'",
                self.workflow
            )));
        }

        Ok(run_id)
    }

    async fn download_artifacts(&self, run_id: &str) -> Result<Vec<PathBuf>> {
        tokio::fs::create_dir_all(&self.download_dir)
            .await
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to create download directory: {}", e)))?;

        let output = Command::new("gh")
            .args([
                "run", "view", run_id, "--json", "artifacts", "--jq",
                ".artifacts[] | select(.name | startswith(\"benchmark-results-\")) | .name",
            ])
            .output()
            .await
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to list artifacts: {}", e)))?;

        if !output.status.success() {
            return Err(Error::BenchmarkFailed(format!(
                "Failed to list artifacts: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let artifact_names: Vec<String> = String::from_utf8(output.stdout)
            .map_err(|e| Error::BenchmarkFailed(format!("Invalid UTF-8 in artifact list: {}", e)))?
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|s| s.to_string())
            .collect();

        if artifact_names.is_empty() {
            return Err(Error::BenchmarkFailed(format!(
                "No benchmark artifacts found for run {}",
                run_id
            )));
        }

        let mut artifacts = Vec::new();
        for name in artifact_names {
            let artifact_dir = self.download_dir.join(&name);

            let status = Command::new("gh")
                .args([
                    "run",
                    "download",
                    run_id,
                    "--name",
                    &name,
                    "--dir",
                    artifact_dir.to_str().unwrap(),
                ])
                .status()
                .await
                .map_err(|e| Error::BenchmarkFailed(format!("Failed to download artifact {}: {}", name, e)))?;

            if status.success() {
                artifacts.push(artifact_dir);
            } else {
                eprintln!("⚠️  Failed to download artifact: {}", name);
            }
        }

        Ok(artifacts)
    }

    fn parse_artifacts(
        &self,
        artifact_dirs: &[PathBuf],
    ) -> Result<(Vec<FrameworkResult>, Vec<ArtifactInfo>)> {
        let mut frameworks = Vec::new();
        let mut artifact_infos = Vec::new();

        for dir in artifact_dirs {
            let framework_name = dir
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(|n| n.strip_prefix("benchmark-results-"))
                .unwrap_or("unknown")
                .to_string();

            let profile_path = dir.join("profile.json");

            let size_bytes = std::fs::metadata(&profile_path)
                .map(|m| m.len())
                .unwrap_or(0);

            match std::fs::read_to_string(&profile_path) {
                Ok(content) => match serde_json::from_str::<ProfileResult>(&content) {
                    Ok(profile) => {
                        artifact_infos.push(ArtifactInfo {
                            name: format!("benchmark-results-{}", framework_name),
                            framework: framework_name.clone(),
                            size_bytes,
                            downloaded: true,
                            error: None,
                        });

                        frameworks.push(FrameworkResult {
                            framework: framework_name.clone(),
                            profile,
                            status: "completed".to_string(),
                        });
                    }
                    Err(e) => {
                        eprintln!("⚠️  Failed to parse profile.json for {}: {}", framework_name, e);
                        artifact_infos.push(ArtifactInfo {
                            name: format!("benchmark-results-{}", framework_name),
                            framework: framework_name,
                            size_bytes,
                            downloaded: true,
                            error: Some(format!("Parse error: {}", e)),
                        });
                    }
                },
                Err(e) => {
                    eprintln!(
                        "⚠️  Failed to read profile.json for {}: {}",
                        framework_name, e
                    );
                    artifact_infos.push(ArtifactInfo {
                        name: format!("benchmark-results-{}", framework_name),
                        framework: framework_name,
                        size_bytes: 0,
                        downloaded: false,
                        error: Some(format!("Read error: {}", e)),
                    });
                }
            }
        }

        Ok((frameworks, artifact_infos))
    }

    async fn build_aggregated_result(
        &self,
        run_id: &str,
        frameworks: Vec<FrameworkResult>,
        artifacts: Vec<ArtifactInfo>,
    ) -> Result<AggregatedBenchmarkResults> {
        let run_url = format!(
            "https://github.com/{}/actions/runs/{}",
            self.get_repo_slug().await?,
            run_id
        );

        let git_info = self.get_git_info(run_id).await;

        let total_requests: u64 = frameworks
            .iter()
            .map(|f| f.profile.summary.total_requests)
            .sum();

        let total_duration_secs: f64 = frameworks
            .iter()
            .map(|f| f.profile.summary.total_duration_secs as f64)
            .sum();

        let metadata = AggregationMetadata {
            run_id: run_id.to_string(),
            run_url,
            workflow: self.workflow.clone(),
            commit: git_info.0,
            branch: git_info.1,
            aggregated_at: chrono::Utc::now().to_rfc3339(),
            artifact_count: artifacts.len(),
            artifacts,
        };

        let completed = frameworks.len();
        let failed = metadata
            .artifacts
            .iter()
            .filter(|a| !a.downloaded)
            .count();

        let summary = AggregationSummary {
            total_frameworks: completed + failed,
            completed,
            failed,
            total_requests,
            total_duration_secs,
        };

        Ok(AggregatedBenchmarkResults {
            metadata,
            frameworks,
            summary,
        })
    }

    async fn get_repo_slug(&self) -> Result<String> {
        let output = Command::new("gh")
            .args(["repo", "view", "--json", "nameWithOwner", "--jq", ".nameWithOwner"])
            .output()
            .await
            .map_err(|e| Error::BenchmarkFailed(format!("Failed to get repository slug: {}", e)))?;

        if !output.status.success() {
            return Err(Error::BenchmarkFailed(format!(
                "Failed to get repository slug: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let slug = String::from_utf8(output.stdout)
            .map_err(|e| Error::BenchmarkFailed(format!("Invalid UTF-8 in repository slug: {}", e)))?;

        Ok(slug.trim().to_string())
    }

    async fn get_git_info(&self, run_id: &str) -> (Option<String>, Option<String>) {
        let output = Command::new("gh")
            .args([
                "run",
                "view",
                run_id,
                "--json",
                "headSha,headBranch",
                "--jq",
                "{commit: .headSha, branch: .headBranch}",
            ])
            .output()
            .await;

        match output {
            Ok(output) if output.status.success() => {
                match serde_json::from_slice::<serde_json::Value>(&output.stdout) {
                    Ok(json) => {
                        let commit = json["commit"].as_str().map(|s| s.to_string());
                        let branch = json["branch"].as_str().map(|s| s.to_string());
                        (commit, branch)
                    }
                    Err(_) => (None, None),
                }
            }
            _ => (None, None),
        }
    }
}
