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
    client_code: String,
    alarm_event_code: String,
    handshake_code: String,
}

pub fn message_from_data(data: &[u8]) -> Message {
  let v: Message = serde_json::from_slice(data).unwrap();
  return v;
}

pub fn parse_message_string(msg: &Message) -> MessageString {
    let mut msg_split: Vec<&str> = msg.message_string.split("-").collect();
    let msgstr = MessageString {
        client_code: msg_split[0].to_string(),
        alarm_event_code: msg_split[1].to_string(),
        handshake_code: msg_split[2].to_string()
    };
    
    return msgstr;
}

pub fn validate_handshake(msgstr: &MessageString, handshake: &str) -> bool {
    if msgstr.handshake_code != handshake {
        return false;
    } else {
        return true;
    }
}

pub fn validate_alarm_event_code(msgstr: &MessageString) -> bool {
    if msgtsr.alarm_event_code.char().count() != 5 {
        return false;
    } else {
        return true;
    }
}

pub fn validate_client_code(msgstr: &MessageString) -> bool {
    if msgtsr.client_code.char().count() != 16 {
        return false;
    } else {
        return true;
    }
}
