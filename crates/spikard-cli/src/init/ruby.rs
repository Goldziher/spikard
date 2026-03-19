//! Ruby Project Scaffolder
//!
//! Generates a minimal Ruby project structure with Spikard integration.
//! Follows modern Ruby conventions with RBS type annotations and `RSpec` testing.

use super::scaffolder::{ProjectScaffolder, ScaffoldedFile};
use anyhow::Result;
use heck::ToPascalCase;
use std::path::Path;
use std::path::PathBuf;

/// Ruby project scaffolder
pub struct RubyScaffolder;

impl ProjectScaffolder for RubyScaffolder {
    fn scaffold(&self, _project_dir: &Path, project_name: &str) -> Result<Vec<ScaffoldedFile>> {
        let snake_name = project_name.replace('-', "_").to_lowercase();
        let module_name = snake_name.to_pascal_case();

        let mut files = vec![];

        // Gemfile
        files.push(ScaffoldedFile::new(PathBuf::from("Gemfile"), self.generate_gemfile()));

        // .gitignore
        files.push(ScaffoldedFile::new(
            PathBuf::from(".gitignore"),
            self.generate_gitignore(),
        ));

        // README.md
        files.push(ScaffoldedFile::new(
            PathBuf::from("README.md"),
            self.generate_readme(&snake_name),
        ));

        files.push(ScaffoldedFile::new(
            PathBuf::from("bin/server"),
            self.generate_server_script(&snake_name, &module_name),
        ));

        // lib/my_app.rb
        files.push(ScaffoldedFile::new(
            PathBuf::from(format!("lib/{snake_name}.rb")),
            self.generate_app_rb(&module_name),
        ));

        // sig/my_app.rbs
        files.push(ScaffoldedFile::new(
            PathBuf::from(format!("sig/{snake_name}.rbs")),
            self.generate_app_rbs(&snake_name, &module_name),
        ));

        // spec/my_app_spec.rb
        files.push(ScaffoldedFile::new(
            PathBuf::from(format!("spec/{snake_name}_spec.rb")),
            self.generate_app_spec_rb(&snake_name, &module_name),
        ));

        files.push(ScaffoldedFile::new(
            PathBuf::from("spec/spec_helper.rb"),
            self.generate_spec_helper(),
        ));

        // .rspec
        files.push(ScaffoldedFile::new(PathBuf::from(".rspec"), self.generate_rspec()));

        // Rakefile (optional, for task automation)
        files.push(ScaffoldedFile::new(PathBuf::from("Rakefile"), self.generate_rakefile()));

        Ok(files)
    }

    fn next_steps(&self, project_name: &str) -> Vec<String> {
        vec![
            format!("cd {}", project_name),
            "bundle install".to_string(),
            "bundle exec ruby bin/server".to_string(),
        ]
    }
}

impl RubyScaffolder {
    fn generate_gemfile(&self) -> String {
        let version = env!("CARGO_PKG_VERSION");
        format!(
            r#"# frozen_string_literal: true

source "https://rubygems.org"

ruby ">= 3.2.0"

gem "spikard", "~> {version}"

group :development, :test do
  gem "rspec", "~> 3.13"
  gem "steep", "~> 1.9"
  gem "rubocop", "~> 1.64"
end
"#
        )
    }

    fn generate_gitignore(&self) -> String {
        r"# Dependencies
/vendor/

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# Testing
/coverage/
/.rspec_status
/.rspec_results

# Environment
.env
.env.local

# RubyMine
.rmvrc

# Temp files
*.tmp
*.log

# OS
.DS_Store
Thumbs.db

# Steep
.steep.log
"
        .to_string()
    }

    fn generate_readme(&self, snake_name: &str) -> String {
        format!(
            r"# {snake_name}

A Spikard Ruby application.

## Requirements

- Ruby 3.2+
- Bundler

## Installation

```bash
bundle install
```

## Development

Start the development server:

```bash
bundle exec ruby bin/server
```

The server will start on `http://127.0.0.1:8000`.

## Testing

Run tests:

```bash
bundle exec rspec
```

Run tests with coverage:

```bash
COVERAGE=true bundle exec rspec
```

## Type Checking

Steep performs static type checking using RBS type annotations:

```bash
bundle exec steep check
```

## Linting & Formatting

Lint the code:

```bash
bundle exec rubocop
```

Auto-fix issues:

```bash
bundle exec rubocop -A
```

## Next Steps

1. Install dependencies: `bundle install`
2. Start the server: `bundle exec ruby bin/server`
3. Make requests to `http://localhost:8000/health` to verify
4. Write your handlers in `lib/{snake_name}.rb`
5. Add tests in `spec/`

## Project Structure

```
my-app/
├── bin/server              # Runnable entrypoint
├── lib/{snake_name}.rb          # Main application code
├── sig/{snake_name}.rbs         # RBS type definitions
├── spec/              # RSpec tests
├── Gemfile            # Ruby dependencies
├── Rakefile           # Rake tasks
└── README.md
```

## Type Annotations

This project uses RBS (Ruby Signature) files for type safety. Steep provides static type checking:

- Type definitions in `sig/{snake_name}.rbs`
- Main code in `lib/{snake_name}.rb`
- Run `bundle exec steep check` to verify types

## Documentation

- [Spikard Documentation](https://github.com/Goldziher/spikard)
- [Ruby Documentation](https://ruby-doc.org)
- [RBS Guide](https://github.com/ruby/rbs)
- [Steep Documentation](https://github.com/soutaro/steep)
"
        )
    }

    fn generate_server_script(&self, snake_name: &str, module_name: &str) -> String {
        format!(
            r#"#!/usr/bin/env ruby
# frozen_string_literal: true

require_relative '../lib/{snake_name}'

app = {module_name}.build_app

puts 'Starting Spikard Ruby server on http://127.0.0.1:8000'
puts 'Press Ctrl+C to stop'
puts ''

app.run
"#
        )
    }

    fn generate_app_rb(&self, module_name: &str) -> String {
        format!(
            r#"# frozen_string_literal: true

require 'json'
require 'spikard'
require 'time'

module {module_name}
  def self.build_app
    app = Spikard::App.new(
      port: 8000,
      host: '127.0.0.1'
    )

    app.get '/' do |_request|
      {{
        message: 'Hello from Spikard Ruby!',
        timestamp: Time.now.iso8601
      }}
    end

    app.get '/health' do |_request|
      {{
        status: 'healthy',
        timestamp: Time.now.iso8601
      }}
    end

    app.post '/echo' do |request|
      body = request.body.is_a?(Hash) ? request.body : nil
      {{
        echoed: true,
        body: body,
        received_at: Time.now.iso8601
      }}
    rescue StandardError => e
      {{
        status: 400,
        body: {{
          error: 'Invalid request body',
          code: 'invalid_body',
          details: e.message
        }}
      }}
    end

    app
  end
end
"#
        )
    }

    fn generate_app_rbs(&self, snake_name: &str, module_name: &str) -> String {
        format!(
            r"# Type definitions for {snake_name}

module {module_name}
  def self.build_app: () -> Spikard::App
end

module Spikard
  class App
    def initialize: (port: Integer, host: String) -> void
    def get: (String) -> void
    def post: (String) -> void
    def put: (String) -> void
    def delete: (String) -> void
    def patch: (String) -> void
    def run: () -> void
  end

  class Request
    def body: Hash[String, untyped] | nil
    def headers: Hash[String, String]
    def path: String
    def method: String
    def params: Hash[String, String]
    def json: -> Hash[String, untyped]
  end

  class Response
    def self.json: (Hash[String, untyped]) -> Response
    def self.text: (String) -> Response
    def self.status: (Integer, untyped) -> Response
  end
end

"
        )
    }

    fn generate_app_spec_rb(&self, snake_name: &str, module_name: &str) -> String {
        format!(
            r"# frozen_string_literal: true

require 'spec_helper'
require_relative '../lib/{snake_name}'

RSpec.describe {module_name} do
  describe '.build_app' do
    it 'creates a Spikard application' do
      expect(described_class.build_app).to be_a(Spikard::App)
    end
  end

  describe 'generated routes' do
    it 'builds an app without raising' do
      expect {{ described_class.build_app }}.not_to raise_error
    end
  end
end
"
        )
    }

    fn generate_spec_helper(&self) -> String {
        r"# frozen_string_literal: true

require 'bundler/setup'
require 'spikard'
"
        .to_string()
    }

    fn generate_rspec(&self) -> String {
        r"--require spec_helper
--format documentation
--color
"
        .to_string()
    }

    fn generate_rakefile(&self) -> String {
        r"# frozen_string_literal: true

require 'bundler/setup'

desc 'Run the application'
task :run do
  exec('bundle exec ruby bin/server')
end

desc 'Run RSpec tests'
task :spec do
  exec('bundle exec rspec')
end

desc 'Run Steep type checking'
task :type_check do
  exec('bundle exec steep check')
end

desc 'Run RuboCop linter'
task :lint do
  exec('bundle exec rubocop')
end

desc 'Auto-fix RuboCop issues'
task :lint_fix do
  exec('bundle exec rubocop -A')
end

task default: :spec
"
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_ruby_scaffold_creates_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scaffolder = RubyScaffolder;
        let files = scaffolder.scaffold(temp_dir.path(), "test_app")?;

        assert!(!files.is_empty(), "Should create multiple files");

        // Check expected files exist in the vec
        let file_paths: Vec<_> = files.iter().map(|f| f.path.to_string_lossy().to_string()).collect();

        assert!(file_paths.iter().any(|p| p == "Gemfile"));
        assert!(file_paths.iter().any(|p| p == ".gitignore"));
        assert!(file_paths.iter().any(|p| p == "README.md"));
        assert!(file_paths.iter().any(|p| p == "bin/server"));
        assert!(file_paths.iter().any(|p| p.contains("lib/test_app.rb")));
        assert!(file_paths.iter().any(|p| p.contains("sig/test_app.rbs")));
        assert!(file_paths.iter().any(|p| p.contains("spec/test_app_spec.rb")));
        assert!(file_paths.iter().any(|p| p == "spec/spec_helper.rb"));
        assert!(file_paths.iter().any(|p| p == ".rspec"));
        assert!(file_paths.iter().any(|p| p == "Rakefile"));

        Ok(())
    }

    #[test]
    fn test_ruby_scaffold_gemfile_valid() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scaffolder = RubyScaffolder;
        let files = scaffolder.scaffold(temp_dir.path(), "my_app")?;

        let gemfile = files.iter().find(|f| f.path.file_name().unwrap() == "Gemfile").unwrap();

        assert!(gemfile.content.contains("ruby \">= 3.2.0\""));
        assert!(gemfile.content.contains("spikard"));
        assert!(gemfile.content.contains("rspec"));
        assert!(gemfile.content.contains("steep"));
        assert!(gemfile.content.contains("rubocop"));

        Ok(())
    }

    #[test]
    fn test_ruby_scaffold_rbs_type_definitions() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scaffolder = RubyScaffolder;
        let files = scaffolder.scaffold(temp_dir.path(), "test_app")?;

        let rbs = files
            .iter()
            .find(|f| f.path.to_string_lossy().ends_with(".rbs"))
            .unwrap();

        assert!(rbs.content.contains("module Spikard"));
        assert!(rbs.content.contains("class App"));
        assert!(rbs.content.contains("class Request"));
        assert!(rbs.content.contains("class Response"));

        Ok(())
    }

    #[test]
    fn test_ruby_scaffold_app_rb_has_handlers() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scaffolder = RubyScaffolder;
        let files = scaffolder.scaffold(temp_dir.path(), "test_app")?;

        let app_rb = files
            .iter()
            .find(|f| f.path.to_string_lossy().ends_with("lib/test_app.rb"))
            .unwrap();

        assert!(app_rb.content.contains("module TestApp"));
        assert!(app_rb.content.contains("def self.build_app"));
        assert!(app_rb.content.contains("Spikard::App.new"));
        assert!(app_rb.content.contains("app.get"));
        assert!(app_rb.content.contains("app.post"));
        assert!(app_rb.content.contains("'/'"));
        assert!(app_rb.content.contains("'/health'"));
        assert!(app_rb.content.contains("'/echo'"));

        Ok(())
    }

    #[test]
    fn test_ruby_scaffold_spec_file_exists() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scaffolder = RubyScaffolder;
        let files = scaffolder.scaffold(temp_dir.path(), "my_app")?;

        let spec = files
            .iter()
            .find(|f| f.path.to_string_lossy().ends_with("spec/my_app_spec.rb"))
            .unwrap();

        assert!(spec.content.contains("describe"));
        assert!(spec.content.contains("it"));
        assert!(spec.content.contains("expect"));

        Ok(())
    }

    #[test]
    fn test_ruby_next_steps() {
        let scaffolder = RubyScaffolder;
        let steps = scaffolder.next_steps("my_app");

        assert!(!steps.is_empty());
        assert!(steps[0].contains("my_app"));
        assert!(steps.iter().any(|s| s.contains("bundle install")));
        assert!(steps.iter().any(|s| s.contains("bundle exec ruby bin/server")));
    }
}
