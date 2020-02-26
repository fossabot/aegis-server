use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message_string: String,
    timestamp: String,
    is_test: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageString {
    client_code: &str,
    alarm_event_code: &str,
    handshake_code: &str,
}

pub fn message_from_data(data: &[u8]) -> Message {
  let v: Message = serde_json::from_slice(data).unwrap();
  return v;
}

pub fn parse_message_string(msg: Message) -> MessageString {
    let mut msg_split: Vec<&str> = msg.message_string.split("-").collect();
    let msgstr = MessageString {
        client_code: msg_split[0],
        alarm_event_code: msg_split[1],
        handshake_code: msg_split[2]
    };
    
    return msgstr;
}
