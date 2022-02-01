use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel: String,
    pub payload: String
}

impl Message {
    pub fn new(payload: String) -> Message {
        Message {
            id: Message::generate_id(),
            channel: String::from("stats"),
            payload
        }
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}