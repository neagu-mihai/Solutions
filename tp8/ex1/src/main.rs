use nix::unistd::getpid;
use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;

fn wait_return() {
    println!("Press ENTER to continue");
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
}

fn main() {
    println!("Run \"watch pmap {}\"", getpid());
    let mut senders = vec![];
    let mut thread_handles = vec![];
    for i in 0..10 {
        // create a channel
        let (sender, receiver) = channel();
        senders.push(sender);
        let t = thread::spawn(move || {
            println!("thread {}", i);
            // wair for a message of type ()
            let mut r:(i32,u32)=receiver.recv().unwrap();
            // TODO 2 - duplicate the number and return it
            //r=r*2;
            //return r;
            // TODO 5 - compute n^p and return it
            let r0:i32=r.0 as i32;
            let r1:u32=r.1 as u32;
            let p:i32= r0.pow(r1);
            return p;
            // quit
            //println!("thread {} done {}", i,r);
        });
        thread_handles.push(t);
        println!("Started {} thread", i + 1);
        println!("Find the page where the thread's stack is allocated");
        wait_return();
    }

    // send a message to all the threads
    for sender in senders {
        // TODO 1 - send a number to a thread
       /*let t=thread::spawn(move || {
        let val = 12;
        sender.send(val).unwrap();
        });*/

        // TODO 4 - send a tuple (n, p) with two numbers
        sender.send((2,3)).unwrap();
       // sender.send(()).unwrap();
        wait_return();
    }
    // wait for all the threads to exit
    let mut sum=0;
    for thread_handle in thread_handles {
        let a=thread_handle.join().unwrap();
        println!("{}",a);
        sum=sum+a;
    }
    println!("sum:{}",sum);

    // TODO 3 - print the sum of all the numbers
}
