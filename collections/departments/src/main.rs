#![warn(clippy::pedantic)]
use std::collections::HashMap;
use std::io::stdin;

enum Cmd {
    Add { name: String, department: String },

    Remove { name: String, department: String },

    ListDepartment { department: String },

    ListCompany,
    Help,
    Unkown,
    Exit,
}

fn resolve_cmd(cmd_str: &str) -> Cmd {
    if cmd_str == "help" {
        Cmd::Help
    } else if cmd_str == "list" {
        return Cmd::ListCompany;
    } else if cmd_str.starts_with("list-department") {
        let split = cmd_str.split_ascii_whitespace().collect::<Vec<&str>>();
        if split.len() != 2 {
            println!("Malformed list-department cmd");
            return Cmd::Unkown;
        }
        Cmd::ListDepartment {
            department: split[1].to_string(),
        }
    } else if cmd_str.starts_with("add") {
        let split = cmd_str.split_ascii_whitespace().collect::<Vec<&str>>();
        if split.len() != 3 {
            println!("Malformed add cmd");
            return Cmd::Unkown;
        }
        Cmd::Add {
            name: split[1].to_string(),
            department: split[2].to_string(),
        }
    } else if cmd_str.starts_with("remove") {
        let split = cmd_str.split_ascii_whitespace().collect::<Vec<&str>>();
        if split.len() != 3 {
            println!("Malformed remove cmd");
            return Cmd::Unkown;
        }
        Cmd::Remove {
            name: split[1].to_string(),
            department: split[2].to_string(),
        }
    } else if cmd_str == "exit" {
        Cmd::Exit
    } else {
        Cmd::Unkown
    }
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter cmds ('help' for a list of cmds): ");
        let mut cmd = String::new();
        stdin().read_line(&mut cmd).expect("Failed to read line");
        match resolve_cmd(cmd.trim()) {
            Cmd::Add { name, department } => {
                if departments.contains_key(&department) {
                    let employee_list = departments.get_mut(&department).unwrap();
                    employee_list.push(name.to_string());
                } else {
                    departments.insert(department.to_string(), vec![name.to_string()]);
                }
            }

            Cmd::Remove { name, department } => {
                if departments.contains_key(&department) {
                    let employee_list = departments.get_mut(&department).unwrap();
                    let employee_pos = employee_list.iter().position(|employee| *employee == name);
                    if let Some(index) = employee_pos {
                        employee_list.remove(index);
                    } else {
                        println!("No Employee named, {name} found in {department}");
                    }
                } else {
                    println!("Department {department} does not exist");
                }
            }

            Cmd::ListCompany => {
                for department in departments.keys() {
                    println!("{department}:");
                    for (mut number, employee) in
                        departments.get(department).unwrap().iter().enumerate()
                    {
                        number += 1;
                        println!("  {number}. {employee}");
                    }
                }
            }

            Cmd::ListDepartment { department } => match departments.get(&department) {
                Some(employees) => {
                    println!("{department}:");
                    for (mut number, employee) in employees.iter().enumerate() {
                        number += 1;
                        println!("  {number}. {employee}");
                    }
                }
                None => println!("No department named {department}"),
            },

            Cmd::Help => {
                println!("---------------------------------------------------------");
                println!("Department Lister 1.0.0");
                println!("Commands are:");
                println!("Add: add <name> <department>");
                println!("Remove: remove <name> <department>");
                println!("List Company: list");
                println!("List Department: list-department");
                println!("Help: help");
                println!("Exit: exit");
                println!("---------------------------------------------------------");
            }

            Cmd::Exit => break,

            Cmd::Unkown => (),
        }
    }
}
