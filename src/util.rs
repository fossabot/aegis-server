extern crate chrono;
use chrono::prelude::*;

pub fn generate_timestamp() -> DateTime<Local> {
  let local: DateTime<Local> = Local::now();
  return local;
}

pub fn pretty_print(input: &str) {
  println!("{:#?}", input);
}
