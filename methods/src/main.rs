struct Rect{
    width: f32,
    height: f32
}

impl Rect{
    fn area(&self)->f32{
        return self.width*self.height;
    }
}

impl Rect{
    fn square(side: f32)->Rect{
        Rect{
            width: side,
            height: side
        }
    }
}

fn main() {
    let rect = Rect{
        width: 2.5,
        height: 2.25
    };

    println!("Area of a rectangle: {}", rect.area());

    let square = Rect::square(2.0);
    println!("Area of a square: {}", square.area());
}
