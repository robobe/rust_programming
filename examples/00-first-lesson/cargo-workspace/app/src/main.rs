use toolkit::{greetings, numbers};

fn main() {
    println!("{}", greetings::hello("workspace"));
    println!("2 + 3 = {}", numbers::add(2, 3));
}
