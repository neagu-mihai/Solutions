use crossbeam_utils::thread;

fn main() {
    let mut items = Vec::with_capacity(100_000_000);
    for i in 0..100_000_000 {
        items.push(i)
    }

    let mut max = *items.get(0).unwrap();
    for item in items.iter() {
        if max < *item {
            max = *item;
        }
    }

    // TODO 1 - run "time cargo run" and see how fast this program executes

    // TODO 2 - split the Vec into slices of size 100_000_000 / number of cores
    // (hint: use as_slice and chuncks)

    let slices = items.as_slice().chunks(100_000_000 / 24);

    // TODO 3 - use corssbeam::thread:scope to spawn a thread for each chunck
    // that computes the maximum of number of that chunk and returns it

    thread::scope(|s| {
        let mut thread_handles = vec![];
        for slice in slices {
            let t = s.spawn(|_| {
                let mut max = slice[0];
                for item in slice.iter() {
                    if max < *item {
                        max = *item;
                    }
                }
                max
            });
            thread_handles.push(t);
        }

        // TODO 4 - wait for all the threads and compute the maximum number returned

        // TODO 5 - use "time cargo run" to see how fast the program executes

        for thread_handle in thread_handles {
            let value = thread_handle.join().unwrap();
            if value > max {
                max = value;
            }
        }
    })
    .unwrap();

    println!("maximum value is {}", max);
}
