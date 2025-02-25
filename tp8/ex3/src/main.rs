use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut handles = vec![];

    let counts = Arc::new(Mutex::new(0));

    // TODO 1 - start 10 threads and print from each threads its number
    // (hint: use a for, the number is the index in your for)
    for i in 0..10 {
        let thread_counts = counts.clone();

        let t = thread::spawn(move || {
            let mut count = thread_counts.lock().unwrap();
            *count += 1;
            println!("thread {} done", i);
        });
        handles.push(t);
    }

    // TODO 2 - wait for all the 10 threads to finish
    // (hint: use a Vec to add the thread handles and iterate over
    // it here using for)
    for t in handles {
        t.join().ok();
    }

    println!("all threads are done");

    // TODO 3 - use a counter variable that is increased
    // by every thread
    // print its value after all the threads are done
    // (hint: use a Mutex wrapped into an Arc)

    let count = counts.lock().unwrap();

    // TODO 4 - print the counter
    println!("counts {}", count);
}
