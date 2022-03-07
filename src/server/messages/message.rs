use serde::{Deserialize, Serialize};
use uuid::Uuid;
mod parser;
use parser::parse_message;

#[derive{Debug, Serialize, Deserialize}]
pub struct Message {
    pub id: String,
    pub channel: String,
    pub topic: String,
    pub payload: String,
}

impl Message {
    pub fn new() -> Message {
        parse_message()
    }
}
