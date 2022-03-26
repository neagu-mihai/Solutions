use crate::Vector;

#[test]
fn it_adds() {
    let mut v = Vector::new();

    v.add(3);
    v.add(2);

    assert_eq!(v.tabel, vec![2, 3]);
}

#[test]
fn it_removes() {
    let mut v = Vector::new();

    v.add(7);
    v.add(5);
    v.remove(5);

    assert_eq!(v.tabel, vec![7])
}

#[test]
fn nr_prime() {
    let mut v = Vector::new();

    v.add(1);
    v.add(2);
    v.add(3);
    v.add(4);
    v.add(5);

    let prime = v.prime_numbers();

    assert!(prime.tabel[0] == 2 && prime.tabel[1] == 3 && prime.tabel[2] == 5);
}

#[test]
fn is_between() {
    let mut v = Vector::new();

    v.add(1);
    v.add(2);
    v.add(3);
    v.add(4);
    v.add(5);
    v.add(6);

    let new = v.between(2, 3);

    assert!(new.tabel[0] >= 2 && new.tabel[new.tabel.len() - 1] <= 3);
}
