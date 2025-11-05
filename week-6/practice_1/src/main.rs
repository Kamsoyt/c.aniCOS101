fn main() {
    let name = "Chikamso Ani";
    let uni:&str = "Pan-atlantic University";
    let addr:&str = "km 52 lekki-Epe Expressway";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}",uni,addr);

    let department:&'static str = "Computer science";
    let school:&'static str = "School of science and technology";
    print!("Department: {}, \nschool: {}",department,school);
    

}
