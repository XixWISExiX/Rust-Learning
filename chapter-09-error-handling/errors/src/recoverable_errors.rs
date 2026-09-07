use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// Can rewrite main to look like this.
//fn main() -> Result<(), Box<dyn Error>> {
//    let greeting_file = File::open("hello.txt")?; // error returns not 0
//    Ok(()) // returns 0
//}

pub fn run() {
    println!("recoverable_errors.rs");

    // Handle recoverable errors with Result Enum.
    // enum Reusult<SuccessType, FailureType>
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // Finding file that doesn't exist is a recoverable error.
    // Where T is std::fs::file and F is std::io:Error.

    // Example of creating a file if the file doesn't exist
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file {error:?}");
            }
        },
    };

    // Alternative to the above code written with unwrap_or_else
    // METHOD COVERED DEEPER IN CHAPTER 13
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // Shortcuts for Panic on Error
    let greeting_file = File::open("hello.txt").unwrap();

    // We can also choose the panic error message with .expect();
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in the project");

    read_content_from_file();
    question_op_read_content_from_file();
    let file_content = simple_read_content_from_file();
    let file_content = file_content.unwrap(); // Move operator
    println!("{}", file_content);
    let file_content_char = last_char_of_first_line(file_content.as_str());
    println!("{}", file_content_char.unwrap());

    // Needs to be called where fn has return type Result.
    //let greeting_file = File::open("hello.txt")?;

}

// Propagating Errors through functions
fn read_content_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


// ? placed after a result eliminates lots of boiler plate
fn question_op_read_content_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn simple_read_content_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// ? also works for Option type
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
