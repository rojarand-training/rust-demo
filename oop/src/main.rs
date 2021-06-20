trait Shape {
    fn compute_area(&self)->f32;
}

struct Square {
    a_lenght: f32
}

impl Shape for Square {
    fn compute_area(&self)->f32{
        println!("computing area for square!");
        self.a_lenght*self.a_lenght
    }    
}

struct Rect {
    a_lenght: f32,
    b_lenght: f32
}

impl Shape for Rect {
    fn compute_area(&self)->f32{
        println!("computing area for rectangle!");
        self.a_lenght*self.b_lenght
    }    
}

fn main() {
    let square1 = Square{a_lenght: 2.0};
    let rect1 = Rect{a_lenght:3.0, b_lenght:1.5};
    let shapes: 
    //without dyn there is a warning produced: trait objects without an explicit `dyn` are deprecated
        Vec<Box<dyn Shape>> //without explicit declaration that we want to store shape implementations we have compilation error: "expected struct 'Square', found struct 'Rect'"
        = vec!{
            Box::new(square1),//moving square1
            Box::new(rect1)};//moving rect1
    for shape in shapes {
        println!("shape: {}!", shape.compute_area());
    }
    //square1.compute_area(); //square1 is unaccessible because it was moved
}
