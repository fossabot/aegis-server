extern crate chrono;
use chrono::prelude::*;

pub fn generate_timestamp() -> DateTime<Local> {
  let local: DateTime<Local> = Local::now();
  return local;
}

pub fn pretty_print(input: &str) {
  println!("{:#?}", input);
}

pub fn push_key_to_redis(key: &str, val: &str) {
  // Noting here yet....
}

pub fn pull_key_from_redis(key: &str) -> &str {
  // Noting here yet....
}
