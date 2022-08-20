use crate::bank::users::Account;
use std::io;

pub fn menu(account: &mut Account) {
    loop {
        let option: u8 = get_option();

        match option {
            //1 => deposit(&account),
            2 => withdraw(account),
            3 => consult(account),
            _ => break,
        }
    }
}

fn withdraw(account: &mut Account) {
    let amount: i32 = loop {
        println!("Enter amount to withdraw:");

        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Error reading amount");

        match amount.trim().parse::<i32>() {
            Ok(value) => {
                break value;
            },
            Err(value) => {
                invalid_option(value.to_string());
                continue;
            },
        };
    };

    if amount >= 0 && amount <= account.amount() {
        account.withdraw(amount);
    } else {
        invalid_option(amount.to_string());
    }
}

fn consult(account: &mut Account) {
    println!("Balance: {}", account.amount());
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
                show_decoration();
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
