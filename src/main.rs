use crate::program::Program;
use crate::repository::UserRepository;

mod program;
mod repository;
mod user;
mod user_service;

fn main() {
    Program::new(UserRepository::new())
        .create_user("Taro")
        .unwrap();
}
