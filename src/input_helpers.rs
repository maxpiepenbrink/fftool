use std::io;
use std::io::Write;

pub fn get_number_input(message: &str) -> i32 {
    let mut entry = String::new();

    print!("{}", message);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut entry).expect("error: unable to read user input");
    return entry.trim().parse().unwrap();
}

pub fn get_string_input(message: &str) -> String {
    let mut entry = String::new();

    print!("{}", message);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut entry).expect("error: unable to read user input");
    return entry.trim().to_string();
}