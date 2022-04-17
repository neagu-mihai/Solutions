use nix::libc::siginfo_t;
use nix::sys::signal::{SaFlags, SigAction, SigHandler, SigSet};
use nix::unistd::SysconfVar::PAGE_SIZE;
use nix::unistd::{getpid, sysconf};
use std::os::raw::{c_int, c_void};

extern "C" fn sigsegv_handler(_signal: c_int, siginfo: *mut siginfo_t, _extra: *mut c_void) {
    let address = unsafe { (*siginfo).si_addr() };
    println!("segmentation fault at {:p}", address);
    // TODO 2: exit the process
}

fn main() {
    let pagesize = sysconf(PAGE_SIZE).unwrap().unwrap();
    println!("PID: {}, pagesize {}", getpid(), pagesize);

    // register a new SIGSEGV handler

    // build a handler (a function that is called when a signal is received)
    let handler = SigHandler::SigAction(sigsegv_handler);

    // build a SigAction structure
    // - use the previously defined handler
    // - no special flags
    // - mask (make them wait) all signals while executing the handler
    // TODO 1: use the SA_RESETHAND flag, what happens?
    let _sigaction_data = SigAction::new(handler, SaFlags::empty(), SigSet::all());

    // register the SigAction with the structure
    // unsafe { sigaction(nix::sys::signal::Signal::SIGSEGV, &sigaction_data) }.unwrap();

    unsafe {
        // define a NULL (0) pointer
        // TODO 3: change the address of the pointer, what happens?
        let p: *mut u8 = 0 as *mut u8;
        // access it (this segfaults)
        *p = 10;
    }
}
