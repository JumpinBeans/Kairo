//! Defines traits and structures for AI-related functionalities within the HAL,
//! focusing on emotional reasoning.

/// Represents the output of an emotional analysis.
///
/// Contains the identified primary emotion and its intensity.
#[derive(Debug, Clone)]
pub struct EmotionalOutput {
    /// The primary emotion identified (e.g., "joy", "sorrow", "anger").
    pub primary_emotion: String,
    /// The intensity of the identified emotion, typically on a scale (e.g., 0.0 to 1.0).
    pub intensity: f32,
}

/// Trait for an emotional reasoning engine.
///
/// Implementations of this trait can analyze input (e.g., text) and determine
/// an emotional context.
/// `Send` and `Sync` are required for safe sharing via `Arc`.
pub trait EmotionalReasoningEngine: Send + Sync {
    /// Analyzes the emotional context of a given text input.
    ///
    /// # Arguments
    /// * `text_input` - The text string to analyze.
    ///
    /// # Returns
    /// A `Result` containing an `EmotionalOutput` struct or an error string
    /// if the analysis fails.
    fn analyze_emotional_context(&self, text_input: &str) -> Result<EmotionalOutput, String>;
}

/// A basic implementation of the `EmotionalReasoningEngine`.
///
/// This engine performs a simple keyword-based analysis to determine emotions.
pub struct MyEmotionalReasoningEngine;

impl MyEmotionalReasoningEngine {
    /// Creates a new `MyEmotionalReasoningEngine` instance.
    pub fn new() -> Self { Self }
}

impl Default for MyEmotionalReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EmotionalReasoningEngine for MyEmotionalReasoningEngine {
    /// Analyzes text for simple keywords to determine a primary emotion and intensity.
    ///
    /// The analysis is case-insensitive.
    fn analyze_emotional_context(&self, text_input: &str) -> Result<EmotionalOutput, String> {
        let lower_input = text_input.to_lowercase(); // Convert to lowercase for case-insensitive matching.

        if lower_input.contains("happy") || lower_input.contains("joy") {
            Ok(EmotionalOutput {
                primary_emotion: "joy".to_string(),
                intensity: 0.8, // Predefined intensity for "joy"
            })
        } else if lower_input.contains("sad") || lower_input.contains("sorrow") {
            Ok(EmotionalOutput {
                primary_emotion: "sorrow".to_string(),
                intensity: 0.7, // Predefined intensity for "sorrow"
            })
        } else if lower_input.contains("angry") || lower_input.contains("anger") {
            Ok(EmotionalOutput {
                primary_emotion: "anger".to_string(),
                intensity: 0.9, // Predefined intensity for "anger"
            })
        } else if lower_input.contains("fear") {
            Ok(EmotionalOutput {
                primary_emotion: "fear".to_string(),
                intensity: 0.6, // Predefined intensity for "fear"
            })
        }
        else {
            // Default to "neutral" if no specific keywords are found.
            Ok(EmotionalOutput {
                primary_emotion: "neutral".to_string(),
                intensity: 0.5, // Predefined intensity for "neutral"
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_happy() {
        let engine = MyEmotionalReasoningEngine::new();
        let output = engine.analyze_emotional_context("What a happy day!").unwrap();
        assert_eq!(output.primary_emotion, "joy");
        assert_eq!(output.intensity, 0.8);
    }

    #[test]
    fn test_analyze_sad() {
        let engine = MyEmotionalReasoningEngine::new();
        let output = engine.analyze_emotional_context("This is a sad song.").unwrap();
        assert_eq!(output.primary_emotion, "sorrow");
        assert_eq!(output.intensity, 0.7);
    }

    #[test]
    fn test_analyze_angry() {
        let engine = MyEmotionalReasoningEngine::new();
        let output = engine.analyze_emotional_context("I am so angry right now!").unwrap();
        assert_eq!(output.primary_emotion, "anger");
        assert_eq!(output.intensity, 0.9);
    }
    
    #[test]
    fn test_analyze_fear() {
        let engine = MyEmotionalReasoningEngine::new();
        let output = engine.analyze_emotional_context("This is causing a lot of fear.").unwrap();
        assert_eq!(output.primary_emotion, "fear");
        assert_eq!(output.intensity, 0.6);
    }

    #[test]
    fn test_analyze_neutral() {
        let engine = MyEmotionalReasoningEngine::new();
        let output = engine.analyze_emotional_context("This is a neutral statement.").unwrap();
        assert_eq!(output.primary_emotion, "neutral");
        assert_eq!(output.intensity, 0.5);
    }

    #[test]
    fn test_analyze_mixed_joy_dominant() {
        // Example where "happy" might appear with other weaker emotional words
        let engine = MyEmotionalReasoningEngine::new();
        let output = engine.analyze_emotional_context("I felt a bit sad, but overall it was a happy occasion.").unwrap();
        // Current simple logic will pick the first match based on order.
        // "sad" appears first in the text and in the if/else chain.
        // This test highlights the current engine's limitation if we expected "happy" to override.
        // For the current implementation, "sorrow" will be picked.
        // If the requirement was for "happy" to dominate, the engine logic would need to be more complex.
        assert_eq!(output.primary_emotion, "sorrow"); // This will be joy if "happy" is checked before "sad"
                                                   // or if "happy" is later in the string but given priority.
                                                   // Current logic: "sad" is before "happy" in the if/else if chain.
                                                   // Let's check the actual implementation.
                                                   // happy/joy is checked before sad/sorrow. So joy should be output.
        //assert_eq!(output.primary_emotion, "joy");
        //assert_eq!(output.intensity, 0.8);
        // After reviewing MyEmotionalReasoningEngine: "happy" is checked before "sad".
        // So, "joy" should be the output.
         assert_eq!(output.primary_emotion, "joy");
         assert_eq!(output.intensity, 0.8);

    }

    #[test]
    fn test_analyze_empty_string() {
        let engine = MyEmotionalReasoningEngine::new();
        let output = engine.analyze_emotional_context("").unwrap();
        // Empty string should likely result in "neutral"
        assert_eq!(output.primary_emotion, "neutral");
        assert_eq!(output.intensity, 0.5);
    }
}
