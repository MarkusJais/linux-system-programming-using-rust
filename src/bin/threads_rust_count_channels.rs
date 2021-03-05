use std::thread;
use std::sync::{Arc};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    let strings = vec![
        String::from("Rust is awesome. Rust is fun."),
        String::from("Rust can make system programming easier. Rust has no GC."),
        String::from("multi-core programming is hard but Rust can make it easier")
    ];
    let mut threads = vec![];
    let strings = Arc::new(strings);
    let (tx, rx): (Sender<usize> , Receiver<usize>) = mpsc::channel();
    for i in 0..3 {
        let strings = strings.clone();
        let tx = tx.clone();
        threads.push(thread::spawn(move || {
            let words: Vec<&str> = strings[i].split_whitespace().collect();
            let count = words.iter().filter(|x| **x == "Rust").count();
            tx.send(count).unwrap();
        }));
    }
    let mut total_count = 0;
    for _ in 0..3 {
        let count = rx.recv().unwrap();
        total_count += count;
    }
    println!("total count for \"Rust\":{:?}", total_count);
    println!("\ndone!\n");
}
