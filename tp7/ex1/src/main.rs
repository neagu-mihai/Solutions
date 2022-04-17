use nix::sys::mman::{mmap, mprotect, munmap, MapFlags, ProtFlags};
use nix::unistd::SysconfVar::PAGE_SIZE;
use nix::unistd::{getpid, sysconf};
use std::fs::File;
use std::io::stdin;
use std::os::raw::c_void;
use std::os::unix::io::AsRawFd;
use std::{slice, str};

fn wait_return() {
    println!("Press ENTER to continue");
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
}

fn main() {
    // different OS implementation might have different page size
    // to make our software parotable, we ask the system for
    // the page size that it uses
    let pagesize = sysconf(PAGE_SIZE).unwrap().unwrap();
    println!("PID: {}", getpid());

    // pmap prints the process memory layout
    println!(
        "Use a new terminal and run \"watch pmap {}\" to see the memory mappings",
        getpid()
    );
    println!("============================================");
    println!("Take a look at all the memory mappings, try to understand what each of the means:");
    println!("  1. what is ex1?");
    println!("  2. what is libc-nnn.so");
    println!("  3. what is ld-nnn.so?");
    println!("  4. what is [ stack ]");
    println!("  5. what is [ anon ]");
    println!("  6. How much memory does this occupy?");

    wait_return();

    let f = File::open("Cargo.toml").unwrap();
    let fd = f.as_raw_fd();
    // map the first page of the file with no access
    let p = unsafe {
        mmap(
            0 as *mut c_void,
            pagesize as usize,
            ProtFlags::PROT_NONE,
            MapFlags::MAP_PRIVATE,
            fd,
            0,
        )
    }
    .unwrap();
    println!("============================================");
    println!("Mapped 1 page from Cargo.toml");
    println!(" 1. How large is a memory page?");
    println!(" 2. What is the address where Cargo.toml was mapped at?");
    println!(" 3. What are the access rights?");
    wait_return();
    println!(
        "Answer: the page is 4KB mapped at address {:p} with PROT_NONE. Was this your answer?",
        p
    );
    wait_return();

    // change the mapped memory protection so that we can read from it
    unsafe { mprotect(p, pagesize as usize, ProtFlags::PROT_READ) }.unwrap();
    println!("============================================");
    println!(
        "We modified the protections for the mapping of Cargo.toml. What are the new protections?"
    );
    wait_return();
    println!("Answer: PROT_READ. Was this your answer?");

    // get a safe slice to the memory
    let slice = unsafe { slice::from_raw_parts(p as *const u8, pagesize as usize) };
    println!(
        "The mapped memory reading:\n{}",
        str::from_utf8(slice).unwrap()
    );
    wait_return();

    // change the mapped memory protection so that we can write to it
    unsafe { mprotect(p, pagesize as usize, ProtFlags::PROT_WRITE) }.unwrap();
    println!("============================================");
    println!(
        "We modified the protections for the mapping of Cargo.toml. What are the new protections?"
    );
    wait_return();
    println!("Answer: PROT_WRITE. Was this your answer?");
    wait_return();

    println!("============================================");
    println!("We could read from the mapped memory even tough we used PROT_WRITE. Why?");
    let slice = unsafe { slice::from_raw_parts(p as *const u8, pagesize as usize) };
    println!(
        "The mapped memory reading:\n{}",
        str::from_utf8(slice).unwrap()
    );
    wait_return();

    // unmap the file from the memory
    unsafe { munmap(p, pagesize as usize) }.unwrap();
    println!("============================================");
    println!("Un-mapped Cargo.toml");
    println!("Take a look to make sure it is not mapped anymore");
    wait_return();

    // we map some empty memory, meaning we ask for empty memory
    let p = unsafe {
        mmap(
            0 as *mut c_void,
            5 * pagesize as usize,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_ANONYMOUS | MapFlags::MAP_PRIVATE,
            -1,
            0,
        )
    }
    .unwrap();

    println!("============================================");
    println!("We mapped some empty memory at address {:p}:", p);
    println!(" 1. How many pages did map?");
    println!(" 1. What is the protoction level that we have required?");
    wait_return();

    // TODO ex2: verify that the mapped memory consists only out of 0 value bytes
    // a) use p.read_volatile() and p.offset()
    // hint: tyepcast p as *mut u8
    // b) use a slice
    // hint: tyepcast p as *mut u8

    println!("Answer: 5 pages, 20 KB, PROT_READ and PROT_WRITE");
    wait_return();

    unsafe { munmap(p, 5 * pagesize as usize) }.unwrap();
    println!("============================================");
    println!("Un-mapped memory");
    println!("Take a look to make sure it is not mapped anymore");
    wait_return();

    // TODO ex2: try to access the unmapped memory. What happens?
}
