use nix::sys::wait::waitpid;
use nix::unistd::{dup2, execvp, fork, getpid, ForkResult};
use std::ffi::CString;
use std::fs;
use std::os::unix::io::AsRawFd;

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
            println!("Creating output file");
            let file = fs::File::create("output.txt").expect("Error");
            // append
            // let file = fs::OpenOptions::new()
            //     .write(true)
            //     .create(true)
            //     .append(true)
            //     .open("output.txt")
            //     .expect("Error");
            let fd = file.as_raw_fd();
            match dup2(fd, 1) {
                Ok(_) => {
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
                    println!("Failed to redirect due to {}", error);
                }
            }
            println!("Running command in process with pid {}", getpid());
        }
        Err(error) => {
            println!("Fork error {}", error);
        }
    }
}
