use std::io;

fn main(){

    println!("Temperature converter and checker");

    let mut input1 = String::new();

    println!("Input the temperature in celsius");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let temperature:f32 = input1.trim().parse().expect("Not a valid number");

    let fehrenheit:f32 = (9.0 / 5.0) * c + 32.0;
    let kevin:f32 = c + 273.15;

    println!("The temperature in celsius");
    println!("The temperature in fehrenheit {}",fehrenheit);
    println!("The temperature in kevin {}",kevin);

    if  temperature <=0.0 && temperature <=-273.0{
        println!("Freezing point");
    }
    else if  temperature > 0.0 && temperature <=30.0{
        println!("Normal range");
    }
    else if temperature {
        println!("Hot temperature");
    }
}
