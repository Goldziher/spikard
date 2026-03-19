//! Elixir project scaffolder for Spikard applications.

use super::scaffolder::{ProjectScaffolder, ScaffoldedFile};
use anyhow::Result;
use heck::{ToPascalCase, ToSnakeCase};
use std::path::{Path, PathBuf};

/// Elixir project scaffolder.
pub struct ElixirScaffolder;

impl ElixirScaffolder {
    fn app_name(project_name: &str) -> String {
        project_name.replace('-', "_").to_snake_case()
    }

    fn module_name(project_name: &str) -> String {
        Self::app_name(project_name).to_pascal_case()
    }

    fn generate_mix_exs(&self, project_name: &str) -> String {
        let app_name = Self::app_name(project_name);
        let module_name = Self::module_name(project_name);
        let version = env!("CARGO_PKG_VERSION");

        format!(
            r#"defmodule {module_name}.MixProject do
  use Mix.Project

  def project do
    [
      app: :{app_name},
      version: "0.1.0",
      elixir: "~> 1.19",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {{:spikard, "~> {version}"}},
      {{:jason, "~> 1.4"}}
    ]
  end
end
"#
        )
    }

    fn generate_formatter_exs(&self) -> String {
        r#"[
  inputs: ["{mix,.formatter}.exs", "{config,lib,test}/**/*.{ex,exs}"],
  line_length: 120
]
"#
        .to_string()
    }

    fn generate_router(&self, project_name: &str) -> String {
        let module_name = Self::module_name(project_name);

        format!(
            r#"defmodule {module_name}.Router do
  @moduledoc "Generated Spikard router scaffold."

  use Spikard.Router

  get "/health", &health/1

  @spec health(Spikard.Request.t()) :: Spikard.Response.t()
  def health(_request) do
    Spikard.Response.json(%{{status: "ok"}})
  end
end
"#
        )
    }

    fn generate_test(&self, project_name: &str) -> String {
        let module_name = Self::module_name(project_name);

        format!(
            r#"defmodule {module_name}.RouterTest do
  use ExUnit.Case, async: true

  alias {module_name}.Router

  test "router exposes the generated health route" do
    routes = Router.routes()

    assert Enum.any?(routes, fn route ->
             route.method == "GET" and route.path == "/health"
           end)
  end
end
"#
        )
    }

    fn generate_gitignore(&self) -> String {
        r#"/_build/
/deps/
/cover/
/.elixir_ls/
/.lexical/
erl_crash.dump
*.ez
.DS_Store
"#
        .to_string()
    }
}

impl ProjectScaffolder for ElixirScaffolder {
    fn scaffold(&self, _project_dir: &Path, project_name: &str) -> Result<Vec<ScaffoldedFile>> {
        let app_name = Self::app_name(project_name);

        Ok(vec![
            ScaffoldedFile::new(PathBuf::from("mix.exs"), self.generate_mix_exs(project_name)),
            ScaffoldedFile::new(PathBuf::from(".formatter.exs"), self.generate_formatter_exs()),
            ScaffoldedFile::new(
                PathBuf::from(format!("lib/{app_name}.ex")),
                self.generate_router(project_name),
            ),
            ScaffoldedFile::new(
                PathBuf::from(format!("test/{}_test.exs", app_name)),
                self.generate_test(project_name),
            ),
            ScaffoldedFile::new(PathBuf::from("test/test_helper.exs"), "ExUnit.start()\n".to_string()),
            ScaffoldedFile::new(PathBuf::from(".gitignore"), self.generate_gitignore()),
        ])
    }

    fn next_steps(&self, project_name: &str) -> Vec<String> {
        vec![
            format!("cd {}", project_name),
            "mix deps.get".to_string(),
            "mix test".to_string(),
        ]
    }
}
