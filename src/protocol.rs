use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message_string: String,
    timestamp: String,
    is_test: bool,
}

pub fn message_from_data(data: &[u8]) -> Message {
  let v: Message = serde_json::from_slice(data).unwrap();
  return v;
}
