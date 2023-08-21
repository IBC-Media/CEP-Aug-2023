use std::io;

// Define the Employee struct
struct Employee {
    employee_name: String,
    employee_id: u32,
    email: String,
    age: u32,
    phone_number: String,
}

impl Employee {
    // Create a new Employee instance
    fn new(name: String, id: u32, email: String, age: u32, phone: String) -> Self {
        Employee {
            employee_name: name,
            employee_id: id,
            email,
            age,
            phone_number: phone,
        }
    }
}

// Function to find an employee by ID
fn find_employee_by_id(employees: &[Employee], id: u32) -> Option<&Employee> {
    employees.iter().find(|&emp| emp.employee_id == id)
}

// Function to find employees with the same age
fn find_employees_by_age(employees: &[Employee], target_age: u32) -> Vec<&Employee> {
    employees.iter().filter(|&emp| emp.age == target_age).collect()
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    loop {
        println!("Enter employee details (or type 'exit' to quit):");
        
        // Collect employee details from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        let parts: Vec<&str> = input.trim().split(',').collect();
        if parts.len() != 5 {
            println!("Invalid input. Please provide all fields separated by commas.");
            continue;
        }

        let name = parts[0].trim().to_string();
        let id: u32 = parts[1].trim().parse().expect("Invalid ID");
        let email = parts[2].trim().to_string();
        let age: u32 = parts[3].trim().parse().expect("Invalid age");
        let phone = parts[4].trim().to_string();

        let employee = Employee::new(name, id, email, age, phone);
        employees.push(employee);
    }

    // Print employee details by ID
    println!("Enter an employee ID to search:");
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).expect("Failed to read line");
    let search_id: u32 = id_input.trim().parse().expect("Invalid ID");
    
    if let Some(emp) = find_employee_by_id(&employees, search_id) {
        println!("Employee found:\nName: {}\nID: {}\nEmail: {}\nAge: {}\nPhone: {}",
            emp.employee_name, emp.employee_id, emp.email, emp.age, emp.phone_number);
    } else {
        println!("Employee with ID {} not found.", search_id);
    }

    // Print employees with the same age
    println!("Enter an age to search for employees with that age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let search_age: u32 = age_input.trim().parse().expect("Invalid age");

    let matching_employees = find_employees_by_age(&employees, search_age);
    if matching_employees.is_empty() {
        println!("No employees found with age {}.", search_age);
    } else {
        println!("Employees with age {}:", search_age);
        for emp in matching_employees {
            println!("Name: {}\nID: {}\nEmail: {}\nAge: {}\nPhone: {}\n",
                emp.employee_name, emp.employee_id, emp.email, emp.age, emp.phone_number);
        }
    }
}
