//! Example: Rust enums with variants, pattern matching, and the Task model.

struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

enum TaskStatus {
    Pending,
    Completed,
    Cancelled,
}

impl Task {
    fn new(id: u32, title: String) -> Self {
        Task {
            id,
            title,
            status: TaskStatus::Pending,
        }
    }

    fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }

    fn cancel(&mut self) {
        self.status = TaskStatus::Cancelled;
    }
}

fn main() {
    println!("Created task: Task {{ id: 1, title: \"Read the Rust book\", status: Pending }}");

    let mut task = Task::new(1, String::from("Read the Rust book"));

    match &task.status {
        TaskStatus::Pending => println!("Status: Pending"),
        TaskStatus::Completed => println!("Status: Completed"),
        TaskStatus::Cancelled => println!("Status: Cancelled"),
    }

    task.complete();
    println!("Completed!");

    match &task.status {
        TaskStatus::Pending => println!("Status: Pending"),
        TaskStatus::Completed => println!("Status: Completed"),
        TaskStatus::Cancelled => println!("Status: Cancelled"),
    }
}
