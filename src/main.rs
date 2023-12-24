use crate::program::Program;

mod program;
mod user;
mod user_service;

fn main() {
    Program::new().create_user("Taro").unwrap();
}
