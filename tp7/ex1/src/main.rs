use std::io::stdin;
use nix::unistd::getpid;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use nix::sys::mman::{mmap, ProtFlags, MapFlags, mprotect, munmap};
use nix::unistd::SysconfVar::PAGE_SIZE;
use std::os::raw::c_void;
use std::{slice, str};

fn main() {
    println!("PID: {}", getpid());
    println!("Press ENTER to continue");
    let mut s=String::new();
    stdin().read_line(&mut s).unwrap();

    let f = File::open("Cargo.toml").unwrap();
    let fd = f.as_raw_fd();
    let p = unsafe { mmap(0 as *mut c_void, PAGE_SIZE as usize, ProtFlags::PROT_NONE, MapFlags::MAP_PRIVATE, fd, 0) }.unwrap();
    println!("Mapped Cargo.toml to {:p} with PROT_NONE", p);

    println!("Press ENTER to continue");
    let mut s=String::new();
    stdin().read_line(&mut s).unwrap();

    unsafe { mprotect(p, PAGE_SIZE as usize, ProtFlags::PROT_READ) }.unwrap();
    println!("Mapped Cargo.toml to {:p} with PROT_READ", p);

    let slice = unsafe { slice::from_raw_parts(p as *const u8, PAGE_SIZE as usize) };
    println!("bytes {:?}", slice);
    println!("text version\n{}", str::from_utf8(slice).unwrap());

    println!("Press ENTER to continue");
    let mut s=String::new();
    stdin().read_line(&mut s).unwrap();

    unsafe { mprotect(p, PAGE_SIZE as usize, ProtFlags::PROT_WRITE) }.unwrap();
    println!("Mapped Cargo.toml to {:p} with PROT_WRITE", p);

    let slice = unsafe { slice::from_raw_parts(p as *const u8, PAGE_SIZE as usize) };
    println!("bytes {:?}", slice);
    println!("text version\n{}", str::from_utf8(slice).unwrap());

    println!("Press ENTER to continue");
    let mut s=String::new();
    stdin().read_line(&mut s).unwrap();

    unsafe { munmap (p, PAGE_SIZE as usize) }.unwrap();
    println!("Un-mapped Cargo.toml");

    println!("Press ENTER to continue");
    let mut s=String::new();
    stdin().read_line(&mut s).unwrap();
}
