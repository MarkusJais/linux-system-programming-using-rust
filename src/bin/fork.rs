extern crate nix;

use nix::unistd::{fork, getpid, getppid};
use nix::unistd::Fork::{Parent, Child};



fn main() {
    let pid = fork();
    match pid {
        Ok(Child) => {
            println!("in child process with pid: {} and parent pid:{}", getpid(), getppid());
        }
        Ok(Parent(child_pid)) => {
            println!("in parent process with pid: {} and child pid:{}", getpid(), child_pid);
        },
        // panic, fork should never fail unless there is a serious problem with the OS
        Err(_) => panic!("Error: Fork Failed")
    }
}
