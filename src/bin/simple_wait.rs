extern crate nix;

use nix::unistd::{fork, ForkResult};
use nix::sys::wait::wait;
use nix::sys::wait::WaitStatus; 



fn main() {
    let pid = unsafe { fork() };
    match pid {
        Ok(ForkResult::Child) => {
            // ignore child
        }
        Ok(ForkResult::Parent{ child: _, .. }) => {
            let wait_status = wait();
            match wait_status {
                 Ok(WaitStatus::Exited(pid, status)) =>  {
                    println!("child process with pid {} has successfully exited with status: {}", pid, status);
                },
                // panic, must never happen
                Ok(_) => panic!("Child still alive, should never happen"),

                // panic, waitpid should never fail
                Err(_) => panic!("Error: waitpid Failed")
            }
        },
        // panic, fork should never fail unless there is a serious problem with the OS
        Err(_) => panic!("Error: Fork Failed")
    }
}
