use can_dbc::Dbc as ParsedDbc;
use can_dbc::Message as ParsedMessage;
use can_dbc::Signal as ParsedSignal;
use can_dbc::SignalExtendedValueTypeList as ParsedExtendedValueType;
use crate::ir::{map_into, SignalValueEnum, ExtendedValueType, Signal, Message, SignalIdx, SignalLayout, SignalLayoutIdx};
use can_dbc::ValueDescription as ParsedValueDescription;

use std::collections::HashMap;

use crate::DbcFile;

type SignalKey = (can_dbc::MessageId, String);

pub struct IRBuilder {
    file: DbcFile,
    can_dbc_messages: Vec<ParsedMessage>,

    value_enum_map: HashMap<SignalKey, SignalValueEnum>,
    extended_type_map: HashMap<SignalKey, ExtendedValueType>,
    signal_layout_map: HashMap<SignalLayout, SignalLayoutIdx>,
}

impl IRBuilder {

    pub fn to_ir(value: ParsedDbc) -> DbcFile {
        let mut builder = Self::new(value);
        builder.build();
        builder.file
    }

    fn new(value: ParsedDbc) -> Self {
        let value_enum_map = Self::value_enum_map(value.value_descriptions);
        let extended_type_map = Self::extended_type_map(value.signal_extended_value_type_list);

        let mut file = DbcFile::default();
        file.nodes = map_into(value.nodes);

        Self {
            file,
            can_dbc_messages: value.messages,
            value_enum_map,
            extended_type_map,
            signal_layout_map: HashMap::new(),
        }
    }

    fn build(&mut self) {
        for msg in std::mem::take(&mut self.can_dbc_messages) {
            if msg.name == "VECTOR__INDEPENDENT_SIG_MSG" {
                continue;
            }

            let message = self.build_message(msg);
            self.file.messages.push(message);
        }
    }

    fn build_message(&mut self, msg: ParsedMessage) -> Message {
        let ParsedMessage { id, name, size, transmitter, signals, .. } = msg;

        let signal_idxs = signals
            .into_iter()
            .map(|sig| self.build_signal(id, sig))
            .collect();

        Message::from_parsed(id, name, size, transmitter, signal_idxs)
    }

    fn build_signal(
        &mut self,
        message_id: can_dbc::MessageId,
        parsed_sig: ParsedSignal,
    ) -> SignalIdx {

        let key = (message_id, parsed_sig.name.clone());

        let layout_idx = self.build_signal_layout(&parsed_sig);

        let mut signal = Signal::from(parsed_sig);
        signal.layout = layout_idx;

        if let Some(enum_val) = self.value_enum_map.remove(&key) {
            signal.signal_value_enum = Some(enum_val);
        }

        if let Some(ext) = self.extended_type_map.remove(&key) {
            signal.extended_type = ext;
        }

        let idx = SignalIdx(self.file.signals.len());
        self.file.signals.push(signal);

        idx
    }

    fn build_signal_layout(&mut self, sig: &ParsedSignal) -> SignalLayoutIdx {
        let layout = SignalLayout::from(sig);

        if let Some(idx) = self.signal_layout_map.get(&layout) {
            return *idx;
        }

        let idx = SignalLayoutIdx(self.file.signal_layouts.len());

        self.file.signal_layouts.push(layout);
        self.signal_layout_map.insert(layout, idx);

        idx
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