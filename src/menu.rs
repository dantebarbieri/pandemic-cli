use std::{fmt::Display, io::Write};

use crossterm::style::{Color, Stylize};

pub fn menu_cancelable(title: &str, options: &[impl Display]) -> usize {
    loop {
        println!("{:=^60}", format!("  {}  ", title));
        println!("\t{}. {}", 0, "Go Back to Previous Menu");
        for i in 0..options.len() {
            println!("\t{}. {}", i + 1, options[i]);
        }
        print!("{}: ", "Enter selection");
        std::io::stdout().flush();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<usize>() {
                Ok(value) => {
                    if value <= options.len() {
                        return value;
                    } else {
                        println!(
                            "{}: Input should be at most {}",
                            "Error".with(Color::Red).bold(),
                            options.len()
                        );
                    }
                },
                Err(_) => println!(
                    "{}: Invalid input: '{}'",
                    "Error".with(Color::Red).bold(),
                    input
                ),
            },
            Err(_) => println!("{}: Could not read input.", "Error".with(Color::Red).bold()),
        }
    }
}

pub fn menu(title: &str, options: &[impl Display]) -> usize {
    loop {
        println!("{:=^60}", format!("  {}  ", title));
        for i in 0..options.len() {
            println!("\t{}. {}", i + 1, options[i]);
        }
        print!("{}: ", "Enter selection");
        std::io::stdout().flush();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<usize>() {
                Ok(value) => {
                    if value >= 1 && value <= options.len() {
                        return value;
                    } else {
                        println!(
                            "{}: Input should be between 1 and {}",
                            "Error".with(Color::Red).bold(),
                            options.len()
                        );
                    }
                },
                Err(_) => println!(
                    "{}: Invalid input: '{}'",
                    "Error".with(Color::Red).bold(),
                    input
                ),
            },
            Err(_) => println!("{}: Could not read input.", "Error".with(Color::Red).bold()),
        }
    }
}
