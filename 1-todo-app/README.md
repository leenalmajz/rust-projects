# Todo CLI

Todo CLI is a simple command-line application to manage a to-do list. You can create new tasks, mark them as done, show tasks, edit tasks, and delete tasks.

## Author

- **Author Name:** Leen Al Majzoub
- **Date:** 05.01.2025

## Features

- Add a new task
- Mark/unmark a task as done
- Show all tasks
- Edit task details
- Delete a task

## Libraries Included

- `chrono` (version 0.4): For handling date and time.

## Functions

### Main Functions

- `get_string_input(prompt: &str) -> Option<String>`: Get string input from the user.
- `get_numeric_input(prompt: &str) -> Option<u8>`: Get numeric input from the user.
- `string_to_date(date_str: &str) -> Option<NaiveDate>`: Convert a string to a `NaiveDate`.

### TodoApp Methods

- `new() -> TodoApp`: Constructor for `TodoApp`.
- `add_new_task(&mut self, description: &str, due_date: Option<NaiveDate>)`: Add a new task.
- `mark_unmark_task(&mut self, index: usize)`: Mark or unmark a task as done.
- `show_tasks_default(&self)`: Show all tasks.
- `show_tasks_done(&self)`: Show all completed tasks.
- `show_tasks_undone(&self)`: Show all incomplete tasks.
- `show_tasks_due_date(&self)`: Show tasks sorted by due date.
- `show_tasks_by_alphabet(&self)`: Show tasks sorted by description.
- `show_task(&self, index: usize)`: Show a specific task.
- `get_task(&mut self, index: usize) -> Option<&mut Task>`: Get a mutable reference to a task.
- `delete_task(&mut self, index: usize)`: Delete a task.

### Task Methods

- `new(description: &str, due_date: Option<NaiveDate>) -> Task`: Constructor for `Task`.

## Installation

1. Clone the repository:
    ```sh
    git clone git@github.com:leenalmajz rust-projects.git
    ```
2. Navigate to the project directory:
    ```sh
    cd 1-todo-app
    ```
3. Build & run the project using Cargo:
    ```sh
    cargo run
    ```
4. Follow instructions shown in the terminal
