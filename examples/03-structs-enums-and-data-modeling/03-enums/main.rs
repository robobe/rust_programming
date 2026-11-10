//! Example: Rust enums with variants, pattern matching, and the Task model.

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

#[derive(Debug, PartialEq)]
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
    let mut task = Task::new(1, String::from("Read the Rust book"));
    println!("Created task: {task:?}");
    assert_eq!(task.id, 1);
    assert_eq!(task.title, "Read the Rust book");

    match &task.status {
        TaskStatus::Pending => println!("Status: Pending"),
        TaskStatus::Completed => println!("Status: Completed"),
        TaskStatus::Cancelled => println!("Status: Cancelled"),
    }

    task.complete();
    assert_eq!(task.status, TaskStatus::Completed);
    println!("Completed!");

    match &task.status {
        TaskStatus::Pending => println!("Status: Pending"),
        TaskStatus::Completed => println!("Status: Completed"),
        TaskStatus::Cancelled => println!("Status: Cancelled"),
    }
    task.cancel();
    assert_eq!(task.status, TaskStatus::Cancelled);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_and_sets_each_status() {
        let mut task = Task::new(1, String::from("Read"));
        assert_eq!(task.status, TaskStatus::Pending);
        task.complete();
        assert_eq!(task.status, TaskStatus::Completed);
        task.cancel();
        assert_eq!(task.status, TaskStatus::Cancelled);
        assert_eq!(task.title, "Read");
    }
}
