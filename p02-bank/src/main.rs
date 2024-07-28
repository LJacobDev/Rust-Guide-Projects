#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,   //balance will represent $10.23 as 1023 so integer is used
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

fn main() {

}