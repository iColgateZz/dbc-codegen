use crate::ir::signal_extended_value_type::ExtendedValueType;
use crate::ir::{Message, Node, Signal, SignalIdx, SignalValueEnum, helpers::map_into};
use can_dbc::{Dbc as ParsedDbc, SignalExtendedValueTypeList};
use can_dbc::Message as ParsedMessage;
use can_dbc::ValueDescription as ParsedValueDescription;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct DbcFile {
    pub nodes: Vec<Node>,
    pub messages: Vec<Message>,
    pub signals: Vec<Signal>,

    //TODO: consider how to use can_dbc::value_tables. Basically,
    //      these are global enums for signal values

    //TODO: can_dbc::comments and attribute_* stuff may be
    //      used as metadata in generated code

    //TODO: consider how to use can_dbc::signal_types and 
    //      signal_type_refs. original dbc-codegen does not
    //      support them. They allow to define a signal once
    //      and then reuse them later.

    //TODO: can_dbc::extended_multiplex is probably also needed
}

impl From<ParsedDbc> for DbcFile {
    fn from(value: ParsedDbc) -> Self {
        //TODO: Add IRBuilder class for this logic
        let mut file = DbcFile::default();

        file.nodes = map_into(value.nodes);

        type SignalKey = (can_dbc::MessageId, String);
        let mut value_enum_map: HashMap<SignalKey, SignalValueEnum> = HashMap::new();

        for value_enum in value.value_descriptions {
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

        let mut extended_type_map: HashMap<SignalKey, ExtendedValueType> = HashMap::new();

        for ext in value.signal_extended_value_type_list {
            let ir_ext_type = ExtendedValueType::from(ext.signal_extended_value_type);

            let SignalExtendedValueTypeList { message_id, signal_name, ..} = ext;
            extended_type_map.insert((message_id, signal_name),ir_ext_type);
        }


        for msg in value.messages {
            //TODO: filter non-relevant messages here
            let mut signal_ids = vec![];
            let ParsedMessage { id, name, size, transmitter, signals, .. } = msg;

            for sig in signals {
                let key = (id, sig.name.clone());
                let mut signal = Signal::from(sig);

                if let Some(enum_val) = value_enum_map.remove(&key) {
                    signal.signal_value_enum = Some(enum_val);
                }

                if let Some(ext) = extended_type_map.remove(&key) {
                    signal.extended_type = ext;
                }

                let id = file.signals.len();
                file.signals.push(signal);
                signal_ids.push(SignalIdx(id));
            }

            file.messages.push(Message::from_parsed(id, name, size, transmitter, signal_ids));
        }

        file
    }
}
