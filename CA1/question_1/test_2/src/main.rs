use std::io;

fn main() {
    loop {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Employe payroll calculator");
    println!("What is your name");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let name = input1.trim().to_lowercase;

    println!("How many hour have you worked");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let hours:f32 = input2.trim().parse().expect("Not a valid number");

    let salary:f32 = 3_000.0 * hours;
    let extra_hours:f32 = 4_500.0 * hours;
    let gross_salary:f32 = salary + extra_hours;
    
    let pat:f32 = 2 - gross_salary

    if gross_salary >0.0 && salary <=3_000.0{
        println!("gross_salary before tax is {}",gross_salary)
    }
    else if gross_salary >0.0 && salary <=3_000.0{
        println!("gross salary after tax is {}",pat);
    }

    let mut count = String::new();
    println!("will you like to calculate again?",(y/n));
    io::stdin().read_line(&mut count).expect("Not a valid String");
    
    if answer == "n"{
        break;
    }    
}
