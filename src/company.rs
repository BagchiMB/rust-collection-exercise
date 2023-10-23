use std::collections::HashMap;
use std::io;

fn add_employees(employees: &mut HashMap<String, Vec<String>>) {
    println!("*******************************************************************************************");
    println!("Welcome to the employee addition wing");
    println!("Enter employees in the format Add Sally to Engineering");
    println!("Here Sally is the name of the employee and Engineering is the department");
    println!("Please stick to this format");
    println!("Enter a statement and press enter, when you don't have anyone else you want to add to the company, press enter with a blank line, to come out of the employee addition wing");

    println!("Start Entering");

    loop {
        let mut entered_statement = String::new();
        io::stdin()
            .read_line(&mut entered_statement)
            .expect("Failed to read line");

        let v: Vec<&str> = entered_statement.split(' ').collect();

        let name = match v.get(1) {
            Some(name) => name,
            None => "",
        };

        let dept = match v.get(3) {
            Some(dept) => dept.replace("\n", ""),
            None => String::new(),
        };

        if name == "" || dept == "" {
            break;
        }

        let e = employees.entry(String::from(dept)).or_insert(vec![]);
        e.push(String::from(name))
    }

    println!("*******************************************************************************************");
}

fn display_by_department(employees: &mut HashMap<String, Vec<String>>) {
    println!("*******************************************************************************************");
    println!("All departments -->");

    for (k, _v) in employees.iter() {
        println!("{k} department");
    }

    println!("Enter department name");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let department_employee_vector = match employees.get(&choice.replace("\n", "")) {
        Some(department_employee_vector) => department_employee_vector,
        None => {
            println!("Wrong department name entered");
            return;
        }
    };

    for e in department_employee_vector {
        println!("{e}")
    }
    println!("*******************************************************************************************");
}

fn display_employees(mut employees: &mut HashMap<String, Vec<String>>) {
    println!("*******************************************************************************************");
    loop {
        println!("Press 1 to Display by Department");
        println!("Press 2 to Display all Employees");
        println!("Press anything else to go back to previous menu");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        if &choice[..1] == "1" {
            display_by_department(&mut employees)
        } else if &choice[..1] == "2" {
            for (k, v) in employees.iter() {
                println!("{k} department employees");
                for e in v {
                    println!("{e}")
                }
            }
        } else {
            break;
        }
    }
    println!("*******************************************************************************************");
}

pub fn add_and_display_employee_interface() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    add_employees(&mut employees);

    loop {
        println!("Welcome to the company");
        println!("Press 1 to add Employees");
        println!("Press 2 to view Employees");
        println!("Press anything else to exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        if &choice[..1] == "1" {
            add_employees(&mut employees)
        } else if &choice[..1] == "2" {
            display_employees(&mut employees)
        } else {
            break;
        }
    }
}
