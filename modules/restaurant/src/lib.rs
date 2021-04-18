mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
    }
    mod serving{
        fn take_order(){}
    }
}

/*
The front_of_house module isn’t public, but because the eat_at_restaurant function is defined in the same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant.
*/

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

/*
Using a semicolon after mod back_of_house rather than using a block tells Rust to load the contents of the module from another file with the same name as the module. 
*/
mod back_of_house;


#[cfg(test)]
mod tests {
	#[test]
	fn demonstrate_use_keyword(){
		use crate::front_of_house::hosting;
		print!("inside foo 1");
		hosting::add_to_waitlist();
	}

    #[test]
    fn demonstrate_using_modules_from_other_files() {
        crate::back_of_house::provisioning::take_delivery();
    }
}
