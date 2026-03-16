use super::transformation::TransformationNode;

/// Attach a SignalExtendedValueType to a Signal if there is one
/// matching the message ID and signal name
pub struct AttachSignalExtendedValueTypes;

impl TransformationNode for AttachSignalExtendedValueTypes {
    fn transform(&self, file: &mut crate::DbcFile) {
        //TODO: use hashmap for better performance
        for msg in &mut file.messages {
            for signal_idx in &msg.signal_idxs {
                let signal = &mut file.signals[signal_idx.0];

                for extended_type in file.signal_extended_value_types.iter() {
                    if extended_type.message_id == msg.id && extended_type.signal_name.raw() == signal.name.raw() {
                        signal.extended_type = extended_type.extended_value_type.clone();
                    }
                }
            }
        }
    }
}
