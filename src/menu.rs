use std::io::Write;

pub fn menu_cancelable(title: &str, options: &[&str]) -> Result<usize, String> {
    println!("{:=^64}", title);
    let mut options = Vec::from(options);
    options.insert(0, "Go Back to Previous Menu");
    for i in 0..options.len() {
        println!("\t{}. {}", i, options[i]);
    }
    print!("{}: ", "Enter selection");
    std::io::stdout().flush();
    let mut input = String::new();
    match std::io::stdin()
    .read_line(&mut input) {
        Ok(_) => match input.trim().parse::<usize>() {
            Ok(value) => Ok(value),
            Err(_) => Err(input)
        },
        Err(_) => Err(String::from("Failed to read line"))
    }
}

pub fn menu(title: &str, options: &[&str]) -> Result<usize, String> {
    println!("{:=^64}", title);
    for i in 0..options.len() {
        println!("\t{}. {}", i + 1, options[i]);
    }
    print!("{}: ", "Enter selection");
    std::io::stdout().flush();
    let mut input = String::new();
    match std::io::stdin()
    .read_line(&mut input) {
        Ok(_) => match input.trim().parse::<usize>() {
            Ok(value) => Ok(value),
            Err(_) => Err(input)
        },
        Err(_) => Err(String::from("Failed to read line"))
    }
}