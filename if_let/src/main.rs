enum Option{
    First,
    Second,
    Third
}

fn main() {
    let option = Option::Second;
    match option {
        Option::Second => { println!("1. Second option selected") }
        _ => { println!("Not second option selected")  } 
    }
    if let Option::Second = option {
        println!("2. Second option selected")
    }
    /*
    //Will not compile due to: note: an implementation of `std::cmp::PartialEq` might be missing for `Option`
    if Option::Second == option {
        println!("3. Second option selected")
    }*/
    
    if let Option::First = option {
        println!("First option selected")
    }

    if let Option::Third = option {
        println!("Third option selected")
    }
}
