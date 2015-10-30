extern crate nix;

use nix::unistd::{fork, getpid, execvp};
use nix::unistd::Fork::{Parent, Child};
use nix::sys::wait::waitpid;
use nix::sys::wait::WaitStatus;
use std::ffi::CString;



fn main() {
    let pid = fork();
    match pid {
        Ok(Child) => {
            let command =  &CString::new("/bin/cat").unwrap();
            let arguments =
                    &[
                    CString::new(b"/bin/cat".as_ref()).unwrap(),
                    CString::new(b"/tmp/file.txt".as_ref()).unwrap(),
                    CString::new(b"-n".as_ref()).unwrap(),
                    ];
            execvp(command, arguments).unwrap();
        }
        Ok(Parent(child_pid)) => {

            println!("in parent process with pid: {} and child pid:{}", getpid(), child_pid);

            let wait_status = waitpid(child_pid, None);
            match wait_status {
                // assert that waitpid returned correct status and the pid is the one of the child
                Ok(WaitStatus::Exited(pid, status)) =>  {
                    println!("child process with pid {} exited with status: {}", pid, status);
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
