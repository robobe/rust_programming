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

fn task_at(tasks: &[Task], position: usize) -> Option<&Task> {
    tasks.get(position)
}

fn main() {
    let tasks = vec![
        Task {
            id: 42,
            title: String::from("Read Rust"),
            status: TaskStatus::Pending,
        },
        Task {
            id: 9,
            title: String::from("Practice"),
            status: TaskStatus::Completed,
        },
        Task {
            id: 15,
            title: String::from("Old task"),
            status: TaskStatus::Cancelled {
                reason: String::from("Duplicate"),
            },
        },
    ];
    match task_at(&tasks, 0) {
        Some(task) => {
            assert_eq!(task.id, 42);
            assert_eq!(task.status, TaskStatus::Pending);
            println!("position 0: task {} - {}", task.id, task.title);
        }
        None => panic!("the fixture must contain position 0"),
    }
    match task_at(&tasks, 99) {
        Some(task) => println!("unexpected task: {}", task.id),
        None => println!("position 99: no task"),
    }
    if let Some(task) = task_at(&tasks, 1) {
        assert_eq!(task.id, 9);
        println!("optional title: {}", task.title);
    }
    assert!(task_at(&tasks, 99).is_none());
    assert!(task_at(&[], 0).is_none());
    assert_eq!(tasks.len(), 3);
}
