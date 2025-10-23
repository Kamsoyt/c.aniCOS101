// Rust program to read the height of a person
// and then print if person is tall, dwarf,
// or average height person

use std::io;

fn main() 
{
    let mut input = String::new();

    println!("\nEnter your height (in centimeteres):");
    io::stdin().read_line(&mut input).expect("Number is not a valid string");
    let height:f32= input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0
    {
        println!("You are of average height");
    }
    else if height >= 170.0 && height <= 195.00
    {
        println!("You are tall");
    }
    else if height < 150.0 && height < 100.0
    {
        println!("You are dwarf");
    }
    else if height < 50.0 && height < 60.0
    {
        println!("Abnormal height");
    }
} 