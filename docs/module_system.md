# AiOS Module System

The AiOS Module System allows for extending the core functionality of the OS by loading and running custom modules. These modules are currently envisioned as Rust files compiled separately or potentially scripts if the system evolves to support them. Integrity and versioning are managed through a simple blockchain mechanism.

## Module Definition

Currently, modules are external pieces of code that AiOS can interact with. The primary interaction defined is through hashing for registration and a simulated "run" command. A module is typically a single file. For the current implementation, modules are expected to be placed within the `ai_os/modules/` directory.

## Module Installation/Placement

1.  **Create your module:** Develop your module logic. For example, you might write a Rust source file.
2.  **Place the module file:** Copy the compiled binary or script file into the `ai_os/modules/` directory within the AiOS project structure.

    ```
    ai_os/
    ├── modules/
    │   └── my_module.rs  # Or my_module.wasm, my_module.sh, etc.
    ├── src/
    └── ...
    ```

## Module Registration (`register_mod`)

Before a module can be run, it must be registered with the AiOS. Registration involves calculating a SHA256 hash of the module file and storing this hash along with the module's name in a JSON file named `blockchain.json`.

*   **Command:** `register_mod <filename_in_modules_dir>`
*   **Process:**
    1.  The user issues the `register_mod` command, specifying the filename of the module located in `ai_os/modules/`.
    2.  AiOS calculates the SHA256 hash of the specified module file.
    3.  A new entry is created in `ai_os/blockchain.json` containing the module's name and its calculated hash.
*   **Example:**
    ```
    AiOS> register_mod my_module.rs
    Module my_module.rs registered with hash: <sha256_hash_value>
    ```

## Module Execution (`run_mod`)

Running a module first involves an integrity check. The system recalculates the module's current hash and compares it against the hash stored during registration.

*   **Command:** `run_mod <module_name> [args...]`
*   **Process:**
    1.  The user issues the `run_mod` command, specifying the module's name (as registered in `blockchain.json`) and any arguments the module might accept.
    2.  AiOS locates the module file in `ai_os/modules/`.
    3.  It calculates the current SHA256 hash of the module file.
    4.  It looks up the module's registered hash in `blockchain.json`.
    5.  **Integrity Check:**
        *   If the current hash matches the registered hash, the module is considered verified. AiOS then simulates its execution (currently, it prints a message including any provided arguments).
        *   If the hashes do not match, an integrity check failure is reported, and the module is not executed. This indicates the module file may have been altered since registration.
        *   If the module is not found in `blockchain.json`, an error is reported.
*   **Example:**
    ```
    AiOS> run_mod my_module.rs param1 param2
    Module my_module.rs verified. (Simulating execution with args: ["param1", "param2"])

    AiOS> run_mod my_tampered_module.rs
    Module my_tampered_module.rs integrity check failed! Hashes do not match. Expected: <old_hash>, Got: <new_hash>
    ```

## `blockchain.json` Structure

The `blockchain.json` file is a simple JSON array of objects, where each object represents a registered module and its hash. This file is located at the root of the `ai_os` project directory (`ai_os/blockchain.json`).

*   **Format:**
    ```json
    [
      {
        "module_name": "module1.rs",
        "hash": "sha256_hash_for_module1"
      },
      {
        "module_name": "another_module.sh",
        "hash": "sha256_hash_for_another_module"
      }
    ]
    ```
*   **Fields:**
    *   `module_name`: The filename of the module (string). This is the name used with the `run_mod` command.
    *   `hash`: The SHA256 hash of the module file content at the time of registration (string).

## Future Considerations

*   **Actual Module Execution:** The current system only simulates module execution. Future enhancements would involve defining a proper ABI or interface for modules and actually loading and running their code (e.g., using dynamic linking for compiled Rust modules, or an interpreter for scripts).
*   **Dependency Management:** For complex modules.
*   **Permissions Model:** To control what modules can access.
*   **More Robust Blockchain:** The current "blockchain" is a simple JSON log. A true blockchain implementation would be significantly more complex.
