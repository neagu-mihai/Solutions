use nix::libc;
use nix::sys::signal::{self, SigHandler, Signal};
use std::thread;
use std::time::Duration;

extern "C" fn handle_signal(signal: libc::c_int) {
    let signal = Signal::try_from(signal).unwrap();
    println!("received signal {}: {}", signal, signal.as_str());
}

fn main() {
    let signals = [
        Signal::SIGHUP,
        Signal::SIGINT,
        Signal::SIGQUIT,
        Signal::SIGTRAP,
        Signal::SIGFPE,
    ];
    let handler = SigHandler::Handler(handle_signal);
    for signal in signals {
        unsafe { signal::signal(signal, handler) }.unwrap();
    }
    loop {
        thread::sleep(Duration::from_millis(10));
    }
}
