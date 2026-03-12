use super::transformation::TransformationNode;

/// Sanitize the names of Signals.
pub struct SanitizeSignalNames;

impl TransformationNode for SanitizeSignalNames {
    fn transform(&self, file: &mut crate::DbcFile) {
        for msg in &mut file.messages {
            for sig in &mut msg.signals {
                sig.name.0 = sig.original_name.0.to_lowercase();
            }
        }
    }
}
