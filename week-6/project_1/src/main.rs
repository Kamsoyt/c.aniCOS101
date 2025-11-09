use std::io;
fn main(){
    println!("what do you want to get");
    println!("Input p, f, a, e");
    println!("p = Poundo yam/ekinkaikin soup - $3_200
              f = Fried rice/chiken - $3_000
              a = Amala & Egusi soup - $2_500
              e = Eba & Egusi soup - $2_000
              w = White rice & stew - $2_500");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What do you want to get");
    io::stdin().read_line(&mut input1).expect("NOt a valid input");
    let food = input1.trim().to_lowercase();

    println!("How many do you want get");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let qty:f32 = input2.trim().parse().expect("Not a valid number");

    let amount_p:f32 = 3_200.0 * qty;
    let amount_f:f32 = 3_000.0 * qty;
    let amount_a:f32 = 2_500.0 * qty;
    let amount_e:f32 = 2_000.0 * qty;
    let amount_w:f32 = 2_500.0 * qty;

    let dis_amountp:f32 = 5.0 / 100.0 * (3_200.0 * qty);
    let dis_amountf:f32 = 5.0 / 100.0 * (3_000.0 * qty);
    let dis_amounta:f32 = 5.0 / 100.0 * (2_500.0 * qty);
    let dis_amounte:f32 = 5.0 / 100.0 * (2_000.0 * qty);
    let dis_amountw:f32 = 5.0 / 100.0 * (2_500.0 * qty);

    let t_amountp:f32 = amount_p - dis_amountp;
    let t_amountf:f32 = amount_f - dis_amountf;
    let t_amounta:f32 = amount_a - dis_amounta;
    let t_amounte:f32 = amount_e - dis_amounte;
    let t_amountw:f32 = amount_w - dis_amountw;

    if food == "p" && amount_p >= 10_000.0{
        println!("Your discount amount is {}",t_amountp);
    }
    else if food == "p" && amount_p < 10_000.0{
        println!("Your amount is {}",amount_p);
    }
    else if food == "f" && amount_f >=10_000.0{
        println!("Your discount amount is {}",t_amountf);
    }
    else if food == "f" && amount_f < 10_000.0{
        println!("Your amount is {}",amount_f);
    }
    else if food == "a" && amount_a >= 10_000.0{
        println!("Your discount amount is {}",t_amounta);
    }
    else if food == "a" && amount_a < 10_000.0{
        println!("You amount is {}",amount_a);
    }
    else if food == "e" && amount_e >= 10_000.0{
        println!("Your discount amount is {}",t_amounte);
    }
    else if food == "e" && amount_e < 10_000.0{
        println!("Your amount is {}",amount_e);
    }
    else if food == "w" && amount_w >= 10_000.0{
        println!("Your discount amount is {}",t_amountw);
    }
    else if food == "w" && amount_w < 10_000.0{
        println!("Your amount is {}",amount_w);
    }
    else{
        println!("Invalid input");
    }
} 
