use std::ops::Deref;
use std::ops::DerefMut;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T)->MyBox<T>{
        MyBox(t) 
    }
}

impl<T> Deref for MyBox<T> {
    /*
    Note!!! Without `type Target = T` there is a compilation error saying:
    "not all trait items implemented, missing: Target"
    */
    type Target = T;
    fn deref(&self) -> &T {
        println!("defering !!!");
        &self.0
    }    
}

impl<T> DerefMut for MyBox<T> {
    /*
    Note!!! Without `type Target = T` there is a compilation error saying:
    "not all trait items implemented, missing: Target"
    */
    //type Target = T;
    fn deref_mut(&mut self) -> &mut T {
        println!("defering mut !!!");
        &mut self.0
    }    
}


fn main() {

    let x = 1;
    let mut y = MyBox::new(x);
    println!("Boxed value: {}", *y);
    println!("Will dereference boxed value with '*'");
    assert_eq!(x, *y);
    println!("Did dereference boxed value with '*'");
    println!("Will dereference mutable value value");
    let dy = &mut *y;
    *dy = 2;
    println!("Did dereference mutable value value");
    println!("Boxed value: {}", *y);
}
