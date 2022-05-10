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
            receiver.recv().unwrap();
            // TODO 2 - duplicate the number and return it
            // TODO 5 - compute n^p and return it
            // quit
            println!("thread {} done", i);
        });
        thread_handles.push(t);
        println!("Started {} thread", i + 1);
        println!("Find the page where the thread's stack is allocated");
        wait_return();
    }

    // send a message to all the threads
    for sender in senders {
        // TODO 1 - send a number to a thread
        // TODO 4 - send a tuple (n, p) with two numbers
        sender.send(()).unwrap();
        wait_return();
    }

    // wait for all the threads to exit
    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }

    // TODO 3 - print the sum of all the numbers
}
