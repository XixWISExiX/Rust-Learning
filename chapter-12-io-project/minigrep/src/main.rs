use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args.into_iter()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // NOTE: Chapter 13 implementation 
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{
        // skip the first redundant arg
        args.next();

        // NOTE: This iterator implementation is quicker than the clone counter part.
        // this gives ownership outside the iterator
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg, 
            None => return Err("Didn't get file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })

    }


    // NOTE: Previous implementation
    //fn build(args: &[String]) -> Result<Config, &'static str> {
    //    // Default arg + 2 custom args
    //    if args.len() < 3 {
    //        return Err("not enough arguments");
    //    }

    //    let query = args[1].clone();
    //    let file_path = args[2].clone();

    //    let ignore_case = env::var("IGNORE_CASE").is_ok();

    //    // Normal:
    //    // cargo run -- to poem.txt
    //    // Ignoreing case:
    //    // IGNORE_CASE=1 cargo run -- to poem.txt

    //    Ok(Config { query, file_path, ignore_case })
    //}
}

use minigrep::{ search, search_case_insensitive };

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");        
    }
    //println!("With text:\n{contents}");

    Ok(())
}


// Non modular code
// fn main() {
// 
//     // NOTE: could get invalid unicode args with args_os() producing
//     // an OsString, but values would differ per platform
//     let args: Vec<String> = env::args().collect();
//     // Rust cannot infer type from the collect method
//     //dbg!(args); // debug macro
// 
//     let program_name = &args[0]; // NOTE: this one is always given
//     let query = &args[1];
//     let file_path = &args[2];
// 
//     println!("Program Name (always given): {}", program_name);
//     println!("Searching for {}", query);
//     println!("In file {}", file_path);
// 
//     // Read contents from txt file
//     // NOTE: when we use expect method on Result, we are expecting the Ok value
//     let contents: String = fs::read_to_string(file_path)
//         .expect("Should have been able to read the file");
// 
//     println!("With text:\n{}", contents);
// }
