struct Square{
    side_length: u32
}

impl Square {

    fn can_hold(&self, other_square: &Square)->bool{
        return self.side_length >= other_square.side_length
    }
}

pub fn greeting(name: &str)->String{
    return format!("Hello {}", name)
}

struct Value0_100 {
    value: i8
}

impl Value0_100 {
    fn new(value: i8)->Value0_100{
        let in_range = 0<=value && value<=100;
        if !in_range {
            panic!("Given value should be from range 0-100, given: {}", value)
        }
        Value0_100{value:value}
    }
}

#[cfg(test)]
mod tests {
    //note we use 'use super::*' here to allow visibility of Square in tests module
    use super::*;
    #[test]
    fn larger_square_can_hold_smaller() {
        let larger = Square {
            side_length : 5
        };
        let smaller = Square{side_length:1};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting should format result with given name, actual: {}", result);
    }

    #[test]
    #[should_panic]
    fn expects_panicking_when_value_out_of_range(){
        Value0_100::new(120);
    }
}
