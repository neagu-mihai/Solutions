/// tp2 / ex 3
/// Author: Amalia Simion


use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let cmnd = args[1].as_str();

        let a = match args[2].parse::<i32>() {
            Ok(result) => result,
            Err(_) => std::process::exit(-1),
        };

        let b = match args[3].parse::<i32>() {
            Ok(result) => result,
            Err(_) => std::process::exit(-1),
        };

        match cmnd {
            "add" => {
                println!("{}", add(a, b));
            }

            "sub" => {
                println!("{}", sub(a, b));
            }

            "mul" => {
                println!("{}", mul(a, b));
            }

            "div" => {
                let division = div(a, b);

                match division {
                    Some(result) => {
                        println!("Rezultatul impartirii este {}", result);
                    }
                    None => {
                        println!("Nu se poate imparti la 0");
                    }
                }
            }

            "rem" => {
                let remainder = rem(a, b);

                match remainder {
                    Some(result) => {
                        println!("Rezultatul impartirii este {}", result);
                    }
                    None => {
                        println!("Nu se poate imparti la 0");
                    }
                }
            }

            _ => {
                println!("Nu ati introdus comanda corecta");
            }
        }
    }
    else {
        println!("Nu ati introdus parametrii corect.");
    }
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}

fn div(a: i32, b: i32) -> Option<i32> {
    if b != 0 {
        return Some(a / b);
    } else {
        return None;
    }
}

fn rem(a: i32, b: i32) -> Option<i32> {
    if b != 0 {
        return Some(a % b);
    } else {
        return None;
    }
}
