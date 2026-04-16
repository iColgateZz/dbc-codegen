use std::collections::HashMap;

use super::transformation::TransformationNode;

/// Append _MSG to message name and ensure uniqueness
pub struct SanitizeMessageNames;

impl TransformationNode for SanitizeMessageNames {
    fn transform(&self, file: &mut crate::DbcFile) {
        let mut counts: HashMap<String, usize> = HashMap::new();

        for msg in &mut file.messages {
            let base = format!("{}_MSG", msg.name.0);

            let count = counts.entry(base.to_lowercase()).or_insert(0);

            let new_name = if *count == 0 {
                base
            } else {
                format!("{}{}", base, count)
            };

            *count += 1;

            msg.name.0 = new_name;
        }

        println!("{counts:?}");
    }
}