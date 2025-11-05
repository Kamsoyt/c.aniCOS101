use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

    println!("Enter your name");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let name = input1.trim().to_lowercase();

    println!("Enter your first text score");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let firstscore:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your second text score");
    io::stdin().read_line(&mut input3).expect("Not a valid String");
    let secondscore:f32 = input3.trim().parse().expect("Not a valid Number");

    println!("Enter your third text score");
    io::stdin().read_line(&mut input4).expect("Not a valid String");
    let thirdscore:f32 = input4.trim().parse().expect("Not a valid number");

    let avg:f32 = (firstscore + secondscore + thirdscore ) / 3.0;
    let avd_rnd:f32 = (avg * 100.0).round() / 100.0;

    if avg >=70.0 && avg <=100.0{
        println!("Congratulations {} You got an A({})",name,avd_rnd);
    }
    else if avg >=60.0 && avg <=69.00{
        println!("Congratulations {} You got a B({})",name,avd_rnd);
    }
    else if avg >=50.0 && avg <=59.0{
        println!("Congratulations {} You got a C({})",name,avd_rnd);
    }
    else if avg >=45.0 && avg <=49.0{
        println!("Congratulations {} You got a D({})",name,avd_rnd);
    }
    else if avg >=0.0 && avg <=44.0{
        println!("Congratulations {} You got an F({})",name,avd_rnd);
    }
}
