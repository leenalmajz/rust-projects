// Manage collection of tasks
pub struct TodoApp {
    tasks: Vec<Task>,
}

impl TodoApp {
    // TodoApp Contructor
    pub fn new () -> TodoApp {
        TodoApp {
            tasks : Vec::new(),
        }
    }

    // Add tasks to TodoApp
    pub fn add_new_task(&mut self, description: &str) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    // Marks tasks as done
    pub fn mark_task_as_done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true
        }
    }

    // Displaying the current list of tasks
    pub fn show_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[]" };
            println!("{} {} : {}", index + 1, task.description, status)
        }
    }

}

// Represent individual tasks
pub struct Task {
    description: String,
    done: bool,
}

impl Task {
    // Task Constructor
    pub fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            done: false,
        }
    }
}
