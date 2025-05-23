# AI Hardware Abstraction Layer (HAL) - Research Notes

This document outlines potential hardware acceleration backends and existing Rust bindings for the AiOS HAL.

## CPU Libraries

*   **OpenBLAS:**
    *   Description: An optimized BLAS (Basic Linear Algebra Subprograms) library. Useful for CPU-bound matrix and vector operations.
    *   Rust Bindings:
        *   [`openblas-src`](https://crates.io/crates/openblas-src): Compiles and links OpenBLAS.
        *   [`blas-src`](https://crates.io/crates/blas-src) with `openblas` feature.
        *   Higher-level libraries like `ndarray` can be configured to use OpenBLAS as a backend.
*   **Intel oneDNN (formerly MKL-DNN):**
    *   Description: Intel's Deep Neural Network Library, optimized for Intel architecture. Provides primitives for deep learning applications.
    *   Rust Bindings:
        *   [`onednn-sys`](https://crates.io/crates/onednn-sys) (low-level bindings, might be less mature).
        *   Search for more active community forks or higher-level abstractions if direct use is complex.
*   **BLIS:**
    *   Description: BLAS-like Library Instantiation Software framework. Another high-performance CPU linear algebra library.
    *   Rust Bindings:
        *   [`blis-src`](https://crates.io/crates/blis-src): Similar to openblas-src.

## GPU Compute

*   **NVIDIA CUDA / TensorRT:**
    *   Description: CUDA is NVIDIA's parallel computing platform and programming model. TensorRT is an SDK for high-performance deep learning inference on NVIDIA GPUs.
    *   Rust Bindings:
        *   [`rustacuda`](https://crates.io/crates/rustacuda): Actively maintained Rust bindings for the CUDA driver API. Allows for writing CUDA kernels and managing GPU memory.
        *   [`cudarc`](https://crates.io/crates/cudarc): Another set of CUDA bindings, aims for safety and ease of use.
        *   TensorRT: Bindings might be less direct. Often, models are compiled to TensorRT plans, and a C++ runtime is used. Rust might interop via C FFI. Search for `tensorrt-rs` or similar.
*   **AMD ROCm / HIP:**
    *   Description: ROCm is AMD's open-source platform for GPU computing. HIP is a C++ runtime API and kernel language that allows developers to create portable applications for AMD and NVIDIA GPUs from a single source code.
    *   Rust Bindings:
        *   Direct HIP bindings for Rust are less common than CUDA. Might require C FFI.
        *   Some projects explore compiling Rust to SPIR-V and then using ROCm's SPIR-V support.
*   **Vulkan Compute:**
    *   Description: Vulkan is a cross-platform graphics and compute API. Its compute shaders can be used for general-purpose GPU programming.
    *   Rust Bindings:
        *   [`vulkano`](https://crates.io/crates/vulkano): Well-regarded, safe, and relatively high-level wrapper around the Vulkan API. Supports compute shaders.
        *   [`ash`](https://crates.io/crates/ash): Unsafe, low-level, and direct Vulkan bindings.
*   **WebGPU (via wgpu-rs):**
    *   Description: WebGPU is an emerging API for graphics and compute, designed for web and native. `wgpu` is a Rust implementation.
    *   Rust Bindings:
        *   [`wgpu`](https://crates.io/crates/wgpu): Can target Vulkan, Metal, DirectX, and eventually WebGPU itself. Provides a modern, Rusty API for GPU compute.

## NPU / Specialized AI Accelerators

*   **Intel OpenVINO / DirectML (on Windows):**
    *   Description: OpenVINO is a toolkit for optimizing and deploying AI inference. DirectML is a low-level API for hardware-accelerated machine learning on Windows.
    *   Rust Bindings:
        *   OpenVINO:
            *   [`openvino-rs`](https://github.com/intel/openvino-rs) (official Intel Rust bindings).
        *   DirectML:
            *   [`windows-rs`](https://crates.io/crates/windows-rs) might provide access to DirectML APIs. Search for specific DirectML examples within its documentation.
*   **Qualcomm SNPE / QNN:**
    *   Description: Snapdragon Neural Processing Engine (SNPE) and Qualcomm AI Engine Direct (QNN) are SDKs for running AI models on Snapdragon platforms.
    *   Rust Bindings:
        *   Likely require C FFI. Official Rust bindings are uncommon for mobile-focused proprietary SDKs. Wrappers would need to be built around the C/C++ libraries.
*   **Google Edge TPU / Coral:**
    *   Description: Edge TPU is Google's ASIC for accelerating ML inference at the edge.
    *   Rust Bindings:
        *   [`libcoral-rs`](https://crates.io/crates/libcoral-rs) (community-driven bindings for libcoral).
        *   May also involve C FFI with the `libedgetpu` library.
*   **Apple Core ML / Neural Engine:**
    *   Description: Core ML is Apple's framework for integrating machine learning models into apps. The Neural Engine is hardware for ML tasks.
    *   Rust Bindings:
        *   [`coreml-rs`](https://crates.io/crates/coreml-rs) (community bindings).
        *   Objective-C or Swift interop might be necessary for full functionality.

## General Notes

*   **FFI (Foreign Function Interface):** For many specialized hardware SDKs, direct Rust bindings may not exist or may be immature. Rust's FFI capabilities (especially with C) are crucial for integrating with existing C/C++ libraries provided by hardware vendors.
*   **ONNX (Open Neural Network Exchange):** Using ONNX as an intermediate model format can be beneficial. Many hardware SDKs support ONNX, and Rust has ONNX runtimes like [`tract-onnx`](https://crates.io/crates/tract-onnx) (CPU-focused but extensible) or can interface with ONNX runtimes that support various EPs (Execution Providers).

This list is not exhaustive but provides a starting point for exploring hardware acceleration options in Rust for AiOS. The maturity and ease of use of bindings can vary significantly.
