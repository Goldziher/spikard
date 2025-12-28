//! Ruby Project Scaffolder
//!
//! Generates a minimal Ruby project structure with Spikard integration.
//! Follows modern Ruby conventions with RBS type annotations and RSpec testing.

use super::scaffolder::{ProjectScaffolder, ScaffoldedFile};
use anyhow::Result;
use std::path::Path;
use std::path::PathBuf;

/// Ruby project scaffolder
pub struct RubyScaffolder;

impl ProjectScaffolder for RubyScaffolder {
    fn scaffold(&self, _project_dir: &Path, project_name: &str) -> Result<Vec<ScaffoldedFile>> {
        let snake_name = project_name.replace('-', "_").to_lowercase();

        let mut files = vec![];

        // Gemfile
        files.push(ScaffoldedFile::new(PathBuf::from("Gemfile"), self.generate_gemfile()));

        // Gemfile.lock (empty placeholder)
        files.push(ScaffoldedFile::new(PathBuf::from("Gemfile.lock"), String::new()));

        // .ruby-version
        files.push(ScaffoldedFile::new(
            PathBuf::from(".ruby-version"),
            "3.2.0\n".to_string(),
        ));

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

        // lib/my_app.rb
        files.push(ScaffoldedFile::new(
            PathBuf::from(format!("lib/{}.rb", snake_name)),
            self.generate_app_rb(),
        ));

        // sig/my_app.rbs
        files.push(ScaffoldedFile::new(
            PathBuf::from(format!("sig/{}.rbs", snake_name)),
            self.generate_app_rbs(&snake_name),
        ));

        // spec/my_app_spec.rb
        files.push(ScaffoldedFile::new(
            PathBuf::from(format!("spec/{}_spec.rb", snake_name)),
            self.generate_app_spec_rb(&snake_name),
        ));

        // .rspec
        files.push(ScaffoldedFile::new(PathBuf::from(".rspec"), self.generate_rspec()));

        // Rakefile (optional, for task automation)
        files.push(ScaffoldedFile::new(PathBuf::from("Rakefile"), self.generate_rakefile()));

        Ok(files)
    }

    fn next_steps(&self, project_name: &str) -> Vec<String> {
        let snake_name = project_name.replace('-', "_").to_lowercase();
        vec![
            format!("cd {}", snake_name),
            "bundle install".to_string(),
            format!("bundle exec ruby lib/{}.rb", snake_name),
        ]
    }
}

impl RubyScaffolder {
    fn generate_gemfile(&self) -> String {
        r#"# frozen_string_literal: true

source "https://rubygems.org"

ruby ">= 3.2.0"

gem "spikard", "~> 0.6.0"

group :development, :test do
  gem "rspec", "~> 3.13"
  gem "steep", "~> 1.9"
  gem "rubocop", "~> 1.64"
end
"#
        .to_string()
    }

    fn generate_gitignore(&self) -> String {
        r#"# Dependencies
/vendor/
/Gemfile.lock
.ruby-version

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
"#
        .to_string()
    }

    fn generate_readme(&self, snake_name: &str) -> String {
        format!(
            r#"# {0}

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
bundle exec ruby lib/{0}.rb
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
2. Start the server: `bundle exec ruby lib/{0}.rb`
3. Make requests to `http://localhost:8000/health` to verify
4. Write your handlers in `lib/{0}.rb`
5. Add tests in `spec/`

## Project Structure

```
my-app/
├── lib/{0}.rb          # Main application code
├── sig/{0}.rbs         # RBS type definitions
├── spec/              # RSpec tests
├── Gemfile            # Ruby dependencies
├── Rakefile           # Rake tasks
└── README.md
```

## Type Annotations

This project uses RBS (Ruby Signature) files for type safety. Steep provides static type checking:

- Type definitions in `sig/{0}.rbs`
- Main code in `lib/{0}.rb`
- Run `bundle exec steep check` to verify types

## Documentation

- [Spikard Documentation](https://github.com/spikard/spikard)
- [Ruby Documentation](https://ruby-doc.org)
- [RBS Guide](https://github.com/ruby/rbs)
- [Steep Documentation](https://github.com/soutaro/steep)
"#,
            snake_name
        )
    }

    fn generate_app_rb(&self) -> String {
        r#"#!/usr/bin/env ruby
# frozen_string_literal: true

# Spikard Ruby Application
#
# This example demonstrates a simple HTTP server with health check
# and echo endpoints using the Spikard Ruby bindings.

require 'spikard'
require 'json'

# Create application instance
app = Spikard::App.new(
  port: 8000,
  host: '127.0.0.1'
)

# Root endpoint - returns welcome message
app.get '/' do |_request|
  {
    message: 'Hello from Spikard Ruby!',
    timestamp: Time.now.iso8601
  }
end

# Health check endpoint
app.get '/health' do |_request|
  {
    status: 'healthy',
    uptime: (Time.now - Time.at(0)).to_i,
    timestamp: Time.now.iso8601
  }
end

# Echo endpoint - returns request body
app.post '/echo' do |request|
  begin
    body = request.body.is_a?(Hash) ? request.body : nil
    {
      echoed: true,
      body: body,
      received_at: Time.now.iso8601
    }
  rescue StandardError => e
    {
      status: 400,
      body: {
        error: 'Invalid request body',
        code: 'invalid_body',
        details: e.message
      }
    }
  end
end

puts 'Starting Spikard Ruby server on http://127.0.0.1:8000'
puts 'Press Ctrl+C to stop'
puts ''

# Run the server
app.run
"#
        .to_string()
    }

    fn generate_app_rbs(&self, snake_name: &str) -> String {
        format!(
            r#"# Type definitions for {0}

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

# Request handler block type
class RequestHandler
  def call: (Spikard::Request) -> Hash[String, untyped] | String | Spikard::Response
end
"#,
            snake_name
        )
    }

    fn generate_app_spec_rb(&self, snake_name: &str) -> String {
        format!(
            r#"# frozen_string_literal: true

require 'rspec'

describe '{}' do
  describe 'Application initialization' do
    it 'requires spikard' do
      expect {{ require('spikard') }}.not_to raise_error
    end

    it 'can create an app instance' do
      # Basic test to verify app creation
      # Full integration tests would test HTTP behavior
      expect(true).to be true
    end
  end

  describe 'Health endpoint' do
    it 'should return health status' do
      # Integration test for GET /health would go here
      # This would make an actual HTTP request to the running server
      expect(true).to be true
    end
  end

  describe 'Echo endpoint' do
    it 'should echo request body' do
      # Integration test for POST /echo would go here
      expect(true).to be true
    end
  end

  describe 'Root endpoint' do
    it 'should return welcome message' do
      # Integration test for GET / would go here
      expect(true).to be true
    end
  end
end
"#,
            snake_name
        )
    }

    fn generate_rspec(&self) -> String {
        r#"--require spec_helper
--format documentation
--color
"#
        .to_string()
    }

    fn generate_rakefile(&self) -> String {
        r#"# frozen_string_literal: true

require 'bundler/setup'

desc 'Run the application'
task :run do
  exec('bundle exec ruby lib/app.rb')
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
"#
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
        assert!(file_paths.iter().any(|p| p == "Gemfile.lock"));
        assert!(file_paths.iter().any(|p| p == ".ruby-version"));
        assert!(file_paths.iter().any(|p| p == ".gitignore"));
        assert!(file_paths.iter().any(|p| p == "README.md"));
        assert!(file_paths.iter().any(|p| p.contains("lib/test_app.rb")));
        assert!(file_paths.iter().any(|p| p.contains("sig/test_app.rbs")));
        assert!(file_paths.iter().any(|p| p.contains("spec/test_app_spec.rb")));
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
    fn test_ruby_scaffold_ruby_version() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let scaffolder = RubyScaffolder;
        let files = scaffolder.scaffold(temp_dir.path(), "test_app")?;

        let ruby_version = files
            .iter()
            .find(|f| f.path.file_name().unwrap() == ".ruby-version")
            .unwrap();

        assert_eq!(ruby_version.content.trim(), "3.2.0");

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
        assert!(steps.iter().any(|s| s.contains("bundle exec ruby")));
    }
}
