extern crate nix;

use nix::unistd::{fork, getpid, getppid};
use nix::unistd::Fork::{Parent, Child};
use nix::sys::wait::waitpid;
use nix::sys::wait::WaitStatus::Exited;



fn main() {
    let pid = fork();
    match pid {
        Ok(Child) => {
            println!("in child process. I have pid: {}", getpid());
            println!("my parent process has pid: {}", getppid());
        } // ignore child here

        Ok(Parent(child_pid)) => {

            println!("my child process has pid: {}", child_pid);

            let wait_status = waitpid(child_pid, None);
            match wait_status {
                // assert that waitpid returned correct status and the pid is the one of the child
                Ok(Exited(pid)) =>  {
                    println!("child process with pid {} has successfull exited", pid);
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
