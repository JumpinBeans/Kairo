//! Defines traits for abstracting operating system services like file system access
//! and console I/O, allowing for platform-agnostic implementations or testing mocks.

use std::path::{Path, PathBuf};

/// Trait defining operations for a file system service.
///
/// This abstraction allows for interacting with the file system in a way
/// that can be implemented by the host OS or a simulated environment.
/// Errors are generally returned as `String` for simplicity in this abstraction.
pub trait FileSystemService {
    /// Reads the entire contents of a file into a string.
    fn read_to_string(&self, path: &Path) -> Result<String, String>;
    /// Writes a string to a file, creating the file if it does not exist,
    /// and truncating it if it does.
    fn write_string(&self, path: &Path, content: &str) -> Result<(), String>;
    /// Lists the names of entries within a directory.
    fn list_directory(&self, path: &Path) -> Result<Vec<String>, String>;
    /// Creates a new directory and any necessary parent directories.
    fn create_directory(&self, path: &Path) -> Result<(), String>;
    /// Removes a file.
    fn remove_file(&self, path: &Path) -> Result<(), String>;
    /// Removes a directory and all its contents recursively.
    fn remove_directory_recursive(&self, path: &Path) -> Result<(), String>;
    /// Returns the current working directory as a `PathBuf`.
    fn current_working_directory(&self) -> Result<PathBuf, String>;
    /// Changes the current working directory to the specified path.
    fn set_current_working_directory(&self, path: &Path) -> Result<(), String>;
    /// Checks if a path exists and is accessible.
    fn path_exists(&self, path: &Path) -> bool;
}

/// Trait defining operations for a console service.
///
/// This abstraction allows for interacting with the console (stdout/stdin)
/// in a way that can be implemented by the host OS or a simulated environment.
pub trait ConsoleService {
    /// Prints a line of text to the console (typically stdout).
    /// Implementations should handle appending a newline character.
    fn print_line(&self, text: &str);
    // /// Reads a line of text from the console (typically stdin).
    // /// Could be added in the future if direct stdin access needs to be abstracted.
    // fn read_line(&self, prompt: &str) -> Result<String, String>;
}

/// A concrete implementation of `FileSystemService` and `ConsoleService`
/// that uses the host operating system's standard library functions.
pub struct HostOsServices;

impl FileSystemService for HostOsServices {
    fn read_to_string(&self, path: &Path) -> Result<String, String> {
        // Delegates to std::fs::read_to_string.
        std::fs::read_to_string(path).map_err(|e| e.to_string())
    }

    fn write_string(&self, path: &Path, content: &str) -> Result<(), String> {
        // Delegates to std::fs::write.
        std::fs::write(path, content).map_err(|e| e.to_string())
    }

    fn list_directory(&self, path: &Path) -> Result<Vec<String>, String> {
        // Delegates to std::fs::read_dir.
        // Converts directory entry names to strings, handling potential non-UTF-8 names.
        std::fs::read_dir(path)
            .map_err(|e| e.to_string())?
            .map(|res| res.map(|e| e.file_name().into_string().unwrap_or_else(|invalid_os_str| {
                // Provide a placeholder for invalid UTF-8 filenames
                format!("Invalid UTF-8 Filename: {:?}", invalid_os_str)
            })))
            .collect::<Result<Vec<String>, _>>() // Collect into Result<Vec<String>, std::io::Error>
            .map_err(|e| e.to_string()) // Map std::io::Error to String
    }

    fn create_directory(&self, path: &Path) -> Result<(), String> {
        // Delegates to std::fs::create_dir_all (like mkdir -p).
        std::fs::create_dir_all(path).map_err(|e| e.to_string())
    }

    fn remove_file(&self, path: &Path) -> Result<(), String> {
        // Delegates to std::fs::remove_file.
        std::fs::remove_file(path).map_err(|e| e.to_string())
    }

    fn remove_directory_recursive(&self, path: &Path) -> Result<(), String> {
        // Delegates to std::fs::remove_dir_all.
        std::fs::remove_dir_all(path).map_err(|e| e.to_string())
    }

    fn current_working_directory(&self) -> Result<PathBuf, String> {
        // Delegates to std::env::current_dir.
        std::env::current_dir().map_err(|e| e.to_string())
    }

    fn set_current_working_directory(&self, path: &Path) -> Result<(), String> {
        // Delegates to std::env::set_current_dir.
        std::env::set_current_dir(path).map_err(|e| e.to_string())
    }

    fn path_exists(&self, path: &Path) -> bool {
        // Delegates to Path::exists.
        path.exists()
    }
}

impl ConsoleService for HostOsServices {
    fn print_line(&self, text: &str) {
        // Delegates to println! macro.
        println!("{}", text);
    }
}
