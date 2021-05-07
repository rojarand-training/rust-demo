fn largest<T: std::cmp::PartialOrd+Copy>(arg_list: &[T])->T{
    let mut l = arg_list[0];
    for item in arg_list{
        if item > &l{
            l = *item;
        }
    }
    return l;
}

fn main() {
    let i32_list = vec![1,5,7,3,9,2];
    let result = largest(&i32_list);
    println!("The largest in the int list: {}", result);

    let f32_list = vec![5.0,72.6,3.0];
    let result = largest(&f32_list);
    println!("The largest in the float list: {}", result);
}
