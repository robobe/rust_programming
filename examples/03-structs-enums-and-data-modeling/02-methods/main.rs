#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
}

impl Task {
    /// Creates a task by taking ownership of a new title string.
    fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: String::from(title),
        }
    }

    /// Borrows the title so the task remains available to the caller.
    fn title(&self) -> &str {
        &self.title
    }

    /// Replaces the title through a mutable borrow of the task.
    fn rename(&mut self, title: &str) {
        self.title = String::from(title);
    }

    /// Consumes the task and returns its owned title.
    fn into_title(self) -> String {
        self.title
    }
}

fn main() {
    let mut task = Task::new(1, "Read the Rust book");
    println!("created: {task:?} (id: {})", task.id);
    println!("title: {}", task.title());

    task.rename("Practice methods");
    println!("renamed: {task:?}");

    let title = task.into_title();
    println!("owned title: {title}");
}

#[cfg(test)]
mod tests {
    use super::Task;

    #[test]
    fn methods_read_rename_and_consume_a_task() {
        let mut task = Task::new(7, "First title");
        assert_eq!(task.id, 7);
        assert_eq!(task.title(), "First title");

        task.rename("Second title");
        assert_eq!(task.title(), "Second title");
        assert_eq!(task.into_title(), "Second title");
    }
}
