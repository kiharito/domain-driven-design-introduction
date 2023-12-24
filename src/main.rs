use crate::user::{User, UserName};

mod user;

fn main() {
    println!("Hello, {:?}!", User::new(UserName::new("taro").unwrap()));
}
