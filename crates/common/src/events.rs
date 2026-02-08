use bytes::Bytes;
use message::pb::chat_dm::{DirectMessage, Text, direct_message::Payload};

trait ProtoElemType {
    fn encode_to_vec(&self) -> Vec<u8>;
}

pub fn bytes_from_proto(elem: impl ProtoElemType) -> Bytes {
    Bytes::from(elem.encode_to_vec())
}

#[derive(Clone, Debug)]
pub enum NetworkEvents {
    SendMsg,
}

#[derive(Clone, Debug)]
pub enum SystemEvents {
    Network(NetworkEvents),
}
