use std::env;
use std::io;
mod employee;
use employee::Employee;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut input: String = String::new();
    let mut name = String::from("David");
    let mut age = 23;
    let mut salary = 17000.0;
    let mut raise = 1.04;
    let mut years = 3;

    if args.len() > 1 {
        println!("Enter employee name:");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                name = input.trim().to_string();
            }
            Err(_) => println!("Error reading input"),
        }

        println!("Enter employee age:");

        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading age");

        age = input.trim().parse().expect("Not a valid age");

        println!("Enter employee salary:");
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading salary");
        salary = input.trim().parse().expect("Not a valid salary");

        println!("Enter employee raise factor:");
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading raise");
        raise = input.trim().parse().expect("Not a valid raise factor");

        println!("Enter year to wait:");
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading years");
        years = input.trim().parse().expect("Not a valid year");
    }

    let e = Employee::new(name, age, salary, raise);

    let future = e.future_salary(years);
    println!("{}", e.to_string());
    println!("----------------------------");
    println!("The salary in {} years will be: {}", years, future);
}
