use std::io;

fn main() {
    println!("Please enter a number: !");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Expected a user input");
    println!("Your input: {}", number)
}


#[test]
fn it_works() {
	assert_eq!(2 + 2, 4);
}
