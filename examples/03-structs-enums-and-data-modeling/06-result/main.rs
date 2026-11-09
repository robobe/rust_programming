#[derive(Debug, PartialEq)]
enum TaskStatus {
    Pending,
    Completed,
    Cancelled { reason: String },
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

#[derive(Debug, PartialEq)]
enum TaskError {
    AlreadyCompleted,
    Cancelled,
}

impl Task {
    fn complete(&mut self) -> Result<(), TaskError> {
        match &self.status {
            TaskStatus::Pending => {
                self.status = TaskStatus::Completed;
                Ok(())
            }
            TaskStatus::Completed => Err(TaskError::AlreadyCompleted),
            TaskStatus::Cancelled { .. } => Err(TaskError::Cancelled),
        }
    }
}

fn main() {
    let mut task = Task {
        id: 7,
        title: String::from("Read Rust"),
        status: TaskStatus::Pending,
    };
    match task.complete() {
        Ok(()) => println!("completed task {}: {}", task.id, task.title),
        Err(error) => println!("completion failed: {error:?}"),
    }
    assert_eq!(task.status, TaskStatus::Completed);
    let repeated = task.complete();
    assert_eq!(repeated, Err(TaskError::AlreadyCompleted));
    match repeated {
        Ok(()) => println!("completed again"),
        Err(error) => println!("second completion: {error:?}"),
    }
    assert_eq!(task.status, TaskStatus::Completed);
    let mut cancelled = Task {
        id: 8,
        title: String::from("Old task"),
        status: TaskStatus::Cancelled {
            reason: String::from("Duplicate"),
        },
    };
    assert_eq!(cancelled.complete(), Err(TaskError::Cancelled));
    assert_eq!(
        cancelled.status,
        TaskStatus::Cancelled {
            reason: String::from("Duplicate")
        }
    );
}
