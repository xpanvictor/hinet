#![cfg(test)]

use prost::Message;

use crate::pb::message::{Msg, MsgStatus};

#[test]
fn test_encoding_decoding() {
    let msg = Msg {
        content: "Hello, world!".to_string(),
        sender_id: "user123".to_string(),
        created_at: 1625077765,
        status: MsgStatus::Sent as i32,
    };
    let encoded_msg = msg.encode_to_vec();
    let decoded_msg = Msg::decode(&*encoded_msg).unwrap();
    assert_eq!(msg, decoded_msg);
}
