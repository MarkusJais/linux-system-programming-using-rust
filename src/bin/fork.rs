extern crate nix;

use nix::unistd::{getpid, getppid};
use nix::unistd::{fork, ForkResult};

fn main() {
    let pid = unsafe { fork() };
    match pid {
        Ok(ForkResult::Child) => {
            println!("in child process with pid: {} and parent pid:{}", getpid(), getppid());
        }
        Ok(ForkResult::Parent { child, .. }) => {
            println!("in parent process with pid: {} and child pid:{}", getpid(), child);
        }
        // panic, fork should never fail unless there is a serious problem with the OS
        Err(_) => panic!("Error: Fork Failed")
    }
}
