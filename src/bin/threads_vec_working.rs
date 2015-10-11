use std::thread;
use std::sync::Arc;
// Broken! An example to show how it does NOT work correctly.
// Don't use this code!!!!!
fn main() {

    let strings = vec![
        String::from("Rust is awesome"),
        String::from("Rust can make system programming easier"),
        String::from("multi-core programming is hard but Rust can make it easier")
    ];
    let mut threads = vec![];

    let arc_vec = Arc::new(strings);



    for i in 0..3 {
        let strings_clone = arc_vec.clone();
        threads.push(thread::spawn(move || {
            println!("{:?}", strings_clone[i]);
        }));
    }

    for thread in threads {
        let res = thread.join();
    }

    println!("\ndone!\n");
}
