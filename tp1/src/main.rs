/// tp 1
/// Author: Amalia Simion

fn main() {
    let mut v = Vector::new();

    v.add(5);
    v.add(7);
    v.add(2);
    v.add(0);
    v.add(3);
    v.add(9);

    v.display();
    println!();

    v.remove(3);
    v.remove(0);
    v.remove(9);

    v.display();

    let nr_prime: Vector = v.prime_numbers();
    let interval: Vector = v.between(2, 7);

    nr_prime.display();
    interval.display();
}

struct Vector {
    tabel: Vec<i32>,
}

impl Vector {
    fn new() -> Vector {
        Vector { tabel: vec![] }
    }

    fn add(&mut self, value: i32) {
        let mut pozitie: usize = 0;

        for index in 0..self.tabel.len() {
            if self.tabel[index] < value {
                pozitie += 1;
            }
        }

        self.tabel.insert(pozitie, value)
    }

    fn remove(&mut self, value: i32) {
        let mut pozitie = None;

        for index in 0..self.tabel.len() {
            if self.tabel[index] == value {
                pozitie = Some(index);
            }
        }

        match pozitie {
            Some(pozitie) => {
                self.tabel.remove(pozitie);
            }
            None => println!("Valoarea introdusa nu exista in vector!"),
        }
    }

    fn display(&self) {
        for index in 0..self.tabel.len() {
            print!("{} ", self.tabel[index]);
        }
    }

    fn prime_numbers(&self) -> Vector {
        let mut aux = Vector::new();

        for element in &self.tabel {
            let mut prim = true;

            if *element <= 1 {
                prim = false;
            } else {
                for index in 2..*element {
                    if element % index == 0 {
                        prim = false;
                    }
                }
            }

            if prim {
                aux.add(*element);
            }
        }

        return aux;
    }

    fn between(&self, mut min: i32, mut max: i32) -> Vector {
        let mut v = Vector::new();

        if min > max {
            let aux = min;
            min = max;
            max = aux;
        }

        for element in &self.tabel {
            if *element >= min && *element <= max {
                v.add(*element);
            }
        }

        return v;
    }
}

#[cfg(test)]
mod tests;

// {
//     use super::*;

//     #[test]
//     fn it_adds() {
//         let mut v = Vector::new();

//         v.add(3);
//         v.add(2);

//         assert!(v.tabel[0] == 2 && v.tabel[1] == 3);
//     }

//     #[test]
//     fn it_removes() {
//         let mut v = Vector::new();

//         v.add(7);
//         v.add(5);
//         v.remove(5);

//         assert_ne!(v.tabel[0], 5);
//     }

//     #[test]
//     fn nr_prime() {
//         let mut v = Vector::new();

//         v.add(1);
//         v.add(2);
//         v.add(3);
//         v.add(4);
//         v.add(5);

//         let prime = v.prime_numbers();

//         assert!(prime.tabel[0] == 2 && prime.tabel[1] == 3 && prime.tabel[2] == 5);
//     }

//     #[test]
//     fn is_between() {
//         let mut v = Vector::new();

//         v.add(1);
//         v.add(2);
//         v.add(3);
//         v.add(4);
//         v.add(5);
//         v.add(6);

//         let new = v.between(2, 3);

//         assert!(new.tabel[0] >= 2 && new.tabel[new.tabel.len() - 1] <= 3);
//     }
// }
