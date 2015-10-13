use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Broken! An example to show how it does NOT work correctly.
// Don't use this code!!!!! It will not compile!
fn main() {
    let strings = vec![
        String::from("Rust is awesome"),
        String::from("Rust can make system programming easier"),
        String::from("multi-core programming is hard but Rust can make it easier")
    ];
    let mut threads = vec![];
    let counts: HashMap<String, i32> = HashMap::new();
    let strings = Arc::new(strings);
    let counts = Arc::new(counts);
    for i in 0..3 {
        let strings = strings.clone();
        let counts = counts.clone();
        threads.push(thread::spawn(move || {
            let words: Vec<&str> = strings[i].split_whitespace().collect();
            for word in words {
                let counter = counts.entry(String::from(word)).or_insert(0);  // does not compile!
                *counter += 1;
            }
        }));
    }
    for thread in threads {
        let _ = thread.join();
    }
    println!("counts:\n{:?}", counts);
    println!("\ndone!\n");
}
