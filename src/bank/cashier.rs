use crate::bank::users::Account;
use std::io;

pub fn menu(account: &mut Account) {
    loop {
        let option: u8 = get_option();

        match option {
            1 => deposit(account),
            2 => withdraw(account),
            3 => consult(account),
            _ => break,
        }
    }
}

fn parse_input_to_i32(variable_name: &str, label: &str) -> i32 {
    let msg: String = format!("Error reading {}", variable_name);
    let msg: &str = &msg[..];

    return loop {
        println!("{label}");

        let mut string = String::new();
        io::stdin().read_line(&mut string).expect(msg);

        match string.trim().parse::<i32>() {
            Ok(value) => break value,
            Err(value) => invalid_option(value.to_string()),
        };
    };
}

fn deposit(account: &mut Account) {
    let amount: i32 = parse_input_to_i32("amount", "Enter amount to deposit");

    account.deposit(amount);
    consult(account);
}

fn withdraw(account: &mut Account) {
    let amount: i32 = parse_input_to_i32("amount", "Enter amount to withdraw:");

    if amount >= 0 && amount <= account.amount() {
        account.withdraw(amount);
    } else {
        invalid_option(amount.to_string());
    }

    consult(account);
}

fn consult(account: &mut Account) {
    show_decoration();
    println!("Balance: {}", account.amount());
    click_enter_to_continue();
}

fn click_enter_to_continue() {
    println!("Click enter to continue...");
    let mut click = String::new();
    io::stdin()
        .read_line(&mut click)
        .expect("Error waiting to continue");
}

fn show_decoration() {
    println!("--------------------------------------------------");
}

fn show_options() {
    show_decoration();
    println!("Select an option:");
    println!("  1. Deposit");
    println!("  2. Withdraw");
    println!("  3. Consult");
    println!("  4. Exit");
}

fn invalid_option(option: String) {
    show_decoration();
    println!("Invalid option: {}", option);
}

fn get_option() -> u8 {
    show_options();
    loop {
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Error reading option");

        let option: u8 = match option.trim().parse() {
            Ok(option) => option,
            Err(option) => {
                invalid_option(option.to_string());
                show_options();
                continue;
            }
        };

        match option {
            1 | 2 | 3 | 4 => {
                //show_decoration();
                break option;
            }
            _ => {
                invalid_option("option out of range".to_string());
                show_options();
                continue;
            }
        }
    }
}
