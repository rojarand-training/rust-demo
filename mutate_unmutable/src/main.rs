use std::cell::RefCell;

struct Counter{
    counter: i32
}

impl Counter{
    fn inc(&mut self){
        self.counter = self.counter+1;
    }
}

struct Square{
    side_length: f32,
    counter_ref_cell: RefCell<Counter>,
}

impl Square {
    fn new(side_length: f32)->Square{
        return Square{side_length, 
            counter_ref_cell: RefCell::new(Counter{counter:0})};
    }

    fn compute_area(&self)->f32{
        //Note here we are changing unmutable self here
        self.counter_ref_cell.borrow_mut().inc();
        return self.side_length*self.side_length;
    }

    fn compute_area2(&mut self)->f32{
        //Note here we are changing inner state of unmutable self here
        self.counter_ref_cell.borrow_mut().inc();
        return self.side_length*self.side_length;
    }

    fn get_number_of_computations(&self)->i32{
        return self.counter_ref_cell.borrow().counter;
    }
}

fn main() {
    let square = Square::new(2.0);
    println!("1. area of a square: {}", square.compute_area());
    println!("2. area of a square: {}", square.compute_area());
    println!("Number of computations: {}", square.get_number_of_computations());
}
