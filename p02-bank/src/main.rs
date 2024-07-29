/*
    Objective:  Simulate a Bank object that can contain multiple Account objects, which can have balances.
                These objects will also have functionality to add accounts, add funds, withdraw funds, summarize status.

                The intention of the project is to demonstrate Rust's ownership, borrowing, and lifetime features,
                and to understand how data moves around inside of a Rust application differently than in other languages
*/

#[derive(Debug)]
struct Account {
    id: u32,
    balance: isize, //balance will represent $10.23 as 1023 so integer is used
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        println!("New account created with ID: {} for Holder: {}", id, holder);
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
        } else if withdrawal > self.balance {
            println!("Insufficient funds");
        }
        self.balance
    }

    ///Returns formatted string of account fields and values.
    fn account_summary(&self) -> String {
        //my version just returned the entire object in a debug formatted string, but the guide used a better way as shown below
        // format!("{:?}", self)
        format!("{} has a balance of {}", self.holder, self.balance)
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

    ///Takes a given Account instance and adds it to the Bank's accounts vector.
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    ///Returns the sum of all the accounts in the Bank
    fn sum_accounts(&self) -> isize {
        //I initially did this for loop, but the guide demonstrates using an iterator as shown below this
        /*
        let mut sum = 0;

        for account in &self.accounts {
            sum += account.balance;
        }

        sum
        */

        //this is a new thing showed in the guide which will be explained more in coming projects
        //I think what it looks like it is doing is using map to return a collection that contains
        //references to just the account balances and then calls sum on that collection of balance references
        self.accounts.iter().map(|account| account.balance).sum()
    }

    ///Returns a Vector<String> containing the summaries of each Account contained in the Bank.
    fn accounts_summary(&self) -> Vec<String> {
        //similarly to above, I used a for loop to accomplish this,
        //but the guide shows that there is an iterator that can do this instead as seen below
        /*
        let mut summary = vec![];

        for account in &self.accounts {
            summary.push(account.account_summary());
        }

        summary
        */

        //this is what the guide showed to do to make it work using iterator style
        //though the .collect::<Vec<String>>() is a new kind of syntax that I haven't seen elsewhere yet
        //the map method gives a collection of the account summary strings, and
        //then those are collected into a Vector of Strings, using a syntax of '.collect::<Vec<String>>()'
        //the documentation popup boxes explain this, that .collect transforms an iterator into a relevant collection
        self.accounts
            .iter()
            .map(|account| account.account_summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();

    //putting in a string like "" is seen as &str, or a string slice,
    //so to get a true String type you need to use String::from(), or format!()
    let mut account = Account::new(1, String::from("TestName"));

    println!("{}", account.account_summary());

    account.credit_funds(10);
    account.debit_funds(5);
    account.debit_funds(15);

    println!("{}", account.account_summary());

    bank.add_account(account);

    //account binding is empty now, fill it with another new Account instance
    account = Account::new(2, String::from("Jeffy"));
    account.credit_funds(100);
    bank.add_account(account);

    bank.accounts[1].credit_funds(15);

    let bank_sum = bank.sum_accounts();
    println!("Sum of Bank's accounts: {}", bank_sum);

    //It is interesting to see that the bank.accounts_summary() output to the console shows the holder name with an escape character.
    //The direct account.account_summary() output doesn't print as having escape characters, but it also doesn't have
    //double quotes wrapped around its output.
    //But when it appears in the bank.accounts_summary() as a vector of strings, there are double quotes
    //wrapped around each element in the output, and then there are escape characters showing in the inner quotes
    //around the holder name that were not intentionally placed there but were added automatically

    //UPDATE:  when I changed the account_summary format!() line to print out a formatted string of specific values and text,
    //rather than just the account struct, it caused the issue mentioned above to go away
    println!("Bank details:\n {:#?}", bank.accounts_summary());

    /*

        The following code is being left as a reminder comment

        This was the main point of this project, to demonstrate
        that Rust has ownership, borrowing, lifetime features
        Most of the videos were about showing examples of it and
        doing exercises to practice understanding of it

        before watching the final videos where he shows implementing the functions,
        I wanted to try doing them myself first before viewing and following the guide's examples

        //printing the account once, no error
        print_account(account);
        //printing the account a second time, an error occurs
        //it has something to do with the ownership / borrowing feature in rust
        print_account(account);
        /*
            error information:
            use of moved value: `account`
            value used here after move rustc Click for full compiler diagnostic
            main.rs(43, 19): value moved here
            main.rs(38, 9): move occurs because `account` has type `Account`, which does not implement the `Copy` trait
            main.rs(29, 27): consider changing this parameter type in function `print_account` to borrow instead if owning the value isn't necessary
            main.rs(2, 1): if `Account` implemented `Clone`, you could clone the valu
        */
    */
}
