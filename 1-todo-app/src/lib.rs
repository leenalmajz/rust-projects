use chrono::{NaiveDate, Utc}; // crate for a date object

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
    pub fn add_new_task(&mut self, description: &str, due_date: Option<NaiveDate>) {
        let task = Task::new(description, due_date);
        self.tasks.push(task);
    }

    // Marks tasks as done
    pub fn mark_unmark_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            if task.done {task.done = false} else {task.done = true}
        }
    }

    // Displaying the current list of tasks
    pub fn show_tasks(&self) {

        println!("\n");
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[ ]" };
            println!("{} | {}. {} | {:?}", status, index + 1, task.description, task.due_date);
        }
    }

}

// Represent individual tasks
pub struct Task {
    description: String,
    due_date: Option<NaiveDate>,
    done: bool,
}

impl Task {
    // Task Constructor
    pub fn new(description: &str, due_date: Option<NaiveDate>) -> Task {
        Task {
            description: description.to_string(),
            due_date: due_date,
            done: false,
        }
    }
}
