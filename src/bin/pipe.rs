extern crate nix;

use nix::unistd::{fork, read, write, pipe};
use nix::unistd::Fork::{Parent, Child};
use std::str;

fn main() {
    let (reader, writer) = pipe().unwrap();
    let msg = "hello from parent";
    let pid = fork();
    match pid {
        Ok(Child)=> {
            let mut read_buf = [0u8; 32];
            let bytes_read = read(reader, &mut read_buf).unwrap();
            let msg_received = str::from_utf8(&read_buf[0 .. bytes_read]).unwrap();
            println!("received from partent:{:?}", msg_received);

        }
        Ok(Parent(child_pid)) => {
            println!("sending to child with pid:{}", child_pid);
            write(writer, msg.as_bytes()).unwrap();
        },
        Err(_) => panic!("Error: Fork Failed")
    }
}
