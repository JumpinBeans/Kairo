# AiOS - Local Setup and Testing Guide

This guide explains how to set up the AiOS application locally on your Windows machine (like a Surface Pro 11) and perform some basic tests.

## 1. Install Rust

AiOS is built with Rust. If you don't have Rust installed, you'll need to install it first.

*   Go to the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
*   Follow the instructions to download and run `rustup-init.exe` for Windows.
*   Choose the default installation options.
*   Once installed, open a new Command Prompt or PowerShell window and verify the installation:
    ```sh
    rustc --version
    cargo --version
    ```
    You should see versions reported for both `rustc` (the Rust compiler) and `cargo` (the Rust package manager). A recent stable version is recommended.

## 2. Obtain the AiOS Source Code

The AiOS source code is managed in a Git repository.

*   **Install Git:** If you don't have Git, download it from [https://git-scm.com/download/win](https://git-scm.com/download/win) and install it.
*   **Clone the Repository:**
    Open a Command Prompt or PowerShell and navigate to where you want to store the AiOS project. Then run:
    ```sh
    git clone <repository_url> ai_os 
    ```
    (Replace `<repository_url>` with the actual URL of the AiOS Git repository. This URL will be provided once the project is hosted.)
    This will create an `ai_os` directory containing the project files.

    Alternatively, if a source code archive (e.g., a `.zip` file) is provided, download and extract it.

## 3. Build the AiOS Application

*   Navigate to the `ai_os` project directory in your terminal:
    ```sh
    cd ai_os
    ```
*   Build the application using Cargo:
    *   For a debug build (faster compilation, for development):
        ```sh
        cargo build
        ```
    *   For a release build (optimized, runs faster, but slower to compile):
        ```sh
        cargo build --release
        ```
*   The executable will be located at:
    *   Debug: `target/debug/ai_os.exe`
    *   Release: `target/release/ai_os.exe`

## 4. Run the AiOS Application

You can run AiOS in a couple of ways from the project's root directory (`ai_os`):

*   **Using `cargo run`** (compiles if necessary, then runs the debug version):
    ```sh
    cargo run
    ```
*   **Directly executing the compiled file:**
    *   Debug:
        ```sh
        target\debug\ai_os.exe
        ```
    *   Release:
        ```sh
        target\release\ai_os.exe
        ```

You should see the AiOS command prompt: `AiOS> `

## 5. Basic Testing & Interaction

Here are some commands you can try to ensure AiOS is working as expected:

*   **Show Help:**
    ```
    AiOS> help
    ```
    This lists all available commands.

*   **File System Operations:**
    ```
    AiOS> pwd
    AiOS> ls
    AiOS> mkdir my_test_folder
    AiOS> ls
    AiOS> cd my_test_folder
    AiOS> pwd
    ```
    To test `cat`, you'll first need to create a file in `my_test_folder`. Since AiOS doesn't yet have a built-in command to create file content directly (like `echo "text" > file.txt`), you can create a sample file using your host Windows system (e.g., with Notepad or PowerShell) inside the `ai_os/my_test_folder/` directory. Let's say you create `example.txt` with some text in it.
    ```
    AiOS> ls 
    AiOS> cat example.txt
    AiOS> cd ..
    AiOS> rm my_test_folder/example.txt 
    AiOS> rm -r my_test_folder 
    ```
    (Confirm with 'y' when prompted by `rm -r`)
    ```
    AiOS> ls
    ```

*   **Module System:**
    1.  Using Windows File Explorer or a text editor, navigate to the `ai_os/modules/` directory.
    2.  Create a new text file named `sample_mod.rs`.
    3.  Open `sample_mod.rs` and paste the following simple Rust code into it:
        ```rust
        // This is just a sample. AiOS currently only hashes this file, it doesn't compile/run it.
        pub fn module_function() {
            // Dummy function
        }
        ```
    4.  Save the file.
    5.  In AiOS, register and run the module:
        ```
        AiOS> register_mod sample_mod.rs
        AiOS> run_mod sample_mod.rs
        ```
        It should report successful verification.
    6.  Now, open `ai_os/modules/sample_mod.rs` again and make a small change (e.g., add a comment `// My comment`). Save it.
    7.  In AiOS, try running it again:
        ```
        AiOS> run_mod sample_mod.rs
        ```
        It should now report an integrity check failure.

*   **HAL (AI & Celestial Memory) Commands:**
    ```
    AiOS> emotion_test This is a happy test.
    AiOS> emotion_test This is a sad situation.
    AiOS> celestial_add_cloud c1 1.0 2.0 3.0 255 0 0 200 0.9 "test_cloud"
    AiOS> celestial_list_clouds
    AiOS> celestial_add_cloud c2 4.0 5.0 6.0 0 255 0 180 0.7 "another_cloud"
    AiOS> celestial_list_clouds 
    ```
    (You can try other parameters for `celestial_add_cloud` as well.)

## 6. Exiting AiOS

To close the AiOS application, type:
```
AiOS> exit
```

This guide should help you get started with testing AiOS on your local machine. If you encounter issues, ensure your Rust installation is up-to-date and that you've correctly cloned/downloaded all project files.
