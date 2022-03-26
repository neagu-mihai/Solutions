use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        if let Ok(list) = fs::read_dir(&args[1]) {
            for file in list {
                println!("{:?}", file.unwrap().path());
                // fara ""
                // nu orice cale se poate transforma in str, pot aparea erori
                // astfel ca to_str intoarce Result<Ok(s), Error>
                // println!("{}", file.unwrap().path().to_str().unwrap());
            }
        } else {
            eprintln!("Error");
        }
    } else {
        eprintln!("USAGE: ex1 folder");
    }
}
