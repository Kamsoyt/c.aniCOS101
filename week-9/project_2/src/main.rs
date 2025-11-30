use std::fs::write;

struct Student {
    name: &'static str,
    matric: &'static str,
    dept: &'static str,
    level: u32,
}

fn main() {
    let students = vec![
        Student { name: "Oluchi Mordi", matric: "ACC10211111", dept: "Accounting", level: 300 },
        Student { name: "Adams Aliyu", matric: "ECO1010101", dept: "Economics", level: 100 },
        Student { name: "Shania Bolade", matric: "CSC1032288", dept: "Computer", level: 200 },
        Student { name: "Adekunle Gold", matric: "EEE1020202", dept: "Electrical", level: 200 },
        Student { name: "Blanda Edemowo", matric: "MEE1020201", dept: "Mechanical", level: 100 },
    ];

    // Display
    for s in &students {
        println!("{} | {} | {} | {}", s.name, s.matric, s.dept, s.level);
    }

    // Save into file
    let mut out = String::from("PAU SMIS\nStudent Name,Matric Number,Department,Level\n");
    for s in &students {
        out.push_str(&format!("{},{},{},{}\n", s.name, s.matric, s.dept, s.level));
    }

    write("students.csv", out).unwrap();
}
