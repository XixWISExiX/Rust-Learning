use std::io;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn run() {
    println!("vectors!");

    let vec: Vec<i32> = Vec::new();

    // Creating a vec with initial values
    let vec: Vec<i32> = vec![];
    let mut vec = vec![1, 2, 3];

    vec.push(6);
    vec.push(7);
    println!("Vector Print: {vec:?}");

    let third: &i32 = &vec[2];
    {
        let third = &vec[2];
        //vec.push(8); // cannot push content while holding reference
        println!("The third element is {}", third);
    }
    
    vec.push(8);

    {
        let third: Option<&i32> = vec.get(2);
        match third {
            Some(third) => println!("There is a third element!"),
            None => println!("There isn't a third element..."),
        }
    }

    let vec = vec![100, 32, 57];
    {
        println!("Print all elements in the vector!");
        for v in &vec {
            println!("{}", v);
        }
    }

    let mut vec = vec![100, 32, 57];
    {
        println!("Print all elements in the vector!");
        for v in &mut vec {
            *v += 50; // dereference operator
            println!("{}", v);
        }

        let last_element = vec.pop();
        println!("Last element gone...");

        for v in &vec {
            println!("{}", v);
        }
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(6.7),
    ];
    for r in &row {
        println!("Element: {r:#?}");
    }

    let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
    vec.sort();
    let mut median = 0;
    for i in 0..(vec.len()/2)+1 {
        median = vec[i]; // copied value because int
    }
    println!("Median of {vec:?} is {median}");

    println!();
}

