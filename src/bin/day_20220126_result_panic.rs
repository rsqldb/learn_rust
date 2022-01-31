use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Error as IOError;

/*
* Result is important Rust enum, which have two values: OK/Err
* Can use match to handle different result cases explicitly.
* Or can use unwrap or except to handle panic
* Or can use return Result to pass over the panic
* ? is an useful operator for panic pass over
 */


fn main() {
    // open_file();
    // open_file_with_unwrap();
    // open_file_with_expect();
    // create_file();
    let result = read_file_with_question_operator().unwrap();
    println!("result text: {}", result)
}

fn create_file() {
    File::create("hello.txt");
}

fn open_file_with_match() -> File {
    let txt_file = File::open("hello.txt");
    match txt_file {
        Ok(file) => file,
        Err(error) => panic!("failed to open the file")
    }
}

fn open_file_with_unwrap() -> File {
  File::open("hello.txt").unwrap()
}

fn open_file_with_expect() -> File {
    File::open("hello.txt").expect("Can't open file")
}

fn read_file_with_error_handler() -> Result<String, IOError> {
    let mut opened_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut read_txt = String::new();
    match opened_file.read_to_string(&mut read_txt) {
        Ok(_) => Ok(read_txt),
        Err(e) => Err(e)
    }
}

fn read_file_with_question_operator() -> Result<String, IOError> {
    //? will call std::covert::from if error type is different, and the method have implemented a from method
    let mut opened_file = File::open("hello.txt")?;
    let mut read_txt = String::new();
    opened_file.read_to_string(&mut read_txt)?;
    Ok(read_txt)
}

fn read_file_simplify() -> Result<String, IOError> {
    let mut read_txt = String::new();
    File::open("hello.txt")?.read_to_string(&mut read_txt)?;
    Ok(read_txt)
}

//If want to use ? operation in main, need to make main return Result<(), Box<dyn Error>> type
// fn main() -> Result<(), Box<dyn Error>> {
//     let mut read_txt = String::new();
//     File::open("hello.txt")?.read_to_string(&mut read_txt)?;
//     println!("result text: {}", read_txt);
//     Ok(())
// }


struct Guess {
    value: i32
}

impl Guess {
    fn new(value:i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("please provide a value >=1 and value <= 100");
        }
        Guess {
            value
        }
    }

    //similar to java get method
    pub fn value(&self) -> i32 {
        self.value
    }
}

