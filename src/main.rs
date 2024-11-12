
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

    fn find_account_by_id (&self, id:u32) -> Option<&Account> {
        self.accounts.iter().find(|account|account.id==id)
    }
}




fn main() {
let mut bank = Bank::new();
let mut account1 = Account::new(1, String::from("samet"));
let mut account2 = Account::new(2, String::from("ali"));
let mut account3 = Account::new(3, String::from("zeynep"));
let mut account4 = Account::new(4, String::from("emir"));
let mut account5 = Account::new(5, String::from("kartacalı"));


account1.deposit(500);
account1.withdraw(100);

account2.deposit(600);

account3.deposit(700);

account4.deposit(800);
account5.deposit(1000);
account5.withdraw(999);

bank.add_account(account1);
bank.add_account(account2);
bank.add_account(account3);
bank.add_account(account4);
bank.add_account(account5);


let account_id = 3;
if let Some(account) = bank.find_account_by_id(account_id) {
    println !("{} isimli kullanıcının hesap bakiyesi {}", account.name, account.balance)
}

println!("-------------------");



println!("{:#?}",bank);


println!("-------------------");



println!("{:#?}",bank.sum());


println!("-------------------");


println!("The total balance is {:#?}",bank.total_balance());
}
