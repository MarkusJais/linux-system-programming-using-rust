use std::thread;

// Broken! An example to show how it does NOT work correctly.
// Don't use this code!!!!! It will not compile!
fn main() {

    let strings = vec![
        String::from("Rust is awesome"),
        String::from("Rust can make system programming easier"),
        String::from("multi-core programming is hard but Rust can make it easier")
    ];
    let mut threads = vec![];

    for i in 0..strings.len() {
        threads.push(thread::spawn(move || {
            println!("{:?}", strings[i]);  // doesn't compile: capture of moved value: `strings`
        }));
    }

    for thread in threads {
        let res = thread.join();
    }
}
