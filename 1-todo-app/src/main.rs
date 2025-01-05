use std::io::{self, Write};
use todo_app::TodoApp;
use chrono::{NaiveDate, Utc};

fn get_string_input(prompt: &str) -> Option<String> {
    // Prints the prompt first to user
    print!("{}", prompt);

    // Ensures the prompt is displayed immediately to user
    std::io::stdout().flush().ok();

    // Make a new input variable to store the response
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the line");

    match input.trim() {
        "q" =>  None,
        _ => Some(input.trim().to_string()),
    }
}

fn get_numeric_input(prompt: &str) -> Option<u8> {
    print!("{}", prompt);
    std::io::stdout().flush().ok();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read the line");
    if input.trim() == "q".to_string() {input = "0".to_string()};
    match input.trim().parse() {
        Ok(value) => Some(value),
        Err(_) => {
            println!("Invalid input, enter proper number");
            None
        }
    }
}

fn string_to_date(date_str: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
}

fn main() {

    let mut todoListApp = TodoApp::new();

    'app: loop {
        // Display menu options
        println!("\n\n_______________________________________\n\n");
        println!("1. Add new Tasks");
        println!("2. Mark/unmark Tasks");
        println!("3. Show Tasks");   // TODO: list them by uncomplete first, alphabetical order, complete first, due date
        println!("4. Edit Tasks");    // TODO: edit the task description or due date
        println!("5. Delete a Task");  // TODO: delete one of all tasks
        println!("6: Exit");   // TODO: are you sure? (Y/n)

        // Get user input for menu choice
        let choice = match get_numeric_input("\nEnter you choice: ") {
            Some(value) => value,
            None => {
                println!("Invalid input, enter valid number between 1-9");
                continue;
            } 
        };

        match choice {
            1 => {

                'new_task: loop {

                    // Get task description from user
                    let description = match get_string_input("\nEnter task description. \nor press q to return back to menu \n\n") {
                        Some(input) => input,
                        None => break,
                    };

                    // Add a due date OPTIONAL:
                    let due_date;
                    'date: loop {
                        match get_string_input("\nEnter due date (YYYY-MM-DD) \nenter to skip \nor press q to return back to menu \n\n") {
                            Some(input) => match input.as_str() {
                                "" => {
                                    due_date = None;
                                    break 'date;
                                }
                                _ => {
                                    match string_to_date(&input) {
                                        Some(date) => {
                                            due_date = Some(date);
                                            break 'date;
                                        },
                                        None => {
                                            println!("Invalid date format");
                                            continue 'date;
                                        },
                                    }
                                }
                            },
                            None => continue 'app,
                        }
                    }
                                    
                    todoListApp.add_new_task(&description, due_date); 
                } 
            },
            2 => {
                'mark_done: loop {
                    todoListApp.show_tasks();
                    println!("\n");
                    let index = match get_numeric_input("Enter the task index to mark/unmark: ") {
                        Some(value) => match value {
                            0 => break,
                            _ => value-1,
                        }
                        None => {
                            println!("Invalid input, enter proper number");
                            continue;
                        },
                    };
                    todoListApp.mark_unmark_task(index as usize);
                }
            }
            3 => todoListApp.show_tasks(),  // TODO: How do you want to show the tasks
            4 => println!("Edit Tasks"), // TODO: Enter a task index to edit it
            5 => println!("Delete Tasks"), // TODO: Enter a task's index to delete it
            6 => break,
            _ => println!("Invalid option, enter number between 1-4"),
        };

        // let args: Vec<String> = env::args().collect();
    }
}