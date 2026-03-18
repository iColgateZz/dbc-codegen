use can_dbc::Dbc as ParsedDbc;
use can_dbc::Message as ParsedMessage;
use can_dbc::SignalExtendedValueTypeList as ParsedExtendedValueType;
use crate::ir::{map_into, SignalValueEnum, ExtendedValueType, Signal, Message, SignalIdx, SignalLayout, SignalLayoutIdx};
use can_dbc::ValueDescription as ParsedValueDescription;

use std::collections::HashMap;

use crate::DbcFile;

type SignalKey = (can_dbc::MessageId, String);

pub struct IRBuilder;

impl IRBuilder {

    pub fn to_ir(value: ParsedDbc) -> DbcFile {
        let mut file = DbcFile::default();

        file.nodes = map_into(value.nodes);
        let mut value_enum_map = Self::value_enum_map(value.value_descriptions);
        let mut extended_type_map = Self::extended_type_map(value.signal_extended_value_type_list);
        let mut signal_layout_map: HashMap<SignalLayout, SignalLayoutIdx> = HashMap::new();

        for msg in value.messages {
            let ParsedMessage { id, name, size, transmitter, signals, .. } = msg;

            if name == "VECTOR__INDEPENDENT_SIG_MSG" {
                continue;
            }

            let mut signal_idxs = vec![];
            for parsed_sig in signals {

                let key = (id, parsed_sig.name.clone());

                let layout = SignalLayout::from(&parsed_sig);
                let layout_idx = if let Some(idx) = signal_layout_map.get(&layout) {
                    *idx
                } else {
                    let idx = SignalLayoutIdx(file.signal_layouts.len());
                    file.signal_layouts.push(layout);
                    signal_layout_map.insert(layout, idx);
                    idx
                };

                let mut signal = Signal::from(parsed_sig);
                signal.layout = layout_idx;

                if let Some(enum_val) = value_enum_map.remove(&key) {
                    signal.signal_value_enum = Some(enum_val);
                }

                if let Some(ext) = extended_type_map.remove(&key) {
                    signal.extended_type = ext;
                }

                let sig_idx = file.signals.len();
                file.signals.push(signal);
                signal_idxs.push(SignalIdx(sig_idx));
            }

            file.messages.push(
                Message::from_parsed(id, name, size, transmitter, signal_idxs)
            );
        }

        file
    }

    fn value_enum_map(value_descriptions: Vec<ParsedValueDescription>) -> HashMap<SignalKey, SignalValueEnum> {
        let mut value_enum_map: HashMap<SignalKey, SignalValueEnum> = HashMap::new();

        for value_enum in value_descriptions {
            if let ParsedValueDescription::Signal {
                message_id,
                name,
                value_descriptions,
            } = value_enum
            {
                let sve = SignalValueEnum::from_parsed( value_descriptions);
                value_enum_map.insert((message_id, name), sve);
            }
        }

        value_enum_map
    }

    fn extended_type_map(extended_types: Vec<ParsedExtendedValueType>) -> HashMap<SignalKey, ExtendedValueType> {
        let mut extended_type_map: HashMap<SignalKey, ExtendedValueType> = HashMap::new();

        for ext in extended_types {
            let ir_ext_type = ExtendedValueType::from(ext.signal_extended_value_type);

            let ParsedExtendedValueType { message_id, signal_name, ..} = ext;
            extended_type_map.insert((message_id, signal_name),ir_ext_type);
        }

        extended_type_map
    }
}