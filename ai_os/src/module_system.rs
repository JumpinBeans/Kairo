use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io; // Keep for specific ErrorKinds if needed, but prefer String errors from FileSystemService
use std::path::Path;
use crate::os_services::FileSystemService; // Import the trait

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockchainEntry {
    pub module_name: String,
    pub hash: String,
}

pub fn calculate_sha256(fs_service: &dyn FileSystemService, filepath_str: &str) -> Result<String, String> {
    // FileSystemService expects &Path, so we convert the &str to &Path
    let path = Path::new(filepath_str);
    // Reading as bytes, as the original fs::read did. FileSystemService might need a read_bytes method for this.
    // For now, assuming read_to_string is acceptable if files are text, or that calculate_sha256 will adapt.
    // The prompt used fs::read which returns Vec<u8>. Our trait has read_to_string.
    // This is a slight mismatch. For SHA256, we need bytes.
    // Let's adjust FileSystemService or this function.
    // Short-term: If we must use read_to_string, this implies modules must be UTF-8.
    // Long-term: FileSystemService should have a `read_bytes` method.
    // For now, we will proceed with read_to_string and note this limitation.
    let content_str = fs_service.read_to_string(path)?;
    let mut hasher = Sha256::new();
    hasher.update(content_str.as_bytes()); // hashing the string's bytes
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

// Function to read blockchain entries from blockchain.json
pub fn read_blockchain_entries(fs_service: &dyn FileSystemService) -> Vec<BlockchainEntry> {
    let blockchain_path = Path::new("ai_os/blockchain.json");
    match fs_service.read_to_string(blockchain_path) {
        Ok(data) => {
            if data.trim().is_empty() {
                return Vec::new();
            }
            match serde_json::from_str::<Vec<BlockchainEntry>>(&data) {
                Ok(entries) => entries,
                Err(e) => {
                    eprintln!("Warning: Could not parse blockchain.json: {}. Starting with an empty blockchain.", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            // Check if the error indicates "file not found"
            // This is a bit tricky as FileSystemService returns String errors.
            // We might need more structured errors or a way to check error kinds.
            // For now, we assume any read error means we should try to initialize.
            // A better approach would be for FileSystemService.read_to_string to return a custom error enum.
            if !fs_service.path_exists(blockchain_path) {
                 // If file not found, initialize with empty list
                match fs_service.write_string(blockchain_path, "[]") {
                    Ok(_) => println!("Initialized empty blockchain.json."),
                    Err(write_err) => eprintln!("Error creating blockchain.json: {}", write_err),
                }
            } else {
                 eprintln!("Warning: Could not read blockchain.json: {}. Starting with an empty blockchain.", e);
            }
            Vec::new()
        }
    }
}

// Function to write blockchain entries to blockchain.json
pub fn write_blockchain_entries(fs_service: &dyn FileSystemService, entries: &Vec<BlockchainEntry>) -> Result<(), String> {
    let json_data = serde_json::to_string_pretty(entries).map_err(|e| e.to_string())?;
    fs_service.write_string(Path::new("ai_os/blockchain.json"), &json_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_services::{HostOsServices, FileSystemService}; // Using HostOsServices for tests
    use std::fs::{self, File};
    use std::io::Write;

    // Helper function to create a temporary file with specific content
    fn create_temp_file(path_str: &str, content: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(path_str)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
    
    // Helper function to remove a file, ignoring errors if it doesn't exist
    fn remove_temp_file(path_str: &str) {
        let _ = fs::remove_file(path_str); // Ignore result, best effort cleanup
    }

    #[test]
    fn test_calculate_sha256_simple() {
        let fs_service = HostOsServices;
        let test_file_path = "test_file_for_hash.txt"; // Relative to project root where tests run
        let content = "hello world";
        
        // Expected SHA256 hash for "hello world"
        // You can calculate this using an online tool or `echo -n "hello world" | sha256sum`
        let expected_hash = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";

        create_temp_file(test_file_path, content).expect("Failed to create temp file for hash test");

        match calculate_sha256(&fs_service, test_file_path) {
            Ok(hash) => {
                assert_eq!(hash, expected_hash);
            }
            Err(e) => {
                panic!("calculate_sha256 failed: {}", e);
            }
        }
        remove_temp_file(test_file_path);
    }

    #[test]
    fn test_blockchain_entry_json_serialization_deserialization() {
        let original_entry = BlockchainEntry {
            module_name: "test_module.rs".to_string(),
            hash: "abcdef1234567890".to_string(),
        };

        // Serialize to JSON
        let json_string = serde_json::to_string(&original_entry).expect("Serialization failed");

        // Deserialize back from JSON
        let deserialized_entry: BlockchainEntry = serde_json::from_str(&json_string).expect("Deserialization failed");

        // Assert that the deserialized version matches the original
        assert_eq!(original_entry.module_name, deserialized_entry.module_name);
        assert_eq!(original_entry.hash, deserialized_entry.hash);
    }

    #[test]
    fn test_read_write_blockchain_entries_empty() {
        let fs_service = HostOsServices;
        let entries: Vec<BlockchainEntry> = Vec::new();
        let test_blockchain_path = "test_blockchain_empty.json";
        let blockchain_file_path_obj = Path::new(test_blockchain_path);

        // Clean up before test, in case a previous run failed
        let _ = fs_service.remove_file(blockchain_file_path_obj);


        // Test writing empty entries
        assert!(write_blockchain_entries(&fs_service, &entries).is_ok());
        assert!(fs_service.path_exists(blockchain_file_path_obj));

        // Test reading empty entries
        let read_entries = read_blockchain_entries(&fs_service);
        assert!(read_entries.is_empty());

        // Test reading empty entries when file content is "[]"
        fs_service.write_string(blockchain_file_path_obj, "[]").unwrap();
        let read_entries_again = read_blockchain_entries(&fs_service);
        assert!(read_entries_again.is_empty());
        
        // Test reading empty entries when file content is "" (empty string)
        fs_service.write_string(blockchain_file_path_obj, "").unwrap();
        let read_entries_empty_str = read_blockchain_entries(&fs_service);
        assert!(read_entries_empty_str.is_empty());

        // Clean up
        let _ = fs_service.remove_file(blockchain_file_path_obj);
    }
    
    #[test]
    fn test_read_write_blockchain_entries_with_data() {
        let fs_service = HostOsServices;
        let mut entries: Vec<BlockchainEntry> = Vec::new();
        entries.push(BlockchainEntry { module_name: "mod1.rs".to_string(), hash: "hash1".to_string() });
        entries.push(BlockchainEntry { module_name: "mod2.py".to_string(), hash: "hash2".to_string() });
        
        let test_blockchain_path = "test_blockchain_data.json";
        let blockchain_file_path_obj = Path::new(test_blockchain_path);

        // Clean up before test
        let _ = fs_service.remove_file(blockchain_file_path_obj);

        // Test writing entries
        assert!(write_blockchain_entries(&fs_service, &entries).is_ok());

        // Test reading entries
        let read_entries = read_blockchain_entries(&fs_service);
        assert_eq!(read_entries.len(), 2);
        assert_eq!(read_entries[0].module_name, "mod1.rs");
        assert_eq!(read_entries[0].hash, "hash1");
        assert_eq!(read_entries[1].module_name, "mod2.py");
        assert_eq!(read_entries[1].hash, "hash2");

        // Clean up
        let _ = fs_service.remove_file(blockchain_file_path_obj);
    }

    #[test]
    fn test_read_blockchain_entries_file_not_found() {
        let fs_service = HostOsServices;
        // Ensure the test file does not exist
        let non_existent_path = "test_blockchain_non_existent.json";
        let path_obj = Path::new(non_existent_path);
        let _ = fs_service.remove_file(path_obj); // Attempt to remove if it exists from a failed run

        // read_blockchain_entries should create an empty file and return an empty Vec
        let entries = read_blockchain_entries(&fs_service);
        assert!(entries.is_empty());
        // Check that the file was created and is empty or "[]"
        assert!(fs_service.path_exists(path_obj));
        let content = fs_service.read_to_string(path_obj).unwrap_or_default();
        assert!(content == "[]" || content.trim().is_empty());

        // Clean up
        let _ = fs_service.remove_file(path_obj);
    }
}
