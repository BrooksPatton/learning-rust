// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

// {
//     engineering: ['sally', 'amir'],
//     sales: ['bob']
// }

// Add <name> to <department>
// Get employees
// Get employees from <department>
use std::io;

#[derive(Debug)]
enum Command {
    add {name: String, department: String},
    get,
    get_by_department(String),
}

fn main() -> io::Result<()> {
    // ask the user for input
    println!("{}", "Enter your command: ");
    let mut raw_input = String::new();
    io::stdin().read_line(&mut raw_input)?;

    // Get the user command
    let command_parts: Vec<_> = raw_input.trim().split(' ').collect();
    let command;

    match command_parts[0] {
        "add" => command = Command::add {name: command_parts[1].to_string(), department: command_parts[command_parts.len() - 1].to_string()},
        "get" => println!("nothing here yet"),// get can be two things, having a department or not :),
        _ => println!("hello")
    };
    println!("{:?}", command_parts[0]);
    // extract parts of the command
    Ok(())
}
