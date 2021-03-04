fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    //can not use s1 anymore
    //println!("{}", s1);//err - value borrowed here after move
    println!("s2: {}", s2);

    let i1 = 5;
    let i2 = i1;
    //Note we can use borrowed primitives such as i1. It is kept on stack
    println!("i1: {}, i2: {}", i1, i2);

    let s3 = String::from("World");
    let s4 = s3.clone();
    //Note!!! You can use variable which was cloned not assigned
    println!("s3: {}, s4: {}", s3, s4);


    let s5 = String::from("Foo");
    takes_ownership(s5);
    //println!("s5: {}", s5); -> err, we can not use s5


    let mut s6 = String::from("Bar");
    s6 = takes_and_returns_ownership(s6);
    println!("s6: {}", s6);
}

fn takes_ownership(s: String){
    println!("String inside takes_ownership: {}", s);
}

fn takes_and_returns_ownership(s: String)->String{
    println!("String inside takes_and_returns_ownership: {}", s);
    return s;
}
