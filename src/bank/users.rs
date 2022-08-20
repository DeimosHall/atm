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

    pub fn owner(&self) -> String {
        self.owner.clone()
    }

    pub fn deposit(&mut self, amount: i32) {
        self.amount += amount;
    }

    pub fn withdraw(&mut self, amount: i32) {
        self.amount -= amount;
    }
}