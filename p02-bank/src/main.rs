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
    


    /*

        The following code is being left as a reminder comment

        This was the main point of this project, to demonstrate
        that Rust has ownership, borrowing, lifetime features
        Most of the videos were about showing examples of it and
        doing exercises to practice understanding of it

        before watching the final videos where he shows implementing the functions,
        I wanted to try doing them myself first in a branch and then depending on
        how close my version is to what he shows, I'll either merge the branch over
        or else leave it as an unmerged branch and go on to make the main branch
        follow the guide's examples

        //printing the account once, no error
        print_account(account);
        //printing the account a second time, an error occurs
        //it has something to do with the ownership / borrowing feature in rust
        print_account(account);
        /*
            error information:
            use of moved value: `account`
            value used here after moverustc Click for full compiler diagnostic
            main.rs(43, 19): value moved here
            main.rs(38, 9): move occurs because `account` has type `Account`, which does not implement the `Copy` trait
            main.rs(29, 27): consider changing this parameter type in function `print_account` to borrow instead if owning the value isn't necessary
            main.rs(2, 1): if `Account` implemented `Clone`, you could clone the valu
        */
    */
}
