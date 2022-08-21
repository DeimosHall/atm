use crate::bank::users::Account;
use crate::string_methods;

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

fn deposit(account: &mut Account) {
    let amount: i32 = string_methods::parse_input_to_i32("amount", "Enter amount to deposit");

    account.set_amount(account.amount() + amount);
    consult(account);
}

fn withdraw(account: &mut Account) {
    let amount: i32 = string_methods::parse_input_to_i32("amount", "Enter amount to withdraw:");

    if amount >= 0 && amount <= account.amount() {
        account.set_amount(account.amount() - amount);
    } else {
        string_methods::invalid_option(amount.to_string());
    }

    consult(account);
}

fn consult(account: &mut Account) {
    string_methods::show_decoration();
    println!("Balance: {}", account.amount());
    string_methods::click_enter_to_continue();
}

fn show_options() {
    string_methods::show_decoration();
    println!("Select an option:");
    println!("  1. Deposit");
    println!("  2. Withdraw");
    println!("  3. Consult");
    println!("  4. Exit");
}

fn get_option() -> u8 {
    show_options();
    loop {
        let option = string_methods::read_line();

        match option.trim().parse::<u8>() {
            Ok(option) => {
                match option {
                    1 | 2 | 3 | 4 => break option,
                    _ => {
                        string_methods::invalid_option("option out of range".to_string());
                        show_options();
                        continue;
                    }
                }
            },
            Err(option) => {
                string_methods::invalid_option(option.to_string());
                show_options();
                continue;
            }
        };
    }
}
