#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
}

fn main() {
    let title = String::from("Read the Rust book");
    let mut task = Task { id: 1, title };

    println!("task: {task:?}");
    println!("title: {}", task.title.as_str());

    task.title = String::from("Practice structs");
    println!("updated: {task:?}");

    let title = task.title;
    println!("moved title: {title}");
    println!("remaining id: {}", task.id);
}
