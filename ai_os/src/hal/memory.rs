//! Defines traits and structures for the Celestial Model Memory within the HAL.
//!
//! This module provides conceptual data structures like `EmotionCloud` and `ResonantNode`,
//! and a trait `CelestialModelMemory` for managing them. This specialized memory
//! is intended for AiOS's unique emotional modeling capabilities.

use std::collections::HashMap;

/// Represents an "Emotion Cloud" in the Celestial Model Memory.
///
/// Emotion Clouds store various attributes related to an emotional state or concept,
/// including its spatial representation, appearance, and nature.
#[derive(Debug, Clone)]
pub struct EmotionCloud {
    /// Unique identifier for the Emotion Cloud.
    pub id: String,
    /// 3D spatial coordinates [x, y, z].
    pub position: [f32; 3],
    /// Color represented as RGBA (Red, Green, Blue, Alpha).
    pub color: [u8; 4],
    /// Intensity of the emotion or concept, e.g., on a 0.0 to 1.0 scale.
    pub intensity: f32,
    /// Textual description of the cloud's shape or form.
    pub shape_description: String,
}

/// Represents a "Resonant Node" in the Celestial Model Memory.
///
/// Resonant Nodes are conceptual points that can be linked to Emotion Clouds
/// and may point to more detailed memory data.
#[derive(Debug, Clone)]
pub struct ResonantNode {
    /// Unique identifier for the Resonant Node.
    pub id: String,
    /// 3D spatial coordinates [x, y, z].
    pub position: [f32; 3],
    /// A list of IDs of `EmotionCloud`s that this node is related to.
    pub related_emotion_cloud_ids: Vec<String>,
    /// A placeholder string that might represent a pointer or key to more detailed memory data.
    pub memory_data_pointer: String,
}

/// Trait defining operations for the Celestial Model Memory.
///
/// This memory system is designed to store and manage `EmotionCloud` and `ResonantNode`
/// instances. Due to its expected mutable nature, implementations are typically
/// wrapped in `Arc<Mutex<...>>`, hence the `Send + Sync` requirement.
pub trait CelestialModelMemory: Send + Sync {
    // --- Emotion Cloud Operations ---

    /// Stores a new `EmotionCloud`.
    /// Returns an error if an `EmotionCloud` with the same ID already exists.
    fn store_emotion_cloud(&mut self, cloud: EmotionCloud) -> Result<(), String>;
    /// Retrieves an `EmotionCloud` by its ID. Returns `None` if not found.
    /// The `EmotionCloud` is returned by value (cloned) to simplify ownership with Mutex.
    fn retrieve_emotion_cloud(&self, id: &str) -> Option<EmotionCloud>;
    /// Lists all currently stored `EmotionCloud`s.
    /// Returns a vector of cloned `EmotionCloud`s.
    fn list_emotion_clouds(&self) -> Vec<EmotionCloud>;
    /// Updates an existing `EmotionCloud`.
    /// Returns an error if the `EmotionCloud` with the given ID is not found.
    fn update_emotion_cloud(&mut self, cloud_update: EmotionCloud) -> Result<(), String>;
    /// Removes an `EmotionCloud` by its ID.
    /// Returns an error if the `EmotionCloud` with the given ID is not found.
    fn remove_emotion_cloud(&mut self, id: &str) -> Result<(), String>;

    // --- Resonant Node Operations ---

    /// Stores a new `ResonantNode`.
    /// Returns an error if a `ResonantNode` with the same ID already exists.
    fn store_resonant_node(&mut self, node: ResonantNode) -> Result<(), String>;
    /// Retrieves a `ResonantNode` by its ID. Returns `None` if not found.
    /// The `ResonantNode` is returned by value (cloned).
    fn retrieve_resonant_node(&self, id: &str) -> Option<ResonantNode>;
    /// Lists all currently stored `ResonantNode`s.
    /// Returns a vector of cloned `ResonantNode`s.
    fn list_resonant_nodes(&self) -> Vec<ResonantNode>;
    /// Updates an existing `ResonantNode`.
    /// Returns an error if the `ResonantNode` with the given ID is not found.
    fn update_resonant_node(&mut self, node_update: ResonantNode) -> Result<(), String>;
    /// Removes a `ResonantNode` by its ID.
    /// Returns an error if the `ResonantNode` with the given ID is not found.
    fn remove_resonant_node(&mut self, id: &str) -> Result<(), String>;
}

/// A simulated implementation of `CelestialModelMemory`.
///
/// This implementation uses `HashMap`s to store `EmotionCloud`s and `ResonantNode`s
/// in memory. It serves as a basic, functional backend for the Celestial Model Memory.
pub struct SimulatedCelestialMemory {
    emotion_clouds: HashMap<String, EmotionCloud>,
    resonant_nodes: HashMap<String, ResonantNode>,
}

impl SimulatedCelestialMemory {
    /// Creates a new, empty `SimulatedCelestialMemory` instance.
    pub fn new() -> Self {
        Self {
            emotion_clouds: HashMap::new(),
            resonant_nodes: HashMap::new(),
        }
    }
}

impl Default for SimulatedCelestialMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl CelestialModelMemory for SimulatedCelestialMemory {
    // --- Emotion Clouds ---
    fn store_emotion_cloud(&mut self, cloud: EmotionCloud) -> Result<(), String> {
        if self.emotion_clouds.contains_key(&cloud.id) {
            return Err(format!("EmotionCloud with id '{}' already exists.", cloud.id));
        }
        self.emotion_clouds.insert(cloud.id.clone(), cloud);
        Ok(())
    }

    fn retrieve_emotion_cloud(&self, id: &str) -> Option<EmotionCloud> {
        self.emotion_clouds.get(id).cloned() // `.cloned()` is used for `Option<&T> -> Option<T>` where T is Clone
    }

    fn list_emotion_clouds(&self) -> Vec<EmotionCloud> {
        self.emotion_clouds.values().cloned().collect() // `.cloned()` for iterators of references
    }

    fn update_emotion_cloud(&mut self, cloud_update: EmotionCloud) -> Result<(), String> {
        if !self.emotion_clouds.contains_key(&cloud_update.id) {
            return Err(format!("EmotionCloud with id '{}' not found for update.", cloud_update.id));
        }
        // `insert` will overwrite if key exists, which is the desired update behavior.
        self.emotion_clouds.insert(cloud_update.id.clone(), cloud_update);
        Ok(())
    }

    fn remove_emotion_cloud(&mut self, id: &str) -> Result<(), String> {
        if self.emotion_clouds.remove(id).is_none() {
            // `remove` returns None if the key was not present.
            return Err(format!("EmotionCloud with id '{}' not found for removal.", id));
        }
        Ok(())
    }

    // --- Resonant Nodes ---
    fn store_resonant_node(&mut self, node: ResonantNode) -> Result<(), String> {
        if self.resonant_nodes.contains_key(&node.id) {
            return Err(format!("ResonantNode with id '{}' already exists.", node.id));
        }
        self.resonant_nodes.insert(node.id.clone(), node);
        Ok(())
    }

    fn retrieve_resonant_node(&self, id: &str) -> Option<ResonantNode> {
        self.resonant_nodes.get(id).cloned()
    }

    fn list_resonant_nodes(&self) -> Vec<ResonantNode> {
        self.resonant_nodes.values().cloned().collect()
    }

    fn update_resonant_node(&mut self, node_update: ResonantNode) -> Result<(), String> {
        if !self.resonant_nodes.contains_key(&node_update.id) {
            return Err(format!("ResonantNode with id '{}' not found for update.", node_update.id));
        }
        self.resonant_nodes.insert(node_update.id.clone(), node_update);
        Ok(())
    }

    fn remove_resonant_node(&mut self, id: &str) -> Result<(), String> {
        if self.resonant_nodes.remove(id).is_none() {
            return Err(format!("ResonantNode with id '{}' not found for removal.", id));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_retrieve_emotion_cloud() {
        let mut memory = SimulatedCelestialMemory::new();
        let cloud = EmotionCloud {
            id: "cloud1".to_string(),
            position: [1.0, 2.0, 3.0],
            color: [255, 0, 0, 255],
            intensity: 0.75,
            shape_description: "sphere".to_string(),
        };

        assert!(memory.store_emotion_cloud(cloud.clone()).is_ok());
        
        let retrieved = memory.retrieve_emotion_cloud("cloud1");
        assert!(retrieved.is_some());
        let retrieved_cloud = retrieved.unwrap();
        assert_eq!(retrieved_cloud.id, cloud.id);
        assert_eq!(retrieved_cloud.position, cloud.position);
        assert_eq!(retrieved_cloud.color, cloud.color);
        assert_eq!(retrieved_cloud.intensity, cloud.intensity);
        assert_eq!(retrieved_cloud.shape_description, cloud.shape_description);

        // Test retrieving non-existent cloud
        assert!(memory.retrieve_emotion_cloud("non_existent_cloud").is_none());
    }

    #[test]
    fn test_list_update_remove_emotion_clouds() {
        let mut memory = SimulatedCelestialMemory::new();
        let cloud1 = EmotionCloud { id: "c1".to_string(), position: [0.0; 3], color: [0; 4], intensity: 0.1, shape_description: "s1".to_string() };
        let cloud2 = EmotionCloud { id: "c2".to_string(), position: [1.0; 3], color: [1; 4], intensity: 0.2, shape_description: "s2".to_string() };
        
        memory.store_emotion_cloud(cloud1.clone()).unwrap();
        memory.store_emotion_cloud(cloud2.clone()).unwrap();

        let clouds = memory.list_emotion_clouds();
        assert_eq!(clouds.len(), 2);
        // Note: HashMap iteration order is not guaranteed, so check for presence instead of order
        assert!(clouds.iter().any(|c| c.id == "c1"));
        assert!(clouds.iter().any(|c| c.id == "c2"));

        let mut updated_cloud1 = cloud1.clone();
        updated_cloud1.intensity = 0.9;
        assert!(memory.update_emotion_cloud(updated_cloud1.clone()).is_ok());
        let retrieved_updated = memory.retrieve_emotion_cloud("c1").unwrap();
        assert_eq!(retrieved_updated.intensity, 0.9);

        assert!(memory.remove_emotion_cloud("c1").is_ok());
        assert!(memory.retrieve_emotion_cloud("c1").is_none());
        assert_eq!(memory.list_emotion_clouds().len(), 1);

        // Test removing non-existent
        assert!(memory.remove_emotion_cloud("c_non_existent").is_err());
         // Test updating non-existent
        let non_existent_update = EmotionCloud { id: "c_non_existent".to_string(), position: [0.0; 3], color: [0; 4], intensity: 0.1, shape_description: "s_ne".to_string() };
        assert!(memory.update_emotion_cloud(non_existent_update).is_err());


    }


    #[test]
    fn test_store_retrieve_resonant_node() {
        let mut memory = SimulatedCelestialMemory::new();
        let node = ResonantNode {
            id: "node1".to_string(),
            position: [4.0, 5.0, 6.0],
            related_emotion_cloud_ids: vec!["cloud1".to_string()],
            memory_data_pointer: "ptr_to_data_123".to_string(),
        };

        assert!(memory.store_resonant_node(node.clone()).is_ok());

        let retrieved = memory.retrieve_resonant_node("node1");
        assert!(retrieved.is_some());
        let retrieved_node = retrieved.unwrap();
        assert_eq!(retrieved_node.id, node.id);
        assert_eq!(retrieved_node.position, node.position);
        assert_eq!(retrieved_node.related_emotion_cloud_ids, node.related_emotion_cloud_ids);
        assert_eq!(retrieved_node.memory_data_pointer, node.memory_data_pointer);
        
        assert!(memory.retrieve_resonant_node("non_existent_node").is_none());
    }
    
    #[test]
    fn test_list_update_remove_resonant_nodes() {
        let mut memory = SimulatedCelestialMemory::new();
        let node1 = ResonantNode { id: "n1".to_string(), position: [0.0; 3], related_emotion_cloud_ids: vec![], memory_data_pointer: "p1".to_string() };
        let node2 = ResonantNode { id: "n2".to_string(), position: [1.0; 3], related_emotion_cloud_ids: vec!["c1".to_string()], memory_data_pointer: "p2".to_string() };
        
        memory.store_resonant_node(node1.clone()).unwrap();
        memory.store_resonant_node(node2.clone()).unwrap();

        let nodes = memory.list_resonant_nodes();
        assert_eq!(nodes.len(), 2);
        assert!(nodes.iter().any(|n| n.id == "n1"));
        assert!(nodes.iter().any(|n| n.id == "n2"));


        let mut updated_node1 = node1.clone();
        updated_node1.memory_data_pointer = "p1_updated".to_string();
        assert!(memory.update_resonant_node(updated_node1.clone()).is_ok());
        let retrieved_updated = memory.retrieve_resonant_node("n1").unwrap();
        assert_eq!(retrieved_updated.memory_data_pointer, "p1_updated");

        assert!(memory.remove_resonant_node("n1").is_ok());
        assert!(memory.retrieve_resonant_node("n1").is_none());
        assert_eq!(memory.list_resonant_nodes().len(), 1);
        
        // Test removing non-existent
        assert!(memory.remove_resonant_node("n_non_existent").is_err());
        // Test updating non-existent
        let non_existent_update = ResonantNode { id: "n_non_existent".to_string(), position: [0.0; 3], related_emotion_cloud_ids: vec![], memory_data_pointer: "p_ne".to_string() };
        assert!(memory.update_resonant_node(non_existent_update).is_err());

    }
}
