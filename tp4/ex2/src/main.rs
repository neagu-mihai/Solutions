use nix::unistd::{fork, getpid, getppid, ForkResult};
use std::thread;
use std::time::Duration;

fn main() {
    let maybe_pid = unsafe { fork() };
    // println!("{:?}", maybe_pid);
    match maybe_pid {
        Ok(ForkResult::Parent { child }) => {
            println!("Parent ({}), child pid is {}", getpid(), child);
        }
        Ok(ForkResult::Child) => {
            println!("Child ({}), parent pid is {}", getpid(), getppid());
            thread::sleep(Duration::from_millis(1000));
            println!("Child ({}), parent pid is {}", getpid(), getppid());
        }
        Err(error) => {
            println!("Fork error {}", error);
        }
    }
}
