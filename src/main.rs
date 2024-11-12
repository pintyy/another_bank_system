use std::fmt::format;

#[derive(Debug)]
struct Account {
    id : u32,
    balance:i32,
    name:String
}

impl Account {
    fn new(id:u32,name:String)->Self {
        Account {

            id,
            name,
            balance:0

        }
    }
    fn deposit(&mut self, amount:i32)->i32 {
        self.balance +=amount;
        self.balance
        
    }

    fn withdraw (&mut self, amount:i32) ->i32 {

        self.balance-=amount;
        self.balance
    }

    fn sum (&self)-> String {

        format!("{} has a balance {} ",self.name, self.balance)

    }
}
#[derive(Debug)]

struct  Bank {
    accounts:Vec<Account>

}

impl  Bank {
    fn new () -> Self {
        Bank {accounts : vec![]}
    }
    fn add_account (&mut self , account:Account) {
        self.accounts.push(account);

    }

    fn total_balance (&self) -> i32 {
        self.accounts.iter().map(|account|account.balance).sum()
    }
    fn sum (&self) -> Vec<String> {
        self.accounts.
        iter().
        map(|account|account.sum()).
        collect::<Vec<String>>()
    } 
}




fn main() {
let mut bank = Bank::new();
let mut account = Account::new(1, String::from("samet"));

account.deposit(500);
account.withdraw(00);
bank.add_account(account);

println!("{:#?}",bank);

println!("{:#?}", bank.sum());
println!("The total balance is {:#?}",bank.total_balance())
}
