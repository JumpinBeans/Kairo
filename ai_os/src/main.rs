//! AiOS is a simple command-line operating system simulation.
//! It provides basic file system operations, module management, and HAL interactions.

#![warn(missing_docs)] // Enable warnings for missing public documentation

use std::io::{self, Write};
use std::path::Path;
use std::sync::Arc;

mod module_system;
use module_system::{BlockchainEntry, calculate_sha256, read_blockchain_entries, write_blockchain_entries};

mod os_services;
use os_services::{ConsoleService, FileSystemService, HostOsServices};

mod hal;
use hal::{Hal, EmotionCloud};

/// Prompts the user for confirmation (y/N) and returns true if confirmed.
///
/// # Arguments
/// * `prompt_text` - The message to display to the user.
/// * `os_services` - A reference to the console service for printing the prompt.
///
/// This function currently uses direct `std::io::stdin()` for input reading,
/// as `ConsoleService` does not yet define an input method.
fn get_confirmation(prompt_text: &str, os_services: &dyn os_services::ConsoleService) -> bool {
    os_services.print_line(prompt_text);
    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_ok() {
        return input.trim().eq_ignore_ascii_case("y");
    }
    false
}

/// The main entry point for the AiOS application.
///
/// Initializes services, then enters a loop to read user input,
/// parse commands, and dispatch them to appropriate handlers.
fn main() {
    // Initialize OS services (file system, console) and HAL services.
    // These are wrapped in Arc for potential shared ownership if AiOS becomes multi-threaded.
    let os_services = Arc::new(HostOsServices);
    let hal_services = Arc::new(Hal::new());

    // Main command loop
    loop {
        // Display the AiOS prompt.
        // Note: `print_line` adds a newline. A more traditional prompt might not.
        os_services.print_line("AiOS> ");
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before reading input.

        let mut input = String::new();
        // Read user input.
        if io::stdin().read_line(&mut input).is_err() {
            os_services.print_line("Error reading input"); // Use os_services for error output
            continue;
        }

        // Prepare input for parsing: trim whitespace.
        let input = input.trim();
        if input.is_empty() {
            continue; // Skip empty input.
        }

        // Parse the command and its arguments.
        // The first word is the command, the rest are arguments.
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or(""); // Default to empty string if no command.
        let args: Vec<&str> = parts.collect();

        // Command dispatcher
        match command {
            // Core OS commands
            "exit" => {
                os_services.print_line("Exiting AiOS...");
                break; // Exit the main loop, terminating the application.
            }
            "echo" => {
                // Prints arguments to the console.
                if args.is_empty() {
                    os_services.print_line(""); // Echo nothing if no arguments.
                } else {
                    os_services.print_line(&args.join(" ")); // Join arguments with spaces.
                }
            }
            "help" => {
                // Displays help information for all commands.
                os_services.print_line("Available commands:");
                os_services.print_line("  help                                - Shows this help message.");
                os_services.print_line("  echo [args...]                      - Prints the arguments to the console.");
                os_services.print_line("  clear                               - Clears the terminal screen.");
                os_services.print_line("  ls [path]                           - Lists directory contents (defaults to current directory).");
                os_services.print_line("  cd <directory>                      - Changes the current working directory.");
                os_services.print_line("  pwd                                 - Prints the current working directory.");
                os_services.print_line("  mkdir <directory_path>              - Creates a new directory (including parent directories).");
                os_services.print_line("  rm [-r|--recursive] <path1> ...     - Removes files or directories (use -r for directories).");
                os_services.print_line("  cat <file1> [file2...]              - Displays the content of one or more files.");
                os_services.print_line("  register_mod <filename>             - Calculates hash of a module file in 'modules/' and adds it to the blockchain.");
                os_services.print_line("  run_mod <name> [args...]            - Verifies and 'runs' a registered module from 'modules/'.");
                os_services.print_line("  emotion_test <text_input...>        - Analyzes emotional context of the input text.");
                os_services.print_line("  celestial_add_cloud <id> <x> ...    - Adds an emotion cloud (see help for full args).");
                os_services.print_line("  celestial_list_clouds               - Lists all stored emotion clouds.");
                os_services.print_line("  exit                                - Exits the AiOS application.");
            }
            "clear" => {
                // Clears the terminal screen.
                // The specific ANSI escape sequence "\x1B[2J\x1B[H" clears the screen and moves the cursor to the home position.
                // This might be better handled inside HostOsServices if it could detect terminal capabilities
                // or if the `clearscreen` crate was used directly by the `HostOsServices` implementation.
                os_services.print_line("\x1B[2J\x1B[H");
            }
            // Module System commands
            "register_mod" => {
                // Registers a module by calculating its hash and storing it in blockchain.json.
                if args.len() != 1 {
                    os_services.print_line("Usage: register_mod <module_filename>");
                    continue;
                }
                let module_filename = args[0];
                let module_path_str = format!("ai_os/modules/{}", module_filename);
                let module_path = Path::new(&module_path_str);

                if !os_services.path_exists(module_path) {
                     os_services.print_line(&format!("Error: Module file '{}' not found.", module_path.display()));
                    continue;
                }

                match calculate_sha256(os_services.as_ref(), &module_path_str) {
                    Ok(hash) => {
                        let mut entries = read_blockchain_entries(os_services.as_ref());
                        let new_entry = BlockchainEntry {
                            module_name: module_filename.to_string(),
                            hash: hash.clone(),
                        };
                        entries.push(new_entry);
                        match write_blockchain_entries(os_services.as_ref(), &entries) {
                            Ok(_) => os_services.print_line(&format!("Module {} registered with hash: {}", module_filename, hash)),
                            Err(e) => os_services.print_line(&format!("Error writing to blockchain: {}", e)),
                        }
                    }
                    Err(e) => {
                        os_services.print_line(&format!("Error calculating hash for {}: {}", module_filename, e));
                    }
                }
            }
            "cat" => {
                if args.is_empty() {
                    os_services.print_line("Usage: cat <file1> [<file2>...]");
                    continue;
                }

                let print_header = args.len() > 1;

                for (i, file_path_str) in args.iter().enumerate() {
                    let file_path = Path::new(file_path_str);

                    if !os_services.path_exists(file_path) {
                        os_services.print_line(&format!("cat: {}: No such file or directory", file_path_str));
                        continue;
                    }

                    // Using std::fs metadata to check if it's a file.
                    // Similar to rm, this is for metadata before using os_services for the actual read.
                    let is_file = match std::fs::metadata(file_path) {
                        Ok(metadata) => metadata.is_file(),
                        Err(e) => {
                            os_services.print_line(&format!("cat: {}: {}", file_path_str, e));
                            continue;
                        }
                    };

                    if !is_file {
                        os_services.print_line(&format!("cat: {}: Is not a file (e.g., it's a directory)", file_path_str));
                        continue;
                    }

                    if print_header {
                        os_services.print_line(&format!("--- {} ---", file_path_str));
                    }

                    match os_services.read_to_string(file_path) {
                        Ok(content) => {
                            // os_services.print_line already adds a newline.
                            // If the content itself has a trailing newline, this might result in double newlines.
                            // For a typical cat, we might print line by line or print the content as is.
                            // For now, we'll print the whole content with print_line.
                            // If content ends with \n, print_line will add another, so strip it.
                            if content.ends_with('\n') {
                                os_services.print_line(content.strip_suffix('\n').unwrap_or(&content));
                            } else {
                                os_services.print_line(&content);
                            }
                        }
                        Err(e) => {
                            os_services.print_line(&format!("cat: {}: {}", file_path_str, e));
                        }
                    }
                    if print_header && i < args.len() - 1 { // Add a newline between files if headers are printed
                        os_services.print_line("");
                    }
                }
            }
            "rm" => {
                if args.is_empty() {
                    os_services.print_line("Usage: rm [-r|--recursive] <path1> [<path2>...]");
                    continue;
                }

                let mut recursive = false;
                let mut paths_to_remove = Vec::new();

                for arg in args {
                    if arg == "-r" || arg == "--recursive" {
                        recursive = true;
                    } else {
                        paths_to_remove.push(arg);
                    }
                }

                if paths_to_remove.is_empty() {
                    os_services.print_line("rm: missing operand");
                    continue;
                }

                for path_str in paths_to_remove {
                    let path = Path::new(path_str);
                    if !os_services.path_exists(path) {
                        os_services.print_line(&format!("rm: cannot remove '{}': No such file or directory", path_str));
                        continue;
                    }

                    // Using std::fs::symlink_metadata to check if it's a dir without following symlinks
                    // This is a direct std::fs call, but it's for metadata checking.
                    // The actual removal operations will use os_services.
                    let is_dir = match std::fs::symlink_metadata(path) {
                        Ok(metadata) => metadata.is_dir(),
                        Err(e) => {
                            os_services.print_line(&format!("rm: cannot access '{}': {}", path_str, e));
                            continue;
                        }
                    };

                    if recursive {
                        if is_dir {
                            let prompt = format!("Recursively remove directory '{}'? (y/N): ", path_str);
                            if get_confirmation(&prompt, os_services.as_ref()) {
                                match os_services.remove_directory_recursive(path) {
                                    Ok(_) => { /* os_services.print_line(&format!("Removed directory '{}'", path_str)); */ }
                                    Err(e) => os_services.print_line(&format!("rm: cannot remove directory '{}': {}", path_str, e)),
                                }
                            } else {
                                os_services.print_line(&format!("Not removing directory '{}'", path_str));
                            }
                        } else { // recursive and it's a file
                            match os_services.remove_file(path) {
                                Ok(_) => { /* os_services.print_line(&format!("Removed file '{}'", path_str)); */ }
                                Err(e) => os_services.print_line(&format!("rm: cannot remove file '{}': {}", path_str, e)),
                            }
                        }
                    } else { // not recursive
                        if is_dir {
                            os_services.print_line(&format!("rm: cannot remove '{}': Is a directory. Use -r to remove directories.", path_str));
                        } else {
                            match os_services.remove_file(path) {
                                Ok(_) => { /* os_services.print_line(&format!("Removed file '{}'", path_str)); */ }
                                Err(e) => os_services.print_line(&format!("rm: cannot remove file '{}': {}", path_str, e)),
                            }
                        }
                    }
                }
            }
            "mkdir" => {
                if args.len() != 1 {
                    os_services.print_line("Usage: mkdir <directory_path>");
                    continue;
                }
                let dir_path_str = args[0];
                let dir_path = Path::new(dir_path_str);

                // Optional: Check if path already exists and is a file
                if os_services.path_exists(dir_path) && dir_path.is_file() {
                     os_services.print_line(&format!("mkdir: cannot create directory '{}': File exists", dir_path_str));
                    continue;
                }
                // os_services.create_directory uses create_dir_all, so it's fine if parts of the path already exist as directories.

                match os_services.create_directory(dir_path) {
                    Ok(_) => {
                        // os_services.print_line(&format!("Directory '{}' created.", dir_path_str)); // Optional success message
                    }
                    Err(e) => {
                        os_services.print_line(&format!("mkdir: cannot create directory '{}': {}", dir_path_str, e));
                    }
                }
            }
            "pwd" => {
                if !args.is_empty() {
                    os_services.print_line("Usage: pwd (no arguments)");
                    continue;
                }
                match os_services.current_working_directory() {
                    Ok(cwd) => {
                        os_services.print_line(&cwd.display().to_string());
                    }
                    Err(e) => {
                        os_services.print_line(&format!("pwd: error getting current directory: {}", e));
                    }
                }
            }
            "cd" => {
                if args.len() != 1 {
                    os_services.print_line("Usage: cd <directory>");
                    continue;
                }
                let target_dir_str = args[0];
                let target_dir = Path::new(target_dir_str);

                if !os_services.path_exists(target_dir) {
                    os_services.print_line(&format!("cd: no such file or directory: {}", target_dir_str));
                    continue;
                }
                // The FileSystemService does not currently have an `is_directory` method.
                // We could add one, or rely on set_current_working_directory to fail if it's not a directory.
                // For now, we proceed and let set_current_working_directory handle it.
                // std::env::set_current_dir will fail if the path is not a directory.

                match os_services.set_current_working_directory(target_dir) {
                    Ok(_) => {
                        // Optionally, print the new CWD. For now, silent on success.
                        // os_services.print_line(&format!("New CWD: {}", os_services.current_working_directory().unwrap_or_default().display()));
                    }
                    Err(e) => {
                        os_services.print_line(&format!("cd: {}: {}", target_dir_str, e));
                    }
                }
            }
            "run_mod" => {
                // "Runs" a module after verifying its integrity against the blockchain.
                if args.is_empty() {
                    os_services.print_line("Usage: run_mod <module_name> [args...]");
                    continue;
                }
                let module_name = args[0];
                let module_args = &args[1..];
                let module_path_str = format!("ai_os/modules/{}", module_name);
                let module_path = Path::new(&module_path_str);


                if !os_services.path_exists(module_path) {
                    os_services.print_line(&format!("Error: Module {} not found at path {}", module_name, module_path.display()));
                    continue;
                }

                match calculate_sha256(os_services.as_ref(), &module_path_str) {
                    Ok(current_hash) => {
                        let entries = read_blockchain_entries(os_services.as_ref());
                        match entries.iter().find(|&entry| entry.module_name == module_name) {
                            Some(entry) => {
                                if entry.hash == current_hash {
                                    os_services.print_line(&format!("Module {} verified. (Simulating execution with args: {:?})", module_name, module_args));
                                } else {
                                    os_services.print_line(&format!("Module {} integrity check failed! Hashes do not match. Expected: {}, Got: {}", module_name, entry.hash, current_hash));
                                }
                            }
                            None => {
                                os_services.print_line(&format!("Module {} not registered in blockchain.", module_name));
                            }
                        }
                    }
                    Err(e) => {
                        os_services.print_line(&format!("Error calculating hash for {}: {}", module_name, e));
                    }
                }
            }
            "emotion_test" => {
                if args.is_empty() {
                    os_services.print_line("Usage: emotion_test <text_input...>");
                    continue;
                }
                let text_input = args.join(" ");
                match hal_services.emotional_engine.analyze_emotional_context(&text_input) {
                    Ok(output) => {
                        os_services.print_line(&format!("Emotional Analysis: Primary: {}, Intensity: {}", output.primary_emotion, output.intensity));
                    }
                    Err(e) => {
                        os_services.print_line(&format!("Error analyzing emotion: {}", e));
                    }
                }
            }
            "celestial_add_cloud" => {
                if args.len() != 10 {
                    os_services.print_line("Usage: celestial_add_cloud <id> <x> <y> <z> <r> <g> <b> <a> <intensity> <shape>");
                    os_services.print_line("Example: celestial_add_cloud cloud1 0.5 1.2 0.8 255 0 0 255 0.9 joyful_sphere");
                    continue;
                }
                let id = args[0].to_string();
                let pos_x = args[1].parse::<f32>();
                let pos_y = args[2].parse::<f32>();
                let pos_z = args[3].parse::<f32>();
                let color_r = args[4].parse::<u8>();
                let color_g = args[5].parse::<u8>();
                let color_b = args[6].parse::<u8>();
                let color_a = args[7].parse::<u8>();
                let intensity = args[8].parse::<f32>();
                let shape = args[9].to_string();

                if vec![pos_x.as_ref(), pos_y.as_ref(), pos_z.as_ref(), intensity.as_ref()].iter().any(|x| x.is_err()) ||
                   vec![color_r.as_ref(), color_g.as_ref(), color_b.as_ref(), color_a.as_ref()].iter().any(|x| x.is_err()) {
                    os_services.print_line("Error: Invalid number format for position, color, or intensity.");
                    continue;
                }

                let cloud = EmotionCloud {
                    id,
                    position: [pos_x.unwrap(), pos_y.unwrap(), pos_z.unwrap()],
                    color: [color_r.unwrap(), color_g.unwrap(), color_b.unwrap(), color_a.unwrap()],
                    intensity: intensity.unwrap(),
                    shape_description: shape,
                };

                match hal_services.celestial_memory.lock() {
                    Ok(mut memory) => {
                        match memory.store_emotion_cloud(cloud) {
                            Ok(_) => os_services.print_line("Emotion cloud stored."),
                            Err(e) => os_services.print_line(&format!("Error storing cloud: {}", e)),
                        }
                    }
                    Err(e) => os_services.print_line(&format!("Failed to lock celestial memory: {}", e)),
                }
            }
            "ls" => {
                let target_path_str = if args.is_empty() {
                    match os_services.current_working_directory() {
                        Ok(cwd) => cwd.to_string_lossy().to_string(),
                        Err(e) => {
                            os_services.print_line(&format!("Error getting current directory: {}", e));
                            continue;
                        }
                    }
                } else {
                    args[0].to_string()
                };
                let target_path = Path::new(&target_path_str);

                if !os_services.path_exists(target_path) {
                    os_services.print_line(&format!("ls: cannot access '{}': No such file or directory", target_path.display()));
                    continue;
                }
                
                // Check if it's a directory. list_directory should ideally handle this,
                // but an explicit check can give a better error message if it's a file.
                // For now, we rely on list_directory's error.

                match os_services.list_directory(target_path) {
                    Ok(entries) => {
                        for entry in entries {
                            os_services.print_line(&entry);
                        }
                    }
                    Err(e) => {
                        // Check if target_path is a file, if so, just print its name
                        // The FileSystemService's list_directory might return an error if it's not a directory.
                        // A more robust `ls` might check path.is_file() and print the filename if it is.
                        // For now, we'll just print the error from list_directory.
                        // If it's a file, std::fs::read_dir returns an error like "Not a directory (os error 20)"
                        if target_path.is_file() { // requires std::path::Path, not os_services.path_is_file
                             os_services.print_line(&target_path.file_name().unwrap_or_default().to_string_lossy());
                        } else {
                            os_services.print_line(&format!("ls: error listing '{}': {}", target_path.display(), e));
                        }
                    }
                }
            }
            "celestial_list_clouds" => {
                 match hal_services.celestial_memory.lock() {
                    Ok(memory) => {
                        let clouds = memory.list_emotion_clouds();
                        if clouds.is_empty() {
                            os_services.print_line("No emotion clouds stored.");
                        } else {
                            os_services.print_line("Stored Emotion Clouds:");
                            for cloud in clouds {
                                os_services.print_line(&format!(
                                    "- ID: {}, Pos: [{:.2}, {:.2}, {:.2}], Color: [{},{},{},{}], Intensity: {:.2}, Shape: {}",
                                    cloud.id, cloud.position[0], cloud.position[1], cloud.position[2],
                                    cloud.color[0], cloud.color[1], cloud.color[2], cloud.color[3],
                                    cloud.intensity, cloud.shape_description
                                ));
                            }
                        }
                    }
                    Err(e) => os_services.print_line(&format!("Failed to lock celestial memory: {}", e)),
                }
            }
            _ => {
                os_services.print_line(&format!("Unknown command: {}", command));
            }
        }
    }
}
