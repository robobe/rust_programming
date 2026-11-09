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
    fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            status: TaskStatus::Pending,
        }
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn rename(&mut self, title: &str) {
        self.title = String::from(title);
    }

    fn into_title(self) -> String {
        self.title
    }

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

fn find_task(tasks: &[Task], id: u32) -> Option<&Task> {
    for task in tasks {
        if task.id == id {
            return Some(task);
        }
    }
    None
}

fn main() {
    let mut task = Task::new(42, String::from("Read Rust"));
    task.rename("Practice task modeling");
    assert_eq!(task.complete(), Ok(()));
    println!("task {} completed: {}", task.id, task.title());
    let repeated = task.complete();
    assert_eq!(repeated, Err(TaskError::AlreadyCompleted));
    println!("repeat: {repeated:?}");
    let tasks = vec![
        task,
        Task {
            id: 9,
            title: String::from("Old task"),
            status: TaskStatus::Cancelled {
                reason: String::from("Duplicate"),
            },
        },
    ];
    match find_task(&tasks, 42) {
        Some(task) => println!("found id {}: {}", task.id, task.title()),
        None => println!("task not found"),
    }
    assert!(find_task(&tasks, 999).is_none());
    println!("missing id 999: {}", find_task(&tasks, 999).is_none());
    assert_eq!(
        Task::new(1, String::from("Owned title")).into_title(),
        "Owned title"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_renames_and_consumes_title() {
        let mut task = Task::new(42, String::from("Read"));
        assert_eq!(task.id, 42);
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.title(), "Read");
        task.rename("Practice");
        assert_eq!(task.into_title(), "Practice");
    }

    #[test]
    fn completes_once_and_preserves_fields_on_rejection() {
        let mut task = Task::new(7, String::from("Read"));
        assert_eq!(task.complete(), Ok(()));
        assert_eq!(task.status, TaskStatus::Completed);
        assert_eq!(task.complete(), Err(TaskError::AlreadyCompleted));
        assert_eq!(task.status, TaskStatus::Completed);
        assert_eq!(task.id, 7);
        assert_eq!(task.title(), "Read");
    }

    #[test]
    fn rejects_cancelled_completion_without_losing_reason() {
        let mut task = Task {
            id: 8,
            title: String::from("Old"),
            status: TaskStatus::Cancelled {
                reason: String::from("Duplicate"),
            },
        };
        assert_eq!(task.complete(), Err(TaskError::Cancelled));
        assert_eq!(
            task.status,
            TaskStatus::Cancelled {
                reason: String::from("Duplicate")
            }
        );
        assert_eq!(task.title(), "Old");
    }

    #[test]
    fn finds_identifier_rather_than_position() {
        let tasks = vec![
            Task::new(42, String::from("First")),
            Task::new(9, String::from("Second")),
        ];
        match find_task(&tasks, 9) {
            Some(task) => assert_eq!(task.title(), "Second"),
            None => panic!("expected task ID 9"),
        }
        assert!(find_task(&tasks, 0).is_none());
        assert!(find_task(&tasks, 999).is_none());
        assert!(find_task(&[], 42).is_none());
    }
}
