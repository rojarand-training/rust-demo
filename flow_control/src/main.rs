fn main() {
    let mut number = 5;
    if number < 10 {
        println!("number is less than 10");
    } else {
        println!("number is equal or grater than 10");
    }

    let condition = true;
    number = if condition { 4 } else { 2 };
    println!("Number is: {}", number);

    /* Traditional ternanry operator is not valid in rust
    number = condition ? 1 : 3;
    println!("Number is: {}", number);
    */

    number = 0;
    let counter = loop {
        number = number+1;
        if (number%7) == 0{
            break number;// NOTE number after the break
        }
    };// <----- NOTE semicolon here
    println!("After the 'loop' counter is: {}", counter);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    /*
    3!
    2!
    1!
    */

}
