use crate::bank::users::Account;
use crate::bank::cashier;

pub mod bank;
pub mod string_methods;

fn main() {
    let owner = String::from("Cayuya");
    let amount = 500;
    let mut account1 = Account::new(owner, amount);

    cashier::menu(&mut account1);
}
