use std::io;

pub fn read_line() -> String {
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("Error reading line");
        
    return string.clone();
}

pub fn click_enter_to_continue() {
    println!("Click enter to continue...");
    let mut click = String::new();
    io::stdin()
        .read_line(&mut click)
        .expect("Error waiting to continue");
}

pub fn parse_input_to_i32(variable_name: &str, label: &str) -> i32 {
    let msg: String = format!("Error reading {}", variable_name);
    let msg: &str = &msg[..];

    return loop {
        println!("{label}");

        let string = read_line();

        match string.trim().parse::<i32>() {
            Ok(value) => break value,
            Err(value) => invalid_option(value.to_string()),
        };
    };
}

pub fn invalid_option(option: String) {
    show_decoration();
    println!("Invalid option: {}", option);
}

pub fn show_decoration() {
    println!("--------------------------------------------------");
}