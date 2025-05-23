//! Defines tensor structures and operations for the HAL.
//!
//! Tensors are fundamental data structures for AI and machine learning, representing
//! multi-dimensional arrays. This module provides a `Tensor` struct, data type
//! definitions, and a trait for tensor operations.

use std::mem;

/// Enumerates the possible data types for tensor elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    /// 32-bit floating point.
    F32,
    /// 32-bit signed integer.
    I32,
    /// 8-bit unsigned integer.
    U8,
}

impl DataType {
    /// Returns the size in bytes of the `DataType`.
    fn size_of(&self) -> usize {
        match self {
            DataType::F32 => mem::size_of::<f32>(),
            DataType::I32 => mem::size_of::<i32>(),
            DataType::U8 => mem::size_of::<u8>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tensor_zeros() {
        let ops = CpuTensorOperations::new();
        let dimensions = vec![2, 3];
        let data_type = DataType::F32;

        let tensor = ops.create_tensor_zeros(dimensions.clone(), data_type.clone()).unwrap();

        assert_eq!(tensor.dimensions, dimensions);
        assert_eq!(tensor.data_type, data_type);
        
        let num_elements: usize = dimensions.iter().product();
        let expected_byte_size = num_elements * data_type.size_of();
        assert_eq!(tensor.data.len(), expected_byte_size);

        // Check if all data is zero
        for byte in tensor.data.iter() {
            assert_eq!(*byte, 0);
        }
    }

    #[test]
    fn test_tensor_as_slice_f32() {
        let dimensions = vec![2, 2];
        let data_type = DataType::F32;
        let data_f32: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        
        // Convert Vec<f32> to Vec<u8>
        let data_u8: Vec<u8> = unsafe {
            let mut temp_data_f32 = data_f32.clone(); // Clone to make it mutable for as_mut_ptr
            let ptr = temp_data_f32.as_mut_ptr();
            let len = temp_data_f32.len() * mem::size_of::<f32>();
            let cap = temp_data_f32.capacity() * mem::size_of::<f32>();
            mem::forget(temp_data_f32);
            Vec::from_raw_parts(ptr as *mut u8, len, cap)
        };

        let tensor = Tensor {
            dimensions,
            data_type,
            data: data_u8,
        };

        let slice_result = tensor.as_slice::<f32>();
        assert!(slice_result.is_ok());
        let slice = slice_result.unwrap();
        assert_eq!(slice, &data_f32[..]);
    }


    #[test]
    fn test_add_f32_tensors() {
        let ops = CpuTensorOperations::new();
        let dim = vec![2, 2];

        // Tensor A: [[1.0, 2.0], [3.0, 4.0]]
        let data_a_f32: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        let data_a_u8: Vec<u8> = unsafe {
            let mut temp_data = data_a_f32.clone();
            let ptr = temp_data.as_mut_ptr();
            let len = temp_data.len() * mem::size_of::<f32>();
            let cap = temp_data.capacity() * mem::size_of::<f32>();
            mem::forget(temp_data);
            Vec::from_raw_parts(ptr as *mut u8, len, cap)
        };
        let tensor_a = Tensor {
            dimensions: dim.clone(),
            data_type: DataType::F32,
            data: data_a_u8,
        };

        // Tensor B: [[5.0, 6.0], [7.0, 8.0]]
        let data_b_f32: Vec<f32> = vec![5.0, 6.0, 7.0, 8.0];
        let data_b_u8: Vec<u8> = unsafe {
            let mut temp_data = data_b_f32.clone();
            let ptr = temp_data.as_mut_ptr();
            let len = temp_data.len() * mem::size_of::<f32>();
            let cap = temp_data.capacity() * mem::size_of::<f32>();
            mem::forget(temp_data);
            Vec::from_raw_parts(ptr as *mut u8, len, cap)
        };
        let tensor_b = Tensor {
            dimensions: dim.clone(),
            data_type: DataType::F32,
            data: data_b_u8,
        };

        let result_tensor = ops.add(&tensor_a, &tensor_b).unwrap();

        assert_eq!(result_tensor.dimensions, dim);
        assert_eq!(result_tensor.data_type, DataType::F32);

        let result_slice: &[f32] = result_tensor.as_slice::<f32>().unwrap();
        let expected_result_f32: Vec<f32> = vec![6.0, 8.0, 10.0, 12.0];
        assert_eq!(result_slice, &expected_result_f32[..]);
    }

    #[test]
    fn test_add_mismatched_dimensions() {
        let ops = CpuTensorOperations::new();
        let tensor_a = ops.create_tensor_zeros(vec![2, 2], DataType::F32).unwrap();
        let tensor_b = ops.create_tensor_zeros(vec![2, 3], DataType::F32).unwrap();
        
        let result = ops.add(&tensor_a, &tensor_b);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Tensor dimensions must match for addition.");
    }

    #[test]
    fn test_add_mismatched_data_types() {
        let ops = CpuTensorOperations::new();
        // Note: DataType::I32 addition is not implemented, but type check should be first.
        // For this test to be more robust, we'd need another implemented type or mock.
        // However, the type check is explicit.
        let tensor_a = ops.create_tensor_zeros(vec![2, 2], DataType::F32).unwrap();
        // Simulate a tensor with a different data type for the check
        let tensor_b_data_u8 = vec![0; 2 * 2 * DataType::I32.size_of()];
        let tensor_b = Tensor {
            dimensions: vec![2,2],
            data_type: DataType::I32, // Different data type
            data: tensor_b_data_u8
        };
        
        let result = ops.add(&tensor_a, &tensor_b);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Tensor data types must match for addition.");
    }
}

/// Represents a multi-dimensional array (tensor).
///
/// Tensors store their shape (dimensions), the data type of their elements,
/// and the actual data in a raw byte buffer (`Vec<u8>`).
#[derive(Debug, Clone)]
pub struct Tensor {
    /// The dimensions of the tensor (e.g., `vec![2, 3, 4]` for a 2x3x4 tensor).
    pub dimensions: Vec<usize>,
    /// The data type of the elements stored in the tensor.
    pub data_type: DataType,
    /// The raw byte buffer containing the tensor data. Data is stored contiguously.
    pub data: Vec<u8>,
}

impl Tensor {
    /// Creates a new tensor filled with zeros.
    ///
    /// # Arguments
    /// * `dimensions` - A vector defining the shape of the tensor.
    /// * `data_type` - The `DataType` of the elements in the tensor.
    pub fn new_zeros(dimensions: Vec<usize>, data_type: DataType) -> Self {
        let num_elements: usize = dimensions.iter().product();
        let byte_size = num_elements * data_type.size_of();
        Self {
            dimensions,
            data_type,
            data: vec![0; byte_size], // Initialize with zeros.
        }
    }

    /// Calculates the total number of elements in the tensor.
    fn num_elements(&self) -> usize {
        self.dimensions.iter().product()
    }

    /// Returns a slice view of the tensor's data interpreted as type `T`.
    ///
    /// # Safety
    /// This method is unsafe because it performs a raw pointer cast. The caller must ensure
    /// that the type `T` is compatible with the tensor's `data_type` and that the
    /// data buffer's alignment and size are correct for `T`.
    ///
    /// # Returns
    /// A `Result` containing a slice `&[T]` or an error string if preconditions are not met.
    pub fn as_slice<T>(&self) -> Result<&[T], String> {
        // Basic check for size compatibility. Alignment is harder to check here robustly
        // without knowing more about T and the original allocation.
        if self.data.len() % mem::size_of::<T>() != 0 {
            return Err("Data size is not a multiple of the type size.".to_string());
        }
        // This relies on the data being correctly aligned for type T.
        Ok(unsafe {
            std::slice::from_raw_parts(self.data.as_ptr() as *const T, self.data.len() / mem::size_of::<T>())
        })
    }
    
    /// Returns a mutable slice view of the tensor's data interpreted as type `T`.
    ///
    /// # Safety
    /// Similar to `as_slice`, this method is unsafe due to raw pointer casting.
    /// The caller must ensure type compatibility and correct alignment/size.
    ///
    /// # Returns
    /// A `Result` containing a mutable slice `&mut [T]` or an error string.
    pub fn as_mut_slice<T>(&mut self) -> Result<&mut [T], String> {
        if self.data.len() % mem::size_of::<T>() != 0 {
            return Err("Data size is not a multiple of the type size.".to_string());
        }
        // Relies on data being correctly aligned for type T.
        Ok(unsafe {
             std::slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.data.len() / mem::size_of::<T>())
        })
    }
}

/// Trait defining common tensor operations.
///
/// Implementations of this trait provide specific backends for tensor computations
/// (e.g., CPU-based, GPU-based).
/// `Send` and `Sync` are required for safe sharing via `Arc`.
pub trait TensorOperations: Send + Sync {
    /// Adds two tensors element-wise.
    ///
    /// # Arguments
    /// * `tensor_a` - The first tensor.
    /// * `tensor_b` - The second tensor.
    ///
    /// # Returns
    /// A `Result` containing the resulting `Tensor` or an error string if the operation fails
    /// (e.g., due to incompatible shapes or data types).
    fn add(&self, tensor_a: &Tensor, tensor_b: &Tensor) -> Result<Tensor, String>;

    /// Creates a new tensor of specified dimensions and data type, initialized with zeros.
    fn create_tensor_zeros(&self, dimensions: Vec<usize>, data_type: DataType) -> Result<Tensor, String>;
}

/// A basic CPU-bound implementation of `TensorOperations`.
pub struct CpuTensorOperations;

impl CpuTensorOperations {
    /// Creates a new `CpuTensorOperations` instance.
    pub fn new() -> Self { Self }
}

impl Default for CpuTensorOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl TensorOperations for CpuTensorOperations {
    fn create_tensor_zeros(&self, dimensions: Vec<usize>, data_type: DataType) -> Result<Tensor, String> {
        Ok(Tensor::new_zeros(dimensions, data_type))
    }

    fn add(&self, tensor_a: &Tensor, tensor_b: &Tensor) -> Result<Tensor, String> {
        // Check for dimension and data type compatibility.
        if tensor_a.dimensions != tensor_b.dimensions {
            return Err("Tensor dimensions must match for addition.".to_string());
        }
        if tensor_a.data_type != tensor_b.data_type {
            return Err("Tensor data types must match for addition.".to_string());
        }

        match tensor_a.data_type {
            DataType::F32 => {
                // Get typed slices from the raw byte buffers.
                let slice_a: &[f32] = tensor_a.as_slice::<f32>()?;
                let slice_b: &[f32] = tensor_b.as_slice::<f32>()?;
                
                // Ensure the number of elements matches (should be guaranteed by dimension check).
                if slice_a.len() != slice_b.len() {
                     return Err("Internal data length mismatch despite same dimensions.".to_string());   
                }

                // Perform element-wise addition.
                let mut result_data_f32: Vec<f32> = Vec::with_capacity(slice_a.len());
                for i in 0..slice_a.len() {
                    result_data_f32.push(slice_a[i] + slice_b[i]);
                }
                
                // Convert the resulting Vec<f32> back to Vec<u8> for the new Tensor.
                // This involves unsafe operations to reinterpret the memory.
                let result_data_u8: Vec<u8> = unsafe {
                    // Ensure the original Vec<f32> does not deallocate the buffer.
                    let ptr = result_data_f32.as_mut_ptr();
                    let len = result_data_f32.len() * mem::size_of::<f32>();
                    let cap = result_data_f32.capacity() * mem::size_of::<f32>();
                    mem::forget(result_data_f32); // Prevent Vec<f32> from dropping the data.
                    // Create a new Vec<u8> from the raw parts.
                    Vec::from_raw_parts(ptr as *mut u8, len, cap)
                };

                Ok(Tensor {
                    dimensions: tensor_a.dimensions.clone(),
                    data_type: tensor_a.data_type.clone(),
                    data: result_data_u8,
                })
            }
            // TODO: Implement addition for other data types (I32, U8).
            _ => Err("Addition for this data type is not yet implemented.".to_string()),
        }
    }
}
