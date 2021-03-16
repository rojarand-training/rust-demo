struct Account {
    username: String,
    login: String,
    password: String,
    activated: bool
}

fn main() {
    let mut account1 = Account{
        activated: true,
        username : String::from("Johny"),
        login : String::from("johny"),
        password : String::from("FooBar")
    };
    account1.password = String::from("FizzBazz");

    let account2 = Account{
        activated: false,
        ..account1
    };

    println!("account1.activated: {}", account1.activated);
    println!("account2.activated: {}", account2.activated);
    println!("account2.username: {}", account2.username);

    let account3 = build_new_account(
        String::from("Bob"), 
        String::from("Boby"), 
        String::from("bob"));

    println!("account3.username: {}", account3.username);
    println!("account3.password: {}", account3.password);
    println!("account3.login: {}", account3.login);
    println!("account3.activated: {}", account3.activated);
}

fn build_new_account(username: String, login: String, password: String)->Account{
    return Account{
        username,
        login,
        password,
        activated: true
    };
}
