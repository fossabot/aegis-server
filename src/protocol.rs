use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Message {
    message_string: String,
    timestamp: String,
    is_test: bool,
}

pub fn message_from_data(data: &str) -> Message {
  let v: Message = serde_json::from_str(data);
  return v;
}
