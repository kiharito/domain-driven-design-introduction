use crate::full_name::FullName;

mod full_name;

fn main() {
    let full_name = FullName::new("Taro", "Suzuki");
    println!("Hello, {} {}!", full_name.first_name(), full_name.last_name());
}
