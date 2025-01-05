use chrono::NaiveDate; // crate for a date object

// Manage collection of tasks
pub struct TodoApp {
    pub tasks: Vec<Task>,
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
    pub fn show_tasks_default(&self) {

        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[ ]" };
            println!("{} | {}. {} | {:?}", status, index + 1, task.description, task.due_date);
        }
    }

    pub fn show_tasks_done(&self) {

        for (index, task) in self.tasks.iter().enumerate() {
            if task.done {
                let status = "[X]";
                println!("{} | {}. {} | {:?}", status, index + 1, task.description, task.due_date);
            }  
        }
    }

    pub fn show_tasks_undone(&self) {

        for (index, task) in self.tasks.iter().enumerate() {
            if !task.done {
                let status = "[ ]";
                println!("{} | {}. {} | {:?}", status, index + 1, task.description, task.due_date);
            }  
        }
    }

    pub fn show_tasks_due_date(&self) {

        let mut tasks_with_due_date: Vec<&Task> = self.tasks.iter().filter(|task| task.due_date.is_some()).collect();
        tasks_with_due_date.sort_by_key(|task| task.due_date);

        for (index, task) in tasks_with_due_date.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[ ]" };
            println!("{} | {}. {} | {:?}", status, index + 1, task.description, task.due_date);
        }
        
        let tasks_without_due_date: Vec<&Task> = self.tasks.iter().filter(|task| task.due_date.is_none()).collect();
        let len = tasks_with_due_date.len();
        for (index, task) in tasks_without_due_date.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[ ]" };
            println!("{} | {}. {} | None", status, index + 1 + len, task.description);
        }
    }

    pub fn show_tasks_by_alphabet(&self) {

        let mut tasks_by_alphabet: Vec<&Task> = self.tasks.iter().collect();
        tasks_by_alphabet.sort_by_key(|task| &task.description);

        for (index, task) in tasks_by_alphabet.iter().enumerate() {
            let status = if task.done { "[X]" } else { "[ ]" };
            println!("{} | {}. {} | {:?}", status, index + 1, task.description, task.due_date);
        }
    }

    pub fn show_task(&self, index: usize) {

        match self.tasks.get(index) {
            Some(task) => {
                let status = if task.done { "[X]" } else { "[ ]" };
                println!("{} | {}. {} | {:?}", status, index + 1, task.description, task.due_date);
            }
            None => println!("Task is not found"),
        };
    }

    pub fn get_task(&mut self, index:usize) -> Option<&mut Task> {
        self.tasks.get_mut(index)
    }

    pub fn delete_task(&mut self, index:usize) {

        if index < self.tasks.len() {
            self.tasks.remove(index);
            println!("Task was deleted successfully.")
        } else {
            println!("Task index out of bounds");
        }

    }
}

// Represent individual tasks
pub struct Task {
    pub description: String,
    pub due_date: Option<NaiveDate>,
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
