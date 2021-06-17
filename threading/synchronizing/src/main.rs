use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rand::Rng;

fn main() {

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let cnt = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            //Synchronisation starts here, it will be run atomically
            let mut num = cnt.lock().unwrap();
            println!("Thread {}", i);
            let mut rng = rand::thread_rng();
            let duration_in_millis = rng.gen_range(0..100);
            thread::sleep(Duration::from_millis(duration_in_millis));
            println!("Thread {}, sleep duration: {}", i, duration_in_millis);
            *num += 1;
            //Ends here
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
