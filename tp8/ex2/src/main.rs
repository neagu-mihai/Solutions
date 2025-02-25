use std::thread;
use std::time::Duration;

fn main() {
    let t = thread::spawn(|| {
        for i in 0..10 {
            println!("thread {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for i in 0..10 {
        println!("main {}", i);
        thread::sleep(Duration::from_millis(500));
    }
    // TODO 2 - wait for the thread to finish
    // (hint: spwan returns a handle, use it to join the thread)
    t.join().ok();
}
