extern crate protocol;
#[macro_use]
extern crate protocol_derive;

#[derive(Protocol, Debug, PartialEq)]
pub struct Message {
    pub message_string: String,
    pub timestamp: String,
    pub is_test: bool,
}

use protocol::Parcel;

// Sample Message
// Message {
//     message_string: "Bob".to_owned(),
//     timestamp: "2014-07-08T09:10:11Z".to_owned(),
//     is_test: true,
//  }.raw_bytes(&protocol::Settings::default()).unwrap()

// write some methods to create / handle / send / receive new Messages
