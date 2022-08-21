#[derive(Debug)]
pub struct Account {
    owner: String,
    amount: i32,
}

impl Account {
    pub fn new(owner: String, amount: i32) -> Account {
        Account { owner, amount, }
    }

    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn set_amount(&mut self, amount: i32) {
        self.amount = amount;
    }

    pub fn owner(&self) -> String {
        self.owner.clone()
    }
}