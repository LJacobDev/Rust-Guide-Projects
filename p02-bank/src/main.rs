#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32, //balance will represent $10.23 as 1023 so integer is used
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: Account){
    println!("Account: {account:#?}");
}

fn main() {
    let bank = Bank::new();

    //putting in a string like "" is seen as &str, or a string slice,
    //so to get a true String type you need to use String::from(), or format!()
    let account = Account::new(1, String::from("TestName"));

    println!("Bank {bank:#?}");
    
    //printing the account once, no error
    print_account(account);
    //printing the account a second time, an error occurs
    //it has something to do with the ownership / borrowing feature in rust and this is not seen in other programming languages usually
    print_account(account);
    /*
        error information:
        use of moved value: `account`
        value used here after moverustcClick for full compiler diagnostic
        main.rs(43, 19): value moved here
        main.rs(38, 9): move occurs because `account` has type `Account`, which does not implement the `Copy` trait
        main.rs(29, 27): consider changing this parameter type in function `print_account` to borrow instead if owning the value isn't necessary
        main.rs(2, 1): if `Account` implemented `Clone`, you could clone the valu
    */
}
