fn main() {
    
    let fullname = "Ani chikamso confidence";
    let department = "Department";
    let uni = "PAU";

    let mut school = "School of science".to_string();
    //push string
    school.push_string(" and technology");

    println!("My name si: {}", fullname);
    // check length
    println!("The length my fullname is: {}",fullname.len());
    println!("I am a studnt of {} Department", department);
    println!("{}",school);
    println!("{}",uni);
}
