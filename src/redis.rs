extern crate redis;
use redis::{Client, Commands, Connection, RedisResult};
use std::collections::HashSet;

fn create_client(url: &str) -> Connection {
    let client = Client::open(url).unwrap();
    let conn = client.get_connection().unwrap();
    return conn;
}
