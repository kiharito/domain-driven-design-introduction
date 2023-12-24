use crate::program::Program;
use crate::repository::RdsUserRepository;

mod program;
mod repository;
mod user;
mod user_service;

fn main() {
    Program::new(RdsUserRepository::new())
        .create_user("Taro")
        .unwrap();
}
