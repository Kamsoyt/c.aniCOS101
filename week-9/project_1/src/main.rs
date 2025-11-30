use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Drink categories based on the provided table
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    // Create output file
    let mut file = File::create("drinks.txt")?;

    // Write formatted table data
    writeln!(file, "High-Quality Drink Categories\n")?;
    
    writeln!(file, "Lager:")?;
    for item in &lager {
        writeln!(file, "  - {}", item)?;
    }
    writeln!(file)?;

    writeln!(file, "Stout:")?;
    for item in &stout {
        writeln!(file, "  - {}", item)?;
    }
    writeln!(file)?;

    writeln!(file, "Non-Alcoholic:")?;
    for item in &non_alcoholic {
        writeln!(file, "  - {}", item)?;
    }

    println!("File 'drinks.txt' created successfully.");
    Ok(())
}

