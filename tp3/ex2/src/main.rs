use std::fs;

// folosit pentru permsiunile de linux
use std::os::unix::fs::MetadataExt;

// folosit pentru aflarea file descriptor-ului folosind as_raw_fd()
use std::os::unix::io::AsRawFd;

// folosit pentru functia read
use std::io::Read;

fn permission_text(number: u32) -> String {
    let mut permisson_text = String::new();
    // daca bitul 2 e setat, avem r
    if number & 0b100 != 0 {
        permisson_text.push_str("r");
    } else {
        permisson_text.push_str("-");
    }
    // daca bitul 1 e setat, avem w
    if number & 0b010 != 0 {
        permisson_text.push_str("w");
    } else {
        permisson_text.push_str("-");
    }
    // daca bitul 0 e setat, avem x
    if number & 0b001 != 0 {
        permisson_text.push_str("x");
    } else {
        permisson_text.push_str("-");
    }
    permisson_text
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 {
        let filename = &args[1];
        let task = &args[2];
        match task.as_str() {
            "print" => {
                // read to string
                let data = fs::read_to_string(filename).expect("Error");
                println!("{}", data);
            }
            "print_buffer" => {
                // open read
                let mut file = fs::File::open(filename).expect("Error");
                println!("fd {}", file.as_raw_fd());
                // facem un buffer pentru date
                // putem folosi valori ma mici pentru dimensiunea
                // buffer-ului astfel incat sa trebuiasca sa citim de
                // ma multe ori din fisier
                let mut buffer = [0u8; 4096];
                // fisierul s-ar putea sa fie mai lung de 4096
                // de bytes, deci trebuie sa citim de mai multe ori
                loop {
                    // incercam sa citim cat putem in buffer
                    // intoarce Ok(bytes_cititi)
                    // sau Err(eroare_de_citire)
                    // daca nu avem eroare
                    let amount_read = file.read(&mut buffer);
                    match amount_read {
                        Ok(amount_read) => {
                            // daca e > 0, avem date
                            if amount_read > 0 {
                                println!("bytes {}", amount_read);
                                println!(
                                    "{}",
                                    std::str::from_utf8(&buffer[0..amount_read]).unwrap()
                                );
                            // daca e 0, am citit tot
                            } else {
                                break;
                            }
                        }
                        Err(error) => {
                            println!("read error {}", error);
                            break;
                        }
                    }
                }
            }
            "size" => {
                let file = fs::File::open(filename).expect("Error");
                println!("{}", file.metadata().expect("Error").len());
            }
            "owner" => {
                let file = fs::File::open(filename).expect("Error");
                println!(
                    "Uid {} Gid {}",
                    file.metadata().expect("Error").uid(),
                    file.metadata().expect("Error").gid()
                );
            }
            "mode_number" => {
                let file = fs::File::open(filename).expect("Error");
                let mode = file.metadata().expect("Error").mode();
                // pot imprima numarul in octal-baza opt {:o} si binar {:b}
                println!("{:b}", mode);
                println!("{:o}", mode);
                // mode are mai multe informatii decat rwxrwxrwx
                // ca sa scapam de ele, folosim doar ultimii 9 biti
                let permissons = mode & 0b111_111_111; // raman doar ultimii 9 biti
                println!("{:o}", permissons);
            }
            "mode_text" => {
                let file = fs::File::open(filename).expect("Error");
                let mode = file.metadata().expect("Error").mode();
                let user = mode >> 6 & 0b111;
                let group = mode >> 3 & 0b111;
                let others = mode & 0b111;
                println!(
                    "{}{}{}",
                    permission_text(user),
                    permission_text(group),
                    permission_text(others)
                );
            }
            "type" => {
                let file = fs::File::open(filename).expect("Error");
                let metadata = file.metadata().expect("Error");
                if metadata.is_dir() {
                    println!("directory");
                } else if metadata.is_file() {
                    println!("file");
                } else if metadata.is_symlink() {
                    // pot testa cu fisiere din /etc, multe sunt link-uri
                    println!("link");
                } else {
                    println!("other");
                }
            }
            _ => eprintln!("Incorrect task"),
        }
    } else {
        eprintln!("USAGE: ex2 filename task");
    }
}
