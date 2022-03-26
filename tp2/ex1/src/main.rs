/// tp2 / ex 1
/// Author: Amalia Simion


fn main() {
    let division = division(3, 0);

    match division {
        Some(result) => {
            println!("Rezultatul impartirii este {}", result);
        }
        None => {
            println!("Nu se poate imparti la 0");
        }
    }
}

fn division(a: i32, b: i32) -> Option<i32> {
    if b != 0 {
        return Some(a / b);
    } else {
        return None;
    }
}
