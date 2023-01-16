use std::io::Write;

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
            Ok(value) => Ok(value - 1),
            Err(_) => Err(input)
        },
        Err(_) => Err(String::from("Failed to read line"))
    }
}