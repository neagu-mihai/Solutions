use std::thread;
use std::time::Duration;

fn main() {
    let t=thread::spawn(|| {
        for i in 0..10 {
            // TODO 1 - print the thread id and sleep a few seconds
            println!("thread:{}",i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 0..10 {
        println!("main {}", i);
        thread::sleep(Duration::from_millis(500));
    }
    t.join().unwrap();
    // TODO 2 - wait for the thread to finish
    // (hint: spwan returns a handle, use it to join the thread)
}
