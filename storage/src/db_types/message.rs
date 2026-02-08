use std::time;
use message

pub struct Message {
    id: String,
    is_group: bool,
    sender_id: String,
    content: String,
    timestamp: time,
    group_id: String,
}

// Converters from DmMessage and GroupMessage
impl From<DmMess
