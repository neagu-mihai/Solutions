use nix::libc;
use nix::sys::signal::{self, SaFlags, SigAction, SigHandler, SigSet, Signal};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::{fork, ForkResult};
use std::thread;
use std::time::Duration;

extern "C" fn handle_signal(signal: libc::c_int) {
    let signal = Signal::try_from(signal).unwrap();
    thread::sleep(Duration::from_millis(100));

    while let Ok(status) = waitpid(None, Some(WaitPidFlag::WNOHANG)) {
        println!("child {:?} exited", status);
    }
    println!("signal handled");
}

fn main() {
    let handler = SigHandler::Handler(handle_signal);
    let mut mask = SigSet::empty();
    mask.add(Signal::SIGCHLD);
    let action = SigAction::new(handler, SaFlags::empty(), mask);
    unsafe { signal::sigaction(Signal::SIGCHLD, &action) }.unwrap();

    let mut processes = 10;

    while processes > 0 {
        processes -= 1;
        let pid = unsafe { fork() };
        match pid {
            Ok(pid) => match pid {
                ForkResult::Parent { child } => {}
                ForkResult::Child => {
                    std::process::exit(0);
                }
            },
            Err(error) => eprintln!("For failed {}", error),
        }
    }

    loop {
        thread::sleep(Duration::from_millis(10));
    }
}
