use std::io;

fn main() {

    loop {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("compound interest calculator");
    println!("Enter your value for p");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let p:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your value for r");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let r:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your value for t");
    io::stdin().read_line(&mut input3).expect("Not a valid String");
    let t:f32 = input3.trim().parse().expect("Not a valid number");

    let amount:f32 = p * ( 1.0 + (r / 100.0)).powf(t);
    println!("amount: {}", amount);

    let ci:f32 = amount - p;
    println!("compound interest: {}", ci);

    let mut choice = String::new();
    println!("Do you want to redo the calculation? (y/n)");
    io::stdin().read_line(&mut choice).expect("Not making sence");
    let choice = choice.trim().to_lowercase();

    if choice == "n"{
        break;
    }
    else if choice == "y"{
        println!("restarting code");
        main();

    }
}


    


}
