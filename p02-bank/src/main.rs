#[derive(Debug)]
struct Account {
    id: u32,
    balance: isize, //balance will represent $10.23 as 1023 so integer is used
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

    ///Increase account balance by specified deposit size if deposit >= 0.  Return new account balance.
    fn credit_funds(&mut self, deposit: isize) -> isize {
        if deposit >= 0 {
            println!("Depositing {}", deposit);
            self.balance += deposit;
            println!("Deposit complete, new balance: {}", self.balance);
        }
        self.balance
    }

    ///Decrease account balance by specified withdrawal size if withdrawal >= 0 and withdrawal < account balance.  Return new account balance.
    fn debit_funds(&mut self, withdrawal: isize) -> isize {
        if withdrawal >= 0 && withdrawal < self.balance {
            println!("Withdrawing {}", withdrawal);
            self.balance -= withdrawal;
            println!("Withdrawal complete, new balance: {}", self.balance);
        }
        if withdrawal > self.balance {
            println!("Insufficient funds");
        }
        self.balance
    }

    ///Returns formatted string of account fields and values.
    fn account_summary(&self) -> String {
        format!("{:?}", self)
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

fn main() {
    let bank = Bank::new();

    //putting in a string like "" is seen as &str, or a string slice,
    //so to get a true String type you need to use String::from(), or format!()
    let mut account = Account::new(1, String::from("TestName"));

    println!("{}", account.account_summary());

    account.credit_funds(10);

    account.debit_funds(5);

    account.debit_funds(15);

    println!("{}", account.account_summary());

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
