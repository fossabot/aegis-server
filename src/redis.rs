extern crate redis;
use redis::Commands;

pub fn create_client(redis_url: &str) -> redis::Client {
  let client = redis::Client::open(redis_url).unwrap();
  return client;
}

pub fn push_key_to_redis(client: redis::Client, key: &str, val: &str) {
  let mut con = client.get_connection();
  let _ : () = con.set(key, val);
}

pub fn pull_key_from_redis(client: redis::Client, key: &str) -> &redis::RedisResult<isize> {
  let mut con = client.get_connection();
  return con.get(key);
}
