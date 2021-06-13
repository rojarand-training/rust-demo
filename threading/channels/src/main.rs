use std::thread;
use std::sync::mpsc;
use std::time::Duration;


fn send_one(){

    let (tx, rx) = mpsc::channel();
    thread::spawn(move ||{
        let msg = String::from("hi");
        tx.send(msg).unwrap(); 
        //println!("Did post msg: {}", msg); !!! Will not compile beacuse msg was 'moved' by send
    });
    let received = rx.recv().unwrap();
    println!("received: {}!", received);
}

fn send_many(){

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let strings = vec![
            String::from("Hello"),
            String::from("we"),
            String::from("are"),
            String::from("demonstrating"),
            String::from("channels"),
        ];
        for str in strings {
            tx.send(str).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}


fn main() {
    send_one();
    send_many();
}
