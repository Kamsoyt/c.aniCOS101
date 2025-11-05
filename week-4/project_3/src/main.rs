//Formular to solve compound interest

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

    println!("Compound interest calculator");
    println!("Enter the value for p");
    io::stdin().read_line(&mut input1).expect("Failed to input number");
    let p:f32 = input1().trim().parse().expect("Not a valid number");

    println!("Enter the value for r");
    io::stdin().read_line(&mut input2).expect("Failed to input number");
    let r:f32 = input2().trim().parse().expect("Not a valid number");

    println!("Enter the value for n");
    io::stdin().read_line(&mut input3).expect("Failed to input number");
    let r:f32 = input3().trim().parse().expect("Not a valid number");

    println!("Enter the value for t");
    io::stdin().read_line(&mut input4).expect("Failed to input number");
    let t:f32 = input4().trim().parse().expect("Not a valid number");


    let a = p * (1.0 + r / n).powf(n * t);

}
