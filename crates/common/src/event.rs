use bytes::Bytes;

trait ProtoElemType {
    fn encode_to_vec(&self) -> Vec<u8>;
}

pub fn bytes_from_proto(elem: impl ProtoElemType) -> Bytes {
    Bytes::from(elem.encode_to_vec())
}
