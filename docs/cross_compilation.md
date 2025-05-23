# Cross-Compilation for Raspberry Pi

This document outlines the steps to set up cross-compilation for Raspberry Pi OS (both 32-bit and 64-bit versions) from a development machine.

## 1. Supported Target Triples

The primary Rust target triples for Raspberry Pi OS are:

*   **32-bit Raspberry Pi OS (e.g., on Raspberry Pi 2, 3, 4 with a 32-bit OS):**
    *   `armv7-unknown-linux-gnueabihf`
*   **64-bit Raspberry Pi OS (e.g., on Raspberry Pi 3, 4, Zero 2 W with a 64-bit OS):**
    *   `aarch64-unknown-linux-gnu`

We will focus on these `gnu` (glibc-based) targets. For fully static binaries, `musl` variants like `armv7-unknown-linux-musleabihf` or `aarch64-unknown-linux-musl` could be considered.

## 2. Toolchain Setup

### 2.1. Install Rust Targets

First, add the desired target triples to your Rust toolchain using `rustup`:

```bash
rustup target add armv7-unknown-linux-gnueabihf
rustup target add aarch64-unknown-linux-gnu
```

### 2.2. Install Linkers

Cross-compilation requires a linker compatible with the target architecture. The specific package names can vary based on your development operating system.

**On Debian/Ubuntu (using `apt`):**

*   For `armv7-unknown-linux-gnueabihf`:
    ```bash
    sudo apt update
    sudo apt install gcc-arm-linux-gnueabihf
    ```
*   For `aarch64-unknown-linux-gnu`:
    ```bash
    sudo apt update
    sudo apt install gcc-aarch64-linux-gnu
    ```

**On other systems:**

*   **macOS (using Homebrew):** You might need to install a generic ARM toolchain. For example, for `aarch64`:
    ```bash
    brew tap ArmMbed/homebrew-formulae
    brew install arm-none-eabi-gcc # Generic, might need more specific or osx-cross/homebrew-arm
    # Specific linkers for linux targets on macOS can be complex.
    # Often, Docker or a Linux VM is used for more robust cross-compilation from macOS to Linux.
    # For aarch64-linux-gnu, you might look for `aarch64-linux-gnu-gcc` via custom taps or pre-built toolchains.
    ```
    A common tool for macOS is `zig cc`, which can simplify cross-compilation for C dependencies.
*   **Windows:** You can install ARM GCC toolchains from [Arm Developer](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-a/downloads). Ensure the linker is added to your PATH. Windows Subsystem for Linux (WSL) with Debian/Ubuntu is also a viable option.

*Note: The exact linker package names and availability can change. Always refer to your system's package manager or toolchain provider.*

### 2.3. Configure Cargo

Create or edit the Cargo configuration file at `.cargo/config.toml` in your project's root directory (for project-specific configuration) or in your Cargo home directory (e.g., `~/.cargo/config.toml` for global configuration).

For the `ai_os` project, this file is located at `ai_os/.cargo/config.toml`.

Add the following lines to specify the linker for each target:

```toml
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

If your linker executables have different names or are not in your PATH, you'll need to adjust the `linker` value accordingly (e.g., provide the full path).

### 2.4. Important Considerations (Glibc Version)

When dynamically linking against `glibc` (as with the `gnu` targets), ensure that the `glibc` version on your target Raspberry Pi is compatible with (ideally the same or newer than) the one used by the cross-compilation toolchain. Mismatches can lead to runtime errors. This is less of an issue with Rust itself, which statically links most of its dependencies, but can become relevant if your project links against C libraries.

Using MUSL targets (`*-musl`) can avoid glibc compatibility issues by producing more statically linked binaries, but they may have other limitations (e.g., DNS resolution, compatibility with some C libraries that expect glibc features).

## 3. Build Commands and Outcome

The following commands are used to cross-compile the AiOS project (executed from within the `ai_os` project directory):

*   **For 32-bit ARM (Raspberry Pi OS):**
    ```bash
    cargo build --target=armv7-unknown-linux-gnueabihf
    ```

*   **For 64-bit ARM (Raspberry Pi OS):**
    ```bash
    cargo build --target=aarch64-unknown-linux-gnu
    ```

### Outcome:

The cross-compilation attempts were **successful** for both target triples.

*   **`armv7-unknown-linux-gnueabihf`:** The build completed successfully. The binary can be found at `target/armv7-unknown-linux-gnueabihf/debug/ai_os`.
*   **`aarch64-unknown-linux-gnu`:** The build completed successfully. The binary can be found at `target/aarch64-unknown-linux-gnu/debug/ai_os`.

Several warnings related to unused code were present during compilation. These are expected as the project is under active development and some features or parts of the HAL are not yet fully utilized by the main application logic. No errors related to cross-compilation itself (e.g., linking issues or incompatible dependencies) were encountered.

The setup using `rustup` for target management, `apt` for linker installation (on a Debian/Ubuntu-based environment), and Cargo's `.cargo/config.toml` for linker specification worked as expected.
This indicates that the current dependencies (`clearscreen`, `serde`, `serde_json`, `sha2`) are compatible with these ARM Linux targets.
