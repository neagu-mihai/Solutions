/// tp2 / ex 2
/// Author: Amalia Simion


use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let a = match args[1].parse::<i32>() {
            Ok(result) => result,
            Err(_) => std::process::exit(-1),
        };

        let b = match args[2].parse::<i32>() {
            Ok(result) => result,
            Err(_) => std::process::exit(-1),
        };

        let division = division(a, b);

        match division {
            Some(result) => {
                println!("Rezultatul impartirii este {}", result);
            }
            None => {
                println!("Nu se poate imparti la 0");
            }
        }
    } else {
        println!("Introduceti 2 numere");
    }
}

fn division(a: i32, b: i32) -> Option<i32> {
    if b != 0 {
        return Some(a / b);
    } else {
        return None;
    }
}
