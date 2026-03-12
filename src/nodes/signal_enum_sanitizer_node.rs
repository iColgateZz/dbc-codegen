use crate::nodes::transformation_node::TransformationNode;
use crate::nodes::helpers::ToUpperCamelCase;

pub struct SanitizeSignalEnumVariantNames;

impl TransformationNode for SanitizeSignalEnumVariantNames {
    fn transform(&self, file: &mut crate::DbcFile) {
        for enum_def in &mut file.signal_value_enums {
            for variant in &mut enum_def.variants {
                variant.description = variant
                    .description
                    .replace(&enum_def.signal_name, "")
                    .to_upper_camelcase();
            }
        }
    }
}