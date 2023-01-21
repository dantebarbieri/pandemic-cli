use std::{fmt::Display, io::Write};

use crossterm::style::{Color, Stylize};

pub fn menu_cancelable(title: &str, options: &[impl Display]) -> usize {
    menu_injectable(title, "Go Back to Previous Menu", options)
}

pub fn menu_injectable(title: &str, first_option: &str, options: &[impl Display]) -> usize {
    loop {
        println!("{:=^94}", format!(" {} ", title));
        println!("\t{}. {}", 0, first_option);
        for i in 0..options.len() {
            println!("\t{}. {}", i + 1, options[i]);
        }
        print!("{}: ", "Enter selection");
        std::io::stdout().flush().unwrap_or_default();
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
        println!("{:=^94}", format!(" {} ", title));
        for i in 0..options.len() {
            println!("\t{}. {}", i + 1, options[i]);
        }
        print!("{}: ", "Enter selection");
        std::io::stdout().flush().unwrap_or_default();
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
