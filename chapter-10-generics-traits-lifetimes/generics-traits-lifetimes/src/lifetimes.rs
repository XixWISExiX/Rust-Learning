pub fn run () {
    println!("lifetimes.rs");
    example1();
    example2();
    //&i32         // a reference
    //&'a i32      // a reference with an explicit lifetime
    //&'a mut i32  // a mutable reference with an explicit lifetime
    example3();
    example4();
    example5();
}

fn example1() {
    // NOTE: The following is an example of having a life time error
    //let r; // Longer life
    //{
    //    let x = 5; // Shorter life
    //    r = &x; // borrowed value doesn't live long enough
    //}

    let x = 5;
    let r = &x;

    println!("r: {r}");
    
}

fn example2() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

// NOTE: Compile is confused on which value will be referenced
//fn longest(str1: & str, str2: & str) -> &str {
//    if str1.len() > str2.len() { str1 } else { str2 } 
//}

// Lifetime 'a is defined for this function and references
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

// Since we are not returning y, we don't need to specify it's lifetime.
fn get_x<'a>(x: &'a str, y: &str) -> &'a str {
    x
    //"x" // this works
    //String::from("x").as_str() // this wouldn't work because the life time doesn't match the return type (this also results in dangling reference).
}

fn example3() {
    let string1 = String::from("long string is long");
    let wont_work;
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), &string2);
        wont_work = longest(string1.as_str(), &string2);
        println!("The longest string is {result}");
    }
    // wont_work life time expired due to string2 expiring
    //println!("The longest string is {wont_work}");

    println!("We can still access string1: {}", string1);
}

// Lifetimes must be defined for structs that have references.
// Lifetime for struct cannot outlive lifetime for part.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn example4() {
    let novel = String::from("Call me Ishmael. Some years ago...");   
    let first_sentence = novel.split('.').next().unwrap();
    //let first_sentence = novel.as_str(); // same concept
    let i = ImportantExcerpt {
        part: first_sentence, // lifetime definition is required
    };
    println!("{}", i.part);
    println!("{}", first_sentence);
}


// NOTE: Here we don't need to define lifetimes because this is
// an example of a Lifetime Elision (deterministic patterns that the
// Rust compiler can handle)
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Invalid: fails rule 2
//fn invalid_example(x: &str, y: &str) -> &str {}

// Lifetime must be labeled in impl if implementing a lifetime defined struct
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

struct Here {
    s: &'static str,
}

// Static lifetimes live for the whole duration of the program
fn example5() {
    let here = Here {
        s: "I have a static lifetime.", // String litteral has program lifetime
    };
    println!("{}", here.s);
}

