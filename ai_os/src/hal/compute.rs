//! Defines traits and structures for compute device management within the HAL.
//! This includes identifying different types of compute devices (CPU, GPU, NPU)
//! and querying their capabilities.

/// Represents the type of a compute device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    /// Central Processing Unit.
    CPU,
    /// Graphics Processing Unit.
    GPU,
    /// Neural Processing Unit (specialized AI accelerator).
    NPU,
}

/// Trait for a compute device.
///
/// Implementations of this trait provide information about a specific compute device,
/// such as its name, type, and capabilities.
/// `Send` and `Sync` are required for safe sharing across threads, common with `Arc`.
pub trait ComputeDevice: Send + Sync {
    /// Returns the name of the compute device.
    fn name(&self) -> String;
    /// Returns the type of the compute device (e.g., CPU, GPU).
    fn device_type(&self) -> DeviceType;
    /// Returns a string describing the capabilities of the device.
    /// This is a simple representation; more detailed capability reporting might be
    /// implemented in the future.
    fn capabilities(&self) -> String;
}

/// A concrete implementation representing a CPU device.
#[derive(Debug, Clone)]
pub struct CpuDevice {
    name: String,
    capabilities_info: String,
}

impl Default for CpuDevice {
    /// Creates a default `CpuDevice` instance.
    fn default() -> Self {
        Self {
            name: "Generic CPU".to_string(),
            capabilities_info: "Basic x86_64 support".to_string(),
        }
    }
}

impl ComputeDevice for CpuDevice {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::CPU
    }

    fn capabilities(&self) -> String {
        self.capabilities_info.clone()
    }
}

/// Trait for a compute service.
///
/// A compute service is responsible for enumerating and potentially managing
/// available compute devices.
/// `Send` and `Sync` are required for safe sharing via `Arc`.
pub trait ComputeService: Send + Sync {
    /// Lists all available compute devices.
    ///
    /// Returns a vector of `Box<dyn ComputeDevice>`, allowing for different concrete
    /// device types.
    fn list_devices(&self) -> Vec<Box<dyn ComputeDevice>>;
    // Future methods could include selecting a device, allocating resources, etc.
}

/// A basic implementation of the `ComputeService`.
///
/// This implementation currently simulates a single CPU device.
pub struct BasicComputeService {
    devices: Vec<Box<dyn ComputeDevice>>,
}

impl BasicComputeService {
    /// Creates a new `BasicComputeService`.
    ///
    /// Initializes with a default CPU device.
    pub fn new() -> Self {
        let cpu = CpuDevice {
            name: "Host CPU".to_string(),
            capabilities_info: "General purpose computation".to_string(),
        };
        Self {
            devices: vec![Box::new(cpu)],
        }
    }
}

impl Default for BasicComputeService {
    /// Creates a default `BasicComputeService`.
    fn default() -> Self {
        Self::new()
    }
}

impl ComputeService for BasicComputeService {
    fn list_devices(&self) -> Vec<Box<dyn ComputeDevice>> {
        // The current implementation of listing devices involves creating new Boxed devices
        // based on the stored ones. This is a simplified approach.
        // In a more complex scenario, devices might be Arcs or references if cloning is expensive
        // or if maintaining device state identity is crucial.
        // Since `CpuDevice` is `Clone`, we can effectively "clone" the boxed device.
        self.devices.iter().map(|device_box| {
            // This map operation effectively clones the device information into a new Box.
            // It assumes that the display name and capabilities are sufficient to represent
            // the device in this context. If `ComputeDevice` itself was `Clone`,
            // a more direct clone of the Box content might be possible, though `Box<dyn Trait>`
            // is not `Clone` directly.
            Box::new(CpuDevice { name: device_box.name(), capabilities_info: device_box.capabilities() }) as Box<dyn ComputeDevice>
        }).collect()
    }
}
