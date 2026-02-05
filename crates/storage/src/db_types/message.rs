use std::time;

use anyhow::anyhow;
use message::pb::{
    chat_dm::{DirectMessage, direct_message},
    chat_group::GroupMessage,
};
use uuid::Uuid;

use crate::error::DbError;

#[derive(Debug, Clone)]
pub struct DbMessage {
    id: Uuid,
    is_group: bool,
    sender_id: String,
    content: String,
    timestamp: u64,
    group_id: Option<String>, // todo: attachment
}

impl TryFrom<DirectMessage> for DbMessage {
    type Error = DbError;
    fn try_from(value: DirectMessage) -> Result<Self, Self::Error> {
        // question: why d heck am I trying to store any other form aside Text.
        Ok(DbMessage {
            id: Uuid::parse_str(&value.id).map_err(DbError::InvalidDataType(anyhow!(
                "Can't format str to uuid"
            )))?,
            is_group: false, // direct message
            sender_id: todo!(),
            content: match value.payload.unwrap() {
                Text(content) => content,
                _ => anyhow!("Don't store others")?,
            },
            timestamp: value.timestamp,
            group_id: None,
        })
    }
}

impl From<GroupMessage> for DbMessage {
    fn from(value: GroupMessage) -> Self {
        todo!()
    }
}
