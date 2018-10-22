use std::io;
//use std::io::prelude::*;

fn main() {
  let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");
    let mut vec = a_str.split_whitespace()
    .map(|x| x.parse::<i32>().expect("parse error"))
    .collect::<Vec<i32>>();
}
