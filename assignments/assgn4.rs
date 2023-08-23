struct Student {
    name: String,
    email: String,
    phno: String,
    id: u32,
}

fn main() {
    // Create a vector to store student details
    let mut students: Vec<Student> = Vec::new();

    // Adding sample student details to the vector
    students.push(Student {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        phno: String::from("123-456-7890"),
        id: 1,
    });

    students.push(Student {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        phno: String::from("987-654-3210"),
        id: 2,
    });

    students.push(Student {
        name: String::from("Charlie"),
        email: String::from("charlie@example.com"),
        phno: String::from("555-555-5555"),
        id: 3,
    });

    // Accessing student details using index with error handling
    let index = 1; // Change this index to test different scenarios
    match students.get(index) {
        Some(student) => {
            println!("Student Details:");
            println!("Name: {}", student.name);
            println!("Email: {}", student.email);
            println!("Phone Number: {}", student.phno);
            println!("ID: {}", student.id);
        }
        None => {
            println!("Student not found at index {}", index);
        }
    }
}
