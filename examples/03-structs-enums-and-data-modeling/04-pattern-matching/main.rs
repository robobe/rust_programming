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

fn status_label(status: &TaskStatus) -> &str {
    match status {
        TaskStatus::Pending => "pending",
        TaskStatus::Completed => "completed",
        TaskStatus::Cancelled { reason } if reason.is_empty() => "cancelled without a reason",
        TaskStatus::Cancelled { .. } => "cancelled with a reason",
    }
}

fn main() {
    let task = Task {
        id: 7,
        title: String::from("Read Rust"),
        status: TaskStatus::Cancelled {
            reason: String::from("Duplicate"),
        },
    };
    let Task { id, title, .. } = &task;
    assert_eq!(*id, 7);
    assert_eq!(title, "Read Rust");
    println!("task {id}: {title}");
    assert_eq!(status_label(&task.status), "cancelled with a reason");
    if let TaskStatus::Cancelled { reason } = &task.status {
        assert_eq!(reason, "Duplicate");
        println!("borrowed reason: {reason}");
    }
    assert_eq!(status_label(&TaskStatus::Pending), "pending");
    assert_eq!(status_label(&TaskStatus::Completed), "completed");
    assert_eq!(
        status_label(&TaskStatus::Cancelled {
            reason: String::new()
        }),
        "cancelled without a reason"
    );
    let status = task.status;
    match status {
        TaskStatus::Cancelled { reason } => println!("owned reason: {reason}"),
        TaskStatus::Pending | TaskStatus::Completed => println!("no reason"),
    }
    // task.status moved out; the independent title field remains usable.
    assert_eq!(task.title, "Read Rust");
}
