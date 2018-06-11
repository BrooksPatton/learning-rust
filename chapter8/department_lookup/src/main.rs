// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

// {
//     engineering: ['sally', 'amir'],
//     sales: ['bob']
// }

// Add <name> to <department>
// Get employees
// Get employees from <department>
use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum Command {
    Add {name: String, department: String},
    Get,
    GetByDepartment(String),
}

fn main() -> io::Result<()> {
    let mut departments = HashMap::new();

    loop {
        // ask the user for input
        println!("{}", "Enter your command: ");
        let mut raw_input = String::new();
        io::stdin().read_line(&mut raw_input)?;

        // Get the user command
        let command_parts: Vec<_> = raw_input.trim().split(' ').map(|c| c.to_lowercase()).collect();
        let last_command_part = &command_parts[command_parts.len() - 1];
        let command;

        match command_parts[0].as_ref() {
            "add" => command = Command::Add {name: command_parts[1].to_string(), department: last_command_part.to_string()},
            "get" => {
                if command_parts.len() == 2 || command_parts.len() == 4 {
                    match last_command_part.as_ref() {
                        "employees" => command = Command::Get,
                        _ => command = Command::GetByDepartment(last_command_part.to_string()),
                    }
                } else {
                    panic!("Invalid command");
                }
            },
            _ => break
        };
        
        match command {
            Command::Add{name, department} => {
                let mut d = departments.entry(department).or_insert(vec![]);
                d.push(name);
                },
            Command::Get => {
                for (key, val) in departments.iter() {
                    println!("--- {} ---", key);
                    for name in val.iter() {
                        println!("{}", name);
                    }
                }
            },
            Command::GetByDepartment(department) => {
                match departments.get(&department) {
                    Some(names) => {
                        println!("--- {} ---", department);
                        for name in names.iter() {
                            println!("{}", name);
                        }
                    },
                    _ => println!("Department not found")
                }
            },
        };
    }

    // extract parts of the command
    Ok(())
}