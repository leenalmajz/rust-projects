use std::io::Write;
use todo_app::{TodoApp, Task};
use chrono::NaiveDate;

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
        println!("3. Show Tasks");
        println!("4. Edit Tasks");
        println!("5. Delete a Task"); 
        println!("6: Exit");   

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
                    todoListApp.show_tasks_default();
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
            3 => {

                println!("\n");

                todoListApp.show_tasks_default();  // TODO: How do you want to show the tasks
                
                'show_list: loop {
                    println!("\n");
                    println!("1. List Tasks by Default");
                    println!("2. List Done Tasks");
                    println!("3. List Uncompletedd Tasks");
                    println!("4. List Tasks by their Due Date");
                    println!("5. List Tasks by their Alphabet");
                    println!("Press q Return back to menu");
                    println!("\n");

                    let choice = match get_numeric_input("Enter your choice: \n") {
                        Some(value) => value,
                        None => {
                            println!("Invalid input, enter a proper input");
                            continue;
                        }
                    };

                    match choice {
                        0 => break,
                        1 => todoListApp.show_tasks_default(),
                        2 => todoListApp.show_tasks_done(),
                        3 => todoListApp.show_tasks_undone(),
                        4 => todoListApp.show_tasks_due_date(),
                        5 => todoListApp.show_tasks_by_alphabet(),
                        _ => {
                            println!("Invalid input, enter a proper input");
                            continue;
                        }
                    };
                };
            }
            4 => {
                
                println!("\n");
                todoListApp.show_tasks_default();
                println!("\n");

                    let index =  match get_numeric_input("Enter the task index to edit the task details: ") {
                        Some(value) => match value {
                            0 => break,
                            _ => value-1,
                        }
                        None => {
                            println!("Invalid input, enter proper number");
                            continue;
                        },
                    };

                    'edit: loop {

                        println!("\n");
                        todoListApp.show_task(index.into());
                        println!("\n1. Edit Description. 2. Edit due Date.");
                        println!("press q to go back to menu\n");
                        
                        let mut task = &mut Task::new(" ", None);
                        match todoListApp.get_task(index.into()) {
                            Some(value) => {task = value},
                            None => continue,
                        }

                        let choice = match get_numeric_input("Choose an option: ") {
                            Some(value) => value,
                            None => {
                                println!("Enter a valid option");
                                continue;
                            }
                        };

                        match choice {
                            0 => break,
                            1 => {
                                let new_description = match get_string_input("Write new description: ") {
                                    Some(input) => input,
                                    None => {
                                        println!("Enter a valid input");
                                        continue;
                                    }
                                };
                                task.description = new_description;
                            }, 
                            2 => {
                                let due_date;
                                'date: loop {
                                        match get_string_input("\nEnter new due date (YYYY-MM-DD) \nenter if you have no due date \nor press q to return back \n\n") {
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
                                        }
                                            None => continue 'edit,
                                    };
                                };

                                task.due_date = due_date;
                            },
                            _ => {
                                println!("Enter a valid choice");
                                continue;
                            }
                        }
                    };
                }
            5 => {

                println!("\n");
                todoListApp.show_tasks_default();
                println!("\n");

                let index =  match get_numeric_input("Enter the task index to edit the task details: ") {
                    Some(value) => match value {
                        0 => break,
                        _ => value-1,
                    }
                    None => {
                        println!("Invalid input, enter proper number");
                        continue;
                    },
                };

                println!("\n");
                todoListApp.show_task(index.into());
                println!("\n");

                let answer = match get_string_input("Are you sure you want to delete this event? (Y/n)\n") {
                    Some(value) => match value.as_str() {
                        "Y" => {
                            todoListApp.delete_task(index.into());
                            continue;
                        },
                        _ => continue,
                    },
                    None => continue,
                };
            }
            6 => {

                let answer = match get_string_input("Are you sure you want to quit the app? (Y/n)\n") {
                    Some(value) => match value.as_str() {
                        "Y" => break,
                        _ => continue,
                    }
                    None => continue,
                };
            }
            _ => println!("Invalid option, enter number between 1-4"),
            };
        }

    }
