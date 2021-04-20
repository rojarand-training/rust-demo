enum SpreadsheetCell{
    Text(String),
    Integer(i32),
    Float(f32)
}

fn print_vector(vec: &Vec<i32>){
    for (i,v) in vec.iter().enumerate() {
        println!("[{}]: {}", i, v);
    }
}

fn main() {
    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    for i in &mut vec1 {
        *i = *i * *i;
    }
    print_vector(&vec1);
    let v0 = vec1[0];

    let vec2 = vec![2,4,6];
    for i in vec2 {
        println!("val in vec2: {}", i);
    }

    let row = vec![
        SpreadsheetCell::Integer(10)//,
        //SpreadsheetCell::Text(String::form("Text")),
        //SpreadsheetCell::Float(10.10)
    ];
    //let vec3 = vec![];
}
