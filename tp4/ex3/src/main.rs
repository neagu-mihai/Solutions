use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, getpid, ForkResult};
use std::ffi::CString;

fn main() {
    let maybe_pid = unsafe { fork() };
    // println!("{:?}", maybe_pid);
    match maybe_pid {
        Ok(ForkResult::Parent { child }) => {
            println!("Waiting command (pid {})", child);
            let status = waitpid(Some(child), None);
            println!("Done {:?}", status);
        }
        Ok(ForkResult::Child) => {
            println!("Running command in process with pid {}", getpid());
            match execvp(
                &CString::new("ls").unwrap(),
                &[CString::new("ls").unwrap(), CString::new("-l").unwrap()],
            ) {
                Ok(_) => {
                    // if exec works, it does not return
                }
                Err(error) => {
                    println!("Exec failed with status {} ({})", error, error as i32);
                }
            }
        }
        Err(error) => {
            println!("Fork error {}", error);
        }
    }
}
