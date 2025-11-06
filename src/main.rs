
fn main () {
    let mut employees: Vec<String> = Vec::new();
    let emp1 = String::from("Alice");
    let emp2 = String::from("Bob");

    employees.push(emp1);
    employees.push(emp2);

    employees.push(String::from("Charlie"));

    let second_emp = employees.get(1);
    match second_emp {
        Some(emp) => println!("The second employee is {}", emp),
        None => println!("There is no second employee")
    }

    for emp in &employees {
        println!("{}", emp);
    }
    
}