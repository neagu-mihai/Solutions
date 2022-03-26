/// tp2 / ex 5
/// Author: Alexandra Negoita
use std::env;

fn divide(numbers: Vec<i32>) -> Option<i32> {
    let mut res = numbers[0];
    // for index in 1..numbers.len(){
    //     if *numbers[index]==0{
    //         return None;
    //     }
    //     else{
    //         res=res/numbers[index];
    //     }
    // }
    for element in numbers.iter().skip(1) {
        if *element == 0 {
            return None;
        } else {
            res = res / element;
        }
    }
    return Some(res);
}
fn rem(numbers: Vec<i32>) -> Option<i32> {
    let mut res = numbers[0];
    for element in numbers.iter().skip(1) {
        if *element == 0 {
            return None;
        } else {
            res = res % element;
        }
    }
    return Some(res);
}
fn add(numbers: Vec<i32>) -> i32 {
    let mut res = 0;
    for element in &numbers {
        res += element;
    }
    return res;
}
fn substract(numbers: Vec<i32>) -> i32 {
    let mut res = 0;
    for element in &numbers {
        res -= element;
    }
    return res;
}
fn multiply(numbers: Vec<i32>) -> i32 {
    let mut res = 1;
    for element in &numbers {
        res *= element;
    }
    return res;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Nu ati introdus parametri corespunzatori.");
        std::process::exit(-1);
    } else {
        //$env:CMD="sub"
        let command = args[1].as_str();
        let mut numbers: Vec<i32> = vec![];
        for index in 2..args.len() {
            numbers.push(match args[index].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Introduceti doar numere.");
                    std::process::exit(-1);
                }
            });
        }
        // let a=match args[1].parse(){
        //     Ok(n) =>{
        //         n
        //     }
        //     Err(_) => {
        //         println!("A nu este un numar.");
        //         std::process::exit(-1);
        //     }
        // };
        // let b=match args[2].parse(){
        //     Ok(n) =>{
        //         n
        //     }
        //     Err(_) => {
        //         println!("B nu este un numar.");
        //         std::process::exit(-1);
        //     }
        // };
        if command == "add" {
            println!("a + b = {}", add(numbers));
        } else if command == "sub" {
            println!("a - b = {}", substract(numbers));
        } else if command == "mul" {
            println!("a * b = {}", multiply(numbers));
        } else if command == "div" {
            let p = divide(numbers);
            match p {
                None => println!("Impartire la zero."),
                Some(v) => println!("a / b = {}", v),
            }
        } else if command == "rem" {
            let p = rem(numbers);
            match p {
                None => println!("Impartire la zero."),
                Some(v) => println!("a % b = {}", v),
            }
        } else {
            println!("Comanda nu este valida.");
            std::process::exit(-1);
        }
    }
}
