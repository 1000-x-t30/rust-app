use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // let _f_unwrap = File::open("hello.txt").unwrap();
    let _f_expect = File::open("hello.txt").expect("Failed to open hello.txt");
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e);
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}