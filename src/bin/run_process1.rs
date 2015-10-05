use std::process::Command;


// running other program as a child
// this is a verbose solution with pattern matching.
// see run_process2 for a shorter version
fn main() {
    let child_process = Command::new("/bin/cat")
                            .arg("/tmp/file.txt")
                            .spawn();

    match child_process {
        Ok(mut child) => {
            match child.wait() {
                Ok(status) => {
                    println!("child exited with status:{:?}", status);
                }

                Err(err) => {
                    println!("error waiting for child: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("error spawning child: {:?}", err);
        }
    }
}
