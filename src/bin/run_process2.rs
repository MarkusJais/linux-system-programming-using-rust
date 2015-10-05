use std::process::Command;


// running other program as a child
// shorter version than run_process1 avoiding pattern matching
fn main() {
    let mut child_process = Command::new("/bin/catss")
                            .arg("/tmp/file.txt")
                            .spawn()
                            .unwrap_or_else(|error| { panic!("error executing child: {}", error) });

    let exit_status = child_process.wait().
                        unwrap_or_else(|error| { panic!("error waiting on child: {}", error) });

    println!("Exit status:{:?}", exit_status);
}
