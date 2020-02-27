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

pub fn parse_message_string(msg: Message) -> MessageString {
    let mut msg_split: Vec<&str> = msg.message_string.split("-").collect();
    let msgstr = MessageString {
        client_code: msg_split[0].to_string(),
        alarm_event_code: msg_split[1].to_string(),
        handshake_code: msg_split[2].to_string()
    };
    
    return msgstr;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    let test_data = b"{
            \"message_string\": \"00000000000001773-00100-22\",
            \"timestamp\": \"2014-07-08T09:10:11Z\",
            \"is_test\": true
    }";
    
    let valid_msg = Message {
        message_string: "00000000000001773-00100-22",
        timestamp: "2014-07-08T09:10:11Z",
        is_test: true,
    };
    
    #[test]
    fn test_message_from_data() {
        asset_eq!(message_from_data(test_data), valid_msg);
    }
}
