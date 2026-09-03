pub fn run() {
    println!("strings!");

    let mut s = String::new();

    let data = "initial contents"; // &str type
    let s = data.to_string(); // &str --> String type
    let s = &s; // String --> &str type

    let hello = String::from("こんにちは");

    let mut s = String::from("initial contents");
    println!("{}", s);
    s.push_str(" + more contents");
    println!("{}", s);
    s.push_str(&String::from(" + more contents"));
    println!("{}", s);
    s.push('!'); // push only adds one char
    println!("{}", s);

    // Can concat with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    // s1 is no longer valid b/c we took ownership
    // of s1 and appended s2, making s3.
    println!("{}", s3);

    // You can use format!() instead of add
    let s1 = "tic";
    let s2 = "tac";
    let s3 = "toe";
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    //s[0]; // not valid code
    // This is the reason string bytes can vary

    let hello = String::from("Hola");
    println!("String '{}' Len in Bytes: {}", hello, hello.len());
    // 1 byte per normal char
    let hello = String::from("Здравствуйте");
    println!("String '{}' Len in Bytes: {}", hello, hello.len());
    // 2 bytes for non-english chars
    let hello = String::from("😀🥶");
    println!("String '{}' Len in Bytes: {}", hello, hello.len());
    // 4 bytes for emoji like chars
    let hello = String::from("Hд😀🥶");
    println!("String '{}' Len in Bytes: {}", hello, hello.len());
    // can mix them too

    // How to actually acces characters in Rust.
    for c in hello.chars() {
        println!("{}", c);
    }

    // You can also do bytes if you want to.
    for b in hello.bytes() {
        println!("{}", b);
    }

    let pig_word = pig_latinify("Dogman eats the horse");
    println!("Pig Latin?!: {}", pig_word);

    println!();
}

// Only works with 1-byte chars
fn pig_latinify(str: &str) -> String {
    let mut pig_str = String::from("");
    for word in str.split_whitespace() {
        let mut word_chars = word.chars(); 
        let first_char = word_chars.next().unwrap(); // chars arr doesn't have first char now
        pig_str += &format!("{}-{}ay ", &word_chars.as_str(), &first_char);                
    }
    return pig_str;
}
