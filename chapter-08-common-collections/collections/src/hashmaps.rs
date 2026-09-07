use std::collections::HashMap;

pub fn run() {
    println!("hashmaps!");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Team {}; Score {}\n", team_name, score);

    for (key, value) in scores {
        println!("Key: {}; Value: {}", key, value);
    }

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // cannot use field_name and field_value anymore...
    // ownership belongs to the map

    // Changing a value of a particular key
    let field_name = String::from("Favorite Color");
    let field_value = map.get(&field_name).unwrap();
    println!("Key is {}; Value is {}", team_name, field_value);

    let field_value = String::from("Yellow");
    map.insert(field_name, field_value);

    let field_name = String::from("Favorite Color");
    let field_value = map.get(&field_name).unwrap();
    println!("Key is {}; Value is {}", team_name, field_value);

    let mut people = HashMap::new();
    people.insert(String::from("John"), 10);
    people.entry(String::from("John")).or_insert(50); // value is 10
    people.entry(String::from("Pork")).or_insert(50); // value is 50

    println!("{people:?}");

    let text = "hello world wonderful world";
    let mut freq = HashMap::new();

    for word in text.split_whitespace(){
        let count = freq.entry(word).or_insert(0); // get address of value
        *count += 1; // increment value from the value address
    }
    println!("FreqMap = {freq:?}");

    println!();
    company_manager_app();
}

use std::io;

fn company_manager_app() {
    let mut employee_map: HashMap<String, Vec<String>> = HashMap::new();
    println!("===================================");
    println!("Welcome to the Company Manager App!");

    loop {
        println!("===================================");
        println!("Please enter the next activity type:");
        println!("Enter (1) for adding an 'Employee' to 'Department'");
        println!("Enter (2) to list all people in a department");
        println!("Enter (3) to list all people in the company");
        println!("Enter (4) to quit");
        println!("===================================");

        let mut entry = String::from("");
        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read line");

        let entry = match entry.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match entry {
            1 => employee_addition(&mut employee_map),
            2 => get_employees_from_department(&mut employee_map),
            3 => println!("{employee_map:?}"),
            4 => break,
            _ => continue,
        }
    }
}

fn employee_addition(employee_map: &mut HashMap<String, Vec<String>>) {
    println!("Enter in employee name:");
    let mut employee_name = String::from("");
    io::stdin()
        .read_line(&mut employee_name)
        .expect("Failed to read line");
    employee_name = employee_name.trim().to_string();

    println!("Enter in department name:");
    let mut department_name = String::from("");
    io::stdin()
        .read_line(&mut department_name)
        .expect("Failed to read line:");
    department_name = department_name.trim().to_string();

    println!("Employee '{}' added to Department '{}'!", employee_name, department_name);
    let vec = employee_map.entry(department_name).or_insert(vec![]);
    vec.push(employee_name);

}

fn get_employees_from_department(employee_map: &mut HashMap<String, Vec<String>>) {
    println!("Enter in department name:");
    let mut department_name = String::from("");
    io::stdin()
        .read_line(&mut department_name)
        .expect("Failed to read line");
    department_name = department_name.trim().to_string();

    match employee_map.get(&department_name) {
        Some(employees) => println!("{employees:?}"),
        None => println!("No employees in this deparment..."),
    }

}
