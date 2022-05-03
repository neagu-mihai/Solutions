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

    // TODO 3 - use corssbeam::thread:scope to spawn a thread for each chunck
    // that computes the maximum of number of that chunk and returns it

    println!("maximum value is {}", max);
}
