// Rust program to manage simple purchace orders for a computer store.

use std::io;

fn main() {
    loop {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();

    println!("Items available: latop,monitor,keyboard,headset");
    
    println!("What item do you want?");
    io::stdin().read_line(&mut input1).expect("Not a valid String");
    let item = input1.trim().to_lowercase();

    println!("How item do you want to buy?");
    io::stdin().read_line(&mut input2).expect("Not a valid String");
    let quantity:f32 = input2.trim().parse().expect("Not a valid number");

    let laptop:f32 = 550_000.0;
    let monitor:f32 = 120_000.0;
    let keyboard:f32 = 15_000.0;
    let headset:f32 = 25_000.0;

    let total_l = laptop * quantity;
    let total_m = monitor * quantity;
    let total_k = keyboard * quantity;
    let total_h = headset * quantity;

    //total cost after discount
    let tc_l = total_l:f32 - (7.0 * total_l);
    let tc_m = total_m:f32 - (7.0 * total_m);
    let tc_k = total_k:f32 - (7.0 * total_k);
    let tc_h = total_h:f32 - (7.0 * total_h);

    if item == "laptop" && total_l >500_000.0{
        println!("Your amount to pay is {}",tc_l);
    }
    if else item == "laptop" && total_l <500_000.0{
        println!("Your amount to pay is {}",total_l);
    }
    if item == "monitor" && total_m >500_000.0{
        println!("Your amount to pay is {}",tc_m);
    }
    if else item == "monitor" && total_m <500_000.0{
        println!("Your amount to pay is {}",total_m)
    }
    if item == "keyboard" && total_k >500,000.0{
        println!("Your amount to pay is {}",tc_k);
    }
    if else item == "keyboard" total_k <500_000.0{
        println!("Your amount to pay is {},total_k");
    }
    if item == "headset" total_h >500_000.0{
        println!("Your amount to pay is {}",tc_h);
    }
    if else item == "headset" total_h <500_000.0{
        println!("your amount to pay is {}",total_h);
    }

    let mut choice = String::new();
    println!("Will you like to purcase another item? (y/n)");
    io::stdin().readline(&mut choice).expect("Not a valid String");
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