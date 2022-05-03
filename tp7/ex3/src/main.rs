use lazy_static::lazy_static;
use nix::libc::siginfo_t;
use nix::sys::mman::{mmap, mprotect, MapFlags, ProtFlags};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet};
use nix::unistd::SysconfVar::PAGE_SIZE;
use nix::unistd::{getpid, sysconf};
use std::collections::HashMap;
use std::os::raw::{c_int, c_void};
use std::slice;
use std::sync::Mutex;

// TODO 6: define an enum called Protection with two items: None, Read and Write
#[derive(Debug)]
enum Protection {
    None,
    Read,
    Write,
}

// Rust does not allow mutable global variables
// we use this trick to be able to have them
lazy_static! (
    // usage: let page_faults = PAGE_FAULTS.lock().unwrap()
    //   *page_faults = ...
    static ref PAGE_FAULTS: Mutex<usize> = Mutex::new(0);
    // TODO 6: define a global HashMap<usize, Protection> that will store the protection of each page
    static ref PAGES: Mutex<HashMap<usize, Protection>> = Mutex::new(HashMap::new());
);

extern "C" fn sigsegv_handler(_signal: c_int, siginfo: *mut siginfo_t, _extra: *mut c_void) {
    let pagesize = sysconf(PAGE_SIZE).unwrap().unwrap();
    let address = unsafe { (*siginfo).si_addr() };
    println!("segmentation fault at {:p}", address);
    // TODO 3: calculate the page id (index) and page address where the fault happens
    let page_id = address as usize / pagesize as usize;
    let page_address = page_id * pagesize as usize;
    println!("page {}, address {:x}", page_id, page_address);
    // TODO 4: change the page protection so that we can read from it
    // unsafe { mprotect(page_address as *mut c_void, pagesize as usize, ProtFlags::PROT_READ) }.unwrap();
    // TODO 5: count the number of page faults
    let mut page_faults = PAGE_FAULTS.lock().unwrap();
    *page_faults = *page_faults + 1;
    // TODO 6: verify is page is in the has map
    // - if not, this is an actual segfault, exit the process
    // - if yes, increase the protection level None -> Read -> Write
    let mut pages = PAGES.lock().unwrap();
    if let Some(protection) = pages.get(&page_id) {
        println!("page protection is {:?}", *protection);
        match *protection {
            Protection::None => {
                unsafe {
                    mprotect(
                        page_address as *mut c_void,
                        pagesize as usize,
                        ProtFlags::PROT_READ,
                    )
                }
                .unwrap();
                println!("page {:x} protection PROT_READ", page_address);
                pages.insert(page_id, Protection::Read);
            }
            Protection::Read => {
                unsafe {
                    mprotect(
                        page_address as *mut c_void,
                        pagesize as usize,
                        ProtFlags::PROT_WRITE,
                    )
                }
                .unwrap();
                println!("page {:x} protection PROT_WRITE", page_address);
                pages.insert(page_id, Protection::Write);
            }
            _ => {
                unreachable!()
            }
        }
    } else {
        println!("not in any of the mapped pages");
        std::process::exit(-1);
    }
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
    let p = unsafe {
        mmap(
            0 as *mut c_void,
            10 * pagesize as usize,
            ProtFlags::PROT_NONE,
            MapFlags::MAP_ANONYMOUS | MapFlags::MAP_PRIVATE,
            -1,
            0,
        )
    }
    .unwrap();
    println!("mapped 10 pages at {:p}", p);

    // TODO 6: set each page in the hash map as Protection::None
    // use { ... } around your code, this is required for Mutex
    {
        let mut pages = PAGES.lock().unwrap();
        for page_id in 0..10 {
            pages.insert(p as usize / pagesize as usize + page_id, Protection::None);
        }
    }

    // TODO 2: get a slice to the mapped memory and read it (slice.iter())
    // reading means printing the value or let _value = v[i];
    let slice = unsafe { slice::from_raw_parts_mut(p as *mut u8, 10 * pagesize as usize) };
    for value in slice.iter() {
        let _value = *value;
    }

    // TODO 6: write to all the position in the slice (slice.iter_mut())
    for value in slice.iter_mut() {
        *value = 90;
    }

    // TODO 5: print the page faults
    // after printing, use drop(page_faults)
    let page_faults = PAGE_FAULTS.lock().unwrap();

    println!("page faults: {}", page_faults);

    drop(page_faults);

    // TODO 7: read from a memory address that is not mapped (any address from 0 to 1000 should do it)
    unsafe {
        let p: *mut u8 = 0 as *mut u8;
        let _value = *p;
    }
}
