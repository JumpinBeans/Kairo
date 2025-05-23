# AiOS CLI Commands

This document provides a reference for all available Command Line Interface (CLI) commands in AiOS.

## General Syntax

Commands are typically entered in the format: `command_name [argument1] [argument2] ...`

Arguments are space-separated. Paths should be specified according to the host operating system's conventions.

## Core Commands

### `help`

*   **Purpose:** Displays a list of all available commands and their brief descriptions.
*   **Syntax:** `help`
*   **Example:**
    ```
    AiOS> help
    ```

### `echo [args...]`

*   **Purpose:** Prints the provided arguments back to the console.
*   **Syntax:** `echo [argument1] [argument2] ...`
*   **Example:**
    ```
    AiOS> echo Hello World
    Hello World
    AiOS> echo
    
    ```

### `clear`

*   **Purpose:** Clears the terminal screen.
*   **Syntax:** `clear`
*   **Example:**
    ```
    AiOS> clear
    ```

### `exit`

*   **Purpose:** Terminates the AiOS application.
*   **Syntax:** `exit`
*   **Example:**
    ```
    AiOS> exit
    Exiting AiOS...
    ```

## File System Commands

### `ls [path]`

*   **Purpose:** Lists the contents of a specified directory. If no path is provided, lists the contents of the current working directory.
*   **Syntax:** `ls [directory_path]`
*   **Examples:**
    ```
    AiOS> ls
    file1.txt
    my_folder
    AiOS> ls /app/ai_os/modules
    some_module.rs
    ```

### `cd <directory>`

*   **Purpose:** Changes the current working directory to the specified directory.
*   **Syntax:** `cd <directory_path>`
*   **Example:**
    ```
    AiOS> cd /app/ai_os/docs
    AiOS> pwd
    /app/ai_os/docs
    ```

### `pwd`

*   **Purpose:** Prints the full path of the current working directory.
*   **Syntax:** `pwd`
*   **Example:**
    ```
    AiOS> pwd
    /app/ai_os
    ```

### `mkdir <directory_path>`

*   **Purpose:** Creates a new directory at the specified path. It can create parent directories if they do not exist (similar to `mkdir -p`).
*   **Syntax:** `mkdir <new_directory_path>`
*   **Example:**
    ```
    AiOS> mkdir /app/new_project/src
    ```

### `rm [-r|--recursive] <path1> [<path2>...]`

*   **Purpose:** Removes specified files or directories.
*   **Syntax:** `rm [-r|--recursive] <path_to_remove_1> [<path_to_remove_2> ...]`
*   **Options:**
    *   `-r` or `--recursive`: Required to remove directories. When used, it will recursively delete the directory and its contents. A confirmation prompt will be shown for recursive deletions.
*   **Examples:**
    ```
    AiOS> rm my_file.txt
    AiOS> rm -r old_project_folder
    Recursively remove directory 'old_project_folder'? (y/N): y
    ```

### `cat <file1> [<file2>...]`

*   **Purpose:** Displays the content of one or more specified files.
*   **Syntax:** `cat <file_path_1> [<file_path_2> ...]`
*   **Example:**
    ```
    AiOS> cat /app/ai_os/src/main.rs
    (Content of main.rs is displayed)
    AiOS> cat file1.txt file2.txt
    --- file1.txt ---
    (Content of file1.txt)

    --- file2.txt ---
    (Content of file2.txt)
    ```

## Module System Commands

### `register_mod <filename>`

*   **Purpose:** Calculates the SHA256 hash of a module file located in the `ai_os/modules/` directory and records its name and hash in the `ai_os/blockchain.json` file.
*   **Syntax:** `register_mod <module_filename_in_modules_dir>`
*   **Example:**
    ```
    AiOS> register_mod my_module.rs
    Module my_module.rs registered with hash: <hash_value>
    ```

### `run_mod <name> [args...]`

*   **Purpose:** Verifies the integrity of a registered module by comparing its current hash with the one stored in `blockchain.json`. If verified, it simulates the execution of the module.
*   **Syntax:** `run_mod <module_name_as_in_blockchain> [module_argument1] ...`
*   **Example:**
    ```
    AiOS> run_mod my_module.rs arg1 arg2
    Module my_module.rs verified. (Simulating execution with args: ["arg1", "arg2"])
    ```

## HAL (Hardware Abstraction Layer) Commands

### `emotion_test <text_input...>`

*   **Purpose:** Analyzes the emotional context of the provided text input using the HAL's Emotional Reasoning Engine.
*   **Syntax:** `emotion_test <text_to_analyze>`
*   **Example:**
    ```
    AiOS> emotion_test This is a very happy day!
    Emotional Analysis: Primary: joy, Intensity: 0.8
    ```

### `celestial_add_cloud <id> <x> <y> <z> <r> <g> <b> <a> <intensity> <shape>`

*   **Purpose:** Adds a new "Emotion Cloud" to the HAL's Celestial Model Memory.
*   **Syntax:** `celestial_add_cloud <id_string> <pos_x_float> <pos_y_float> <pos_z_float> <color_r_u8> <color_g_u8> <color_b_u8> <color_a_u8> <intensity_float> <shape_string>`
*   **Example:**
    ```
    AiOS> celestial_add_cloud cloud_joy_1 10.5 -2.3 5.0 255 255 0 200 0.95 "bright_sphere"
    Emotion cloud stored.
    ```

### `celestial_list_clouds`

*   **Purpose:** Lists all Emotion Clouds currently stored in the HAL's Celestial Model Memory.
*   **Syntax:** `celestial_list_clouds`
*   **Example:**
    ```
    AiOS> celestial_list_clouds
    Stored Emotion Clouds:
    - ID: cloud_joy_1, Pos: [10.50, -2.30, 5.00], Color: [255,255,0,200], Intensity: 0.95, Shape: bright_sphere
    ```
