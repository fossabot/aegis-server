//! take redis client connection, subscribe to the `aegis_alarm` channel.

extern crate redis;
use redis::{Client, Commands, ControlFlow, PubSubCommands};

//! build a connection URL
//! > NOTE: Aegis requires an authenticated redis connection.
pub fn build_redis_url(user: &str, passw: &str, host: &str) -> &str {
  let rurl = format!("redis://{}:{}@{}", user, passw, host);
  return rurl;
}

//! Creates a redis client for use in later pub/sub operations
pub fn create_redis_client(redis_urL: &str) -> () {
  let client = redis::Client::open(redis_url).unwrap();
  return &client;
}

//! Publish a message to the redis channel
pub fn publish(client: (), message_string: &str, channel: &str) {
  let conn = client.get_connection().unwrap();
  let _: ()  = conn.publish(channel, message_string).unwrap();
}
