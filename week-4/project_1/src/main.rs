// Rust root of a quadractic equation

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value for a:");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let a:f32 = input1.trim().parse().expect("Not a valid String");

    println!("Enter the value for b:");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value for c:");
    io::stdin().read_line(&mut input3).expect("Not a valid String");
    let c:f32 = input3.trim().parse().expect("Not a valid number");
    
    let d=b.powf(2.0)-4.0*a*c;

    if d > 0.0{
        let r_one=(-b + d.sqrt() )/(2.0*a);
        let r_two=(-b - d.sqrt() )/(2.0*a);
        println!("There are two real roots");
        println!("The root are {} and {}",r_one,r_two );
    }
    else if d == 0.0{
        let root=-b/(2.0*a); // since the sqrt of 0 = 0
        println!("There is exactly one root");
        println!("The root is {}",root );
    }
    else {
        println!("There are no real roots");
    }
}