extern crate redis;
use redis::Commands;

pub fn create_client(redis_url: &str) -> redis::Client {
  let client = redis::Client::open(redis_url).unwrap();
  return client;
}

fn fetch(client: redis::Client, key: &str) -> redis::RedisResult<&str> {
    let mut con = client.get_connection()?;
    return con.get(key);
}
