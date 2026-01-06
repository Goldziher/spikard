//! Project scaffolding traits and structures for language-specific setup.
//!
//! This module defines the contract for scaffolding new Spikard projects
//! in a language-agnostic way, allowing different implementations for
//! Python, TypeScript, Rust, Ruby, and PHP.

use std::path::PathBuf;

/// A file that will be created as part of project scaffolding.
///
/// # Fields
///
/// - `path`: Relative or absolute path where the file should be written
/// - `content`: The complete text content of the file
#[derive(Debug, Clone)]
pub struct ScaffoldedFile {
    /// Path where the file should be written
    pub path: PathBuf,
    /// Complete content of the file
    pub content: String,
}

impl ScaffoldedFile {
    /// Create a new scaffolded file.
    ///
    /// # Arguments
    ///
    /// - `path`: The target path for this file
    /// - `content`: The file content as a string
    ///
    /// # Example
    ///
    /// ```
    /// use spikard_cli::init::ScaffoldedFile;
    /// use std::path::PathBuf;
    ///
    /// let file = ScaffoldedFile::new(
    ///     PathBuf::from("src/main.py"),
    ///     "print('Hello, world!')".to_string(),
    /// );
    ///
    /// assert_eq!(file.path, PathBuf::from("src/main.py"));
    /// assert!(file.content.contains("Hello"));
    /// ```
    #[must_use] 
    pub const fn new(path: PathBuf, content: String) -> Self {
        Self { path, content }
    }
}

/// Language-agnostic trait for scaffolding new Spikard projects.
///
/// Implementations define how to create the initial project structure,
/// configuration files, and example handlers for a specific language.
///
/// # Design Philosophy
///
/// - **Zero-cost abstraction**: Trait implementations are compiled inline
/// - **Composability**: Each method is independently callable for flexibility
/// - **Clarity**: Method names and signatures are self-documenting
/// - **Extensibility**: New methods can be added without breaking existing implementations
///
/// # Example
///
/// ```ignore
/// use spikard_cli::init::{ProjectScaffolder, ScaffoldedFile};
/// use std::path::PathBuf;
///
/// struct MyScaffolder;
///
/// impl ProjectScaffolder for MyScaffolder {
///     fn scaffold(
///         &self,
///         project_dir: &std::path::Path,
///         project_name: &str,
///     ) -> anyhow::Result<Vec<ScaffoldedFile>> {
///         let mut files = vec![];
///         files.push(ScaffoldedFile::new(
///             PathBuf::from("pyproject.toml"),
///             format!("[project]\nname = \"{}\"", project_name),
///         ));
///         Ok(files)
///     }
///
///     fn next_steps(&self, _project_name: &str) -> Vec<String> {
///         vec!["cd my_api".to_string()]
///     }
/// }
/// ```
pub trait ProjectScaffolder {
    /// Scaffold a new project with language-idiomatic structure.
    ///
    /// This method is responsible for generating all files needed for a new
    /// Spikard project in the target language. The returned files will be
    /// written to disk by the caller.
    ///
    /// # Arguments
    ///
    /// - `project_dir`: The root directory where the project will be created
    /// - `project_name`: The name of the project (used for package names, module names, etc.)
    ///
    /// # Returns
    ///
    /// A vector of `ScaffoldedFile` instances representing all files to be created.
    /// The order of files is not guaranteed to be preserved on disk.
    ///
    /// # Errors
    ///
    /// Returns an error if scaffolding fails for any reason (e.g., invalid project name,
    /// I/O errors, or validation failures).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let files = scaffolder.scaffold(Path::new("."), "my_api")?;
    /// // files might contain:
    /// // - pyproject.toml
    /// // - src/main.py
    /// // - examples/basic_handler.py
    /// // - tests/test_handlers.py
    /// // - README.md
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    fn scaffold(&self, project_dir: &std::path::Path, project_name: &str) -> anyhow::Result<Vec<ScaffoldedFile>>;

    /// Return next steps messages for the user after scaffolding completes.
    ///
    /// These messages guide the user through initial setup steps like
    /// installing dependencies, running tests, or starting the server.
    ///
    /// # Arguments
    ///
    /// - `project_name`: The name of the project that was scaffolded
    ///
    /// # Returns
    ///
    /// A vector of human-readable instruction strings that should be
    /// displayed to the user after successful project creation.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let steps = scaffolder.next_steps("my_api");
    /// // steps might return:
    /// // [
    /// //   "cd my_api",
    /// //   "python -m venv venv",
    /// //   ". venv/bin/activate",
    /// //   "pip install -e .",
    /// //   "python -m pytest",
    /// // ]
    /// ```
    fn next_steps(&self, project_name: &str) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaffolded_file_creation() {
        let file = ScaffoldedFile::new(PathBuf::from("src/main.py"), "print('hello')".to_string());
        assert_eq!(file.path, PathBuf::from("src/main.py"));
        assert_eq!(file.content, "print('hello')");
    }

    #[test]
    fn test_scaffolded_file_clone() {
        let file1 = ScaffoldedFile::new(PathBuf::from("test.txt"), "content".to_string());
        let file2 = file1.clone();
        assert_eq!(file1.path, file2.path);
        assert_eq!(file1.content, file2.content);
    }
}
