use std::thread;
use std::sync::mpsc;
use std::time::Duration;


fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 =  mpsc::Sender::clone(&tx);
    
    let handle1 = thread::spawn(move || {
        let vals = vec![String::from("hi"),
                        String::from("hello"),
                        String::from("Hello"),
                        String::from("loop"),
                        String::from("hi")];
        for val in vals
        {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
 

    let handle2 = thread::spawn(move || {
        let vals = vec![String::from("A"),
                        String::from("B"),
                        String::from("C"),
                        String::from("D"),
                        String::from("E")];
        for val in vals
        {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for rec in rx
    {
        println!("got: {}",rec)
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("Hello, world!");
}
