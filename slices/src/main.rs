fn main() {
    demostrate_unsafe();
    demostrate_safe();
}

fn demostrate_unsafe(){
    let mut s = String::from("Hello, world!");
    let res1 = find_first_word_usize(&s);
    println!("Unsafe res={}", res1);
    s.clear();
    //Note: res1 contains 6 what is invalid position of end of first word
}

fn find_first_word_usize(s: &String)->usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn demostrate_safe(){
    let mut s = String::from("Hello, world!");
    let res1 = find_first_word_slice(&s);
    //s.clear();//NOTE: we can not change string, res1 is still valid in that range
    //^^^^^^^^^ mutable borrow occurs here
    println!("Safe res={}", res1);
    s.clear();//NOTE: we can change string here, res1 is out of range
}

fn find_first_word_slice(s: &String)-> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

