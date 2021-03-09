fn main() {
    let mut s = String::from("Hello ");
    change1(&s);
    change2(&mut s);

    //Mutable references have one big restriction: 
    //You can have only one mutable reference to a particular piece of data in a particular scope. 
    //This code will fail:
    /*
    let r1 = &mut s;
    let r2 = &mut s;
    println!("Hello r1:{}. r2:{}", r1, r2);
    */
    multiple_references_ok();
    multiple_references_ko();
}

fn change1(s: &String){
    println!("You are inside change1: {}", s);
    //s.push_str("World"); //---> variable 's' is immutable
}

fn change2(s: &mut String){
    s.push_str("World"); //---> variable 's' is mutable
    println!("You are inside change2: {}", s);
}

fn multiple_references_ok(){
    let mut s = String::from("Hi");
    let s1 = &s;
    let s2 = &s;
    println!("inside 'multiple_references_ok' - s1: {}, s2: {}", s1, s2);//Note! s1 and s2 are no longer used, we can declare mutable ref.
    let s3 = &mut s;
    s3.push_str(" there");
    println!("inside 'multiple_references_ok' - s3: {}", s3);
}

fn multiple_references_ko(){
    let mut s = String::from("Hi");
    let s1 = &s;
    let s2 = &s;
    //let s3 = &mut s;//Note! We declare mutable ref where two immutable are still used. We can
    //not do it. The compiler will complain.
    println!("inside 'multiple_references_ok' - s1: {}, s2: {}", s1, s2);
    s.push_str(" there");
    println!("inside 'multiple_references_ok' - s: {}", s);
}
