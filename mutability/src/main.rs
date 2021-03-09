fn main() {
    //https://doc.rust-lang.org/book/ch03-02-data-types.html
    let mut mutable_variable = 4;
    mutable_variable = mutable_variable+2;
    let immutable_variable = 4;

    println!("mutable_variable={}, immutable_variable={}", mutable_variable, immutable_variable);
    //immutable_variable = 6;

    //Note!!! char is 4 bytes in size
    let x: char = '😻';
    println!("Char x={}!", x);
}
