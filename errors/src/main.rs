use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::error::Error;
use std::fmt;

fn error1(){
    let file_name = "hello1.txt";
    let f = File::open(file_name);
    let f = match f {
        Ok(file) => {
            println!("Success opening the file: {}", file_name);
            Some(file)
        }
        Err(error) => {
            println!("Failure opening the file: {}!!!", file_name);
            None
        }
    };
}

fn error2(){
    let file_name = "hello2.txt";
    let f = File::open(file_name).unwrap_or_else(|error|
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error|{
                panic!("Could not create the file: {}!!!", file_name);
            })
        } else {
            panic!("Could not open existing the file: {}!!!", file_name);
        }
    );
    println!("Success opening or creating the file: {}", file_name);
}

fn read_user_name_from_file()->Result<String,io::Error>{
    let f = File::open("user_name.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut n = String::new();
    match f.read_to_string(&mut n) {
        Ok(_) => Ok(n),
        Err(error) => Err(error),
    }
}

fn read_user_name_from_file_with_propagation()->Result<String,io::Error>{
    let mut f = File::open("user_name.txt")?;
    let mut n = String::new();
    f.read_to_string(&mut n)?;
    Ok(n)
}

#[derive(Debug)]
struct MyError1(String);
impl Error for MyError1 {}
impl fmt::Display for MyError1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

fn my_error1()->Result<String, MyError1>{
    Err(MyError1("Ooops".into()))
}

#[derive(Debug)]
struct MyError2(String);
impl Error for MyError2 {}
impl fmt::Display for MyError2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

fn my_error2()->Result<String, MyError2>{
    Err(MyError2("Ooops".into()))
}


fn chain_error()->Result<String, Box<dyn Error>>{
    match my_error1() {
        Ok(text) => Ok(text),
        Err(error) => return Err(Box:new(error));
    }
    match my_error2() {
        Ok(text) => Ok(text),
        Err(error) => return Err(Box:new(error));
    }
    Ok(String::from("Hello"))
    //Ok(my_error1()?);
    //Ok(my_error2()?)
}


fn read_users(){
    let res = read_user_name_from_file();
    match res {
        Ok(name) => {
            println!("read_user_name_from_file - Success reading user name from file: {}", name);
        }
        Err(error) => {
            println!("read_user_name_from_file - Could not read user name from file");
        }
    }

    let res = read_user_name_from_file_with_propagation();
    match res {
        Ok(name) => {
            println!("read_user_name_from_file_with_propagation - Success reading user name from file: {}", name);
        }
        Err(error) => {
            println!("read_user_name_from_file_with_propagation - Could not read user name from file");
        }
    }

}

fn main() {
    error1();
    error2();
    read_users();
    chain_error();
}
