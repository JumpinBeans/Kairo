//! The Hardware Abstraction Layer (HAL) for AiOS.
//!
//! This module provides a set of traits and basic implementations for interacting
//! with hardware components, particularly those relevant to AI and specialized computation.
//! It aims to abstract away the specifics of underlying hardware, allowing AiOS
//! to be more portable and extensible.
//!
//! Key components include:
//! - `compute`: For managing and utilizing computational devices (CPU, GPU, NPU).
//! - `tensor`: For tensor operations, crucial for AI/ML tasks.
//! - `ai`: For higher-level AI functionalities like emotional reasoning.
//! - `memory`: For specialized memory models, such as the Celestial Model Memory.
//!
//! The `Hal` struct acts as a central service locator for accessing these HAL functionalities.

// Sub-modules for HAL components
pub mod compute;
pub mod tensor;
pub mod ai;
pub mod memory;

// Re-export key traits and structs for easier access from outside the HAL module.
pub use compute::{ComputeDevice, DeviceType, ComputeService};
pub use tensor::{Tensor, DataType, TensorOperations};
pub use ai::{EmotionalReasoningEngine, EmotionalOutput};
pub use memory::{CelestialModelMemory, EmotionCloud, ResonantNode};

use std::sync::{Arc, Mutex};

// Import concrete implementations to be used in the default `Hal` struct.
// These are basic, simulated, or CPU-bound implementations.
use compute::BasicComputeService;
use tensor::CpuTensorOperations;
use ai::MyEmotionalReasoningEngine;
use memory::SimulatedCelestialMemory;

/// The main Hardware Abstraction Layer (HAL) service locator struct.
///
/// This struct provides centralized access to all HAL services. Services are
/// typically exposed as trait objects wrapped in `Arc` for shared ownership
/// and `Mutex` for interior mutability where needed (e.g., `CelestialModelMemory`).
pub struct Hal {
    /// Provides access to compute device management and information.
    pub compute_service: Arc<dyn ComputeService>,
    /// Provides tensor operations capabilities.
    pub tensor_operations: Arc<dyn TensorOperations>,
    /// Provides emotional reasoning capabilities.
    pub emotional_engine: Arc<dyn EmotionalReasoningEngine>,
    /// Provides access to the Celestial Model Memory, which requires mutable access
    /// and is therefore wrapped in a Mutex.
    pub celestial_memory: Arc<Mutex<dyn CelestialModelMemory>>,
}

impl Hal {
    /// Creates a new `Hal` instance with default service implementations.
    ///
    /// The default implementations are basic and may be simulated or CPU-bound.
    /// This constructor is used to initialize the HAL for AiOS.
    pub fn new() -> Self {
        Self {
            compute_service: Arc::new(BasicComputeService::new()),
            tensor_operations: Arc::new(CpuTensorOperations::new()),
            emotional_engine: Arc::new(MyEmotionalReasoningEngine::new()),
            celestial_memory: Arc::new(Mutex::new(SimulatedCelestialMemory::new())),
        }
    }
}

impl Default for Hal {
    /// Provides a default `Hal` instance.
    /// This is equivalent to calling `Hal::new()`.
    fn default() -> Self {
        Self::new()
    }
}
