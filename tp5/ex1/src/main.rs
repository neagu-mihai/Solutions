use nix::unistd::{close, fork, pipe, write, ForkResult};
use std::fs;
use std::io::{stdin, stdout, Read, Write};
use std::os::unix::io::FromRawFd;

fn main() {
    let (fdw0, fdw1) = pipe().unwrap();
    let (fdr0, fdr1) = pipe().unwrap();

    let pid = unsafe { fork() };
    match pid {
        Ok(pid) => match pid {
            ForkResult::Parent { child } => {
                close(fdw0).unwrap();
                close(fdr1).unwrap();
                let mut s = String::new();
                stdin().read_line(&mut s).unwrap();
                let mut f = unsafe { fs::File::from_raw_fd(fdw1) };
                f.write(s.as_bytes()).unwrap();
                close(fdw1).unwrap();

                let mut f = unsafe { fs::File::from_raw_fd(fdr0) };
                s.clear();
                f.read_to_string(&mut s).unwrap();
                println!("got back: {}", s);
            }
            ForkResult::Child => {
                close(fdw1).unwrap();
                close(fdr0).unwrap();
                let mut f = unsafe { fs::File::from_raw_fd(fdw0) };
                let mut s = String::new();
                f.read_to_string(&mut s).unwrap();
                close(fdw0).unwrap();
                println!("got: {}", s);
                s = s.chars().rev().collect();

                let mut f = unsafe { fs::File::from_raw_fd(fdr1) };
                f.write(s.as_bytes());
            }
        },
        Err(error) => eprintln!("For failed {}", error),
    }
}
