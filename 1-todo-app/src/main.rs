use std::io::{self, Write};
use todo_cli::TodoApp;

fn get_string_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the line");
    input.trim().to_string()
}

fn get_numeric_input(prompt: &str) -> Option<u8> {
    print!("{}", prompt);
    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the line");
    match input.trim().parse() {
        Ok(value) => Some(value),
        Err(_) => {
            println!("Invalid input, enter proper number");
            None
        }
    }
}

fn main() {

    let mut todoListApp = TodoApp::new();

    loop {
        // Display menu options
        println!("1. Add a new Task");
        println!("2. Mark Task as Done");
        println!("3. Show Tasks");
        println!("4. Exit");

        // Get user input for menu choice
        let choice = match get_numeric_input("Enter you choice: ") {
            Some(value) => value,
            None => {
                println!("Invalid input, enter valid number");
                continue;
            } 
        };

        match choice {
            1 => {
                let description = get_string_input("Enter task description: ");
                todoListApp.add_new_task(&description);  
            }
            2 => {
                todoListApp.show_tasks();
                let index = match get_numeric_input("Enter the task index to mark as done: ") {
                    Some(value) => value-1,
                    None => {
                        println!("Invalid input, enter proper number");
                        continue;
                    },
                };
                todoListApp.mark_task_as_done(index as usize);
            }
            3 => todoListApp.show_tasks(),
            4 => break,
            _ => println!("Invalid option, enter number between 1-4"),
        };

        // let args: Vec<String> = env::args().collect();
    }
}