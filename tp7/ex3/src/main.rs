use lazy_static::lazy_static;
use nix::libc::siginfo_t;
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet};
use nix::unistd::SysconfVar::PAGE_SIZE;
use nix::unistd::{getpid, sysconf};
use std::os::raw::{c_int, c_void};
use std::sync::Mutex;

// TODO 6: define an enum called Protection with two items: None, Read and Write

// Rust does not allow mutable global variables
// we use this trick to be able to have them
lazy_static! (
    // usage: let page_faults = PAGE_FAULTS.lock().unwrap()
    //   *page_faults = ...
    static ref PAGE_FAULTS: Mutex<usize> = Mutex::new(0);
    // TODO 6: define a global HashMap<usize, Protection> that will store the protection of each page
);

extern "C" fn sigsegv_handler(_signal: c_int, siginfo: *mut siginfo_t, _extra: *mut c_void) {
    let address = unsafe { (*siginfo).si_addr() };
    println!("segmentation fault at {:p}", address);
    // TODO 3: calculate the page id (index) and page address where the fault happens

    // TODO 4: change the page protection so that we can read from it

    // TODO 5: count the number of page faults

    // TODO 6: verify if the page is in the hashmap
    // - if not, this is an actual segfault, exit the process
    // - if yes, increase the protection level None -> Read -> Write
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
    let sigaction_data = SigAction::new(handler, SaFlags::empty(), SigSet::all());

    // register the SigAction with the structure
    unsafe { sigaction(nix::sys::signal::Signal::SIGSEGV, &sigaction_data) }.unwrap();

    // TODO 1: map empty 10 pages with PROT_NONE

    // TODO 6: set each page in the hash map as Protection::None
    // use { ... } around your code, this is required for Mutex

    // TODO 2: get a slice to the mapped memory and read it (slice.iter())
    // reading means printing the value or let _value = v[i];

    // TODO 6: write to all the position in the slice (slice.iter_mut())

    // TODO 5: print the page faults
    // after printing, use drop(page_faults)

    // TODO 7: read from a memory address that is not mapped (any address from 0 to 1000 should do it)
}
