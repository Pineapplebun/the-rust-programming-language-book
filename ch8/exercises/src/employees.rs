use std::collections::HashMap;
use std::io;

pub fn ask_questions() {
    let mut hmap = HashMap::new();
    println!("Welcome to Company XYZ's CLI tool.");
    println!("Here are the list of commands you can enter:");
    println!("Add <Name> to <Department>");
    println!("Retrieve all");
    println!("Retrieve from <Department>");
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Not a valid command");
        let lc = command.to_lowercase();
        let args = lc.split_whitespace().collect();
        if is_done(&args) {
            break;
        } else if is_retrieve_all(&args) {
            retrieve_all(&mut hmap);
        } else if is_retrieve_from(&args) {
            retrieve_from(args[2], &mut hmap);
        } else if is_add_to(&args) {
            add_to(args[1], args[3], &mut hmap);
        } else {
            println!("Not a valid command");
        }
    }
}

fn is_done(args: &Vec<&str>) -> bool {
    args.len() == 1 && args[0] == "done"
}

fn is_retrieve_all(args: &Vec<&str>) -> bool {
    args.len() == 2 && args[0] == "retrieve" && args[1] == "all"
}

fn is_retrieve_from(args: &Vec<&str>) -> bool {
    args.len() == 3 && args[0] == "retrieve" && args[1] == "from"
}

fn is_add_to(args: &Vec<&str>) -> bool {
    args.len() == 4 && args[0] == "add" && args[2] == "to"
}

fn retrieve_all(hmap: &mut HashMap<String, Vec<String>>) {
    for (_department, employees) in hmap {
        for employee in employees {
            println!("{}", employee);
        }
    }
}

fn retrieve_from(department: &str, hmap: &mut HashMap<String, Vec<String>>) {
    let employees: Option<&Vec<String>> = hmap.get(&String::from(department));
    match employees {
        Some(e) => {
            let mut employees_clone = e.clone();
            employees_clone.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            for employee in employees_clone {
                println!("{}", employee);
            }
        }
        None => {
            println!("There are no employees in {}", department);
        }
    }
}

fn add_to(name: &str, department: &str, hmap: &mut HashMap<String, Vec<String>>) {
    let employees = hmap.entry(String::from(department)).or_insert(vec![]);
    employees.push(String::from(name));
    println!("Added {} to {}", name, department);
}
