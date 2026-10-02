# An I/O Project: Building a Command Line Program

This project is basically making a custom version of *grep* (*g*lobally search a *r*egular *e*xpression and *p*rint).

This project tries to use the following:
- Organizing code (chapter 7)
- Using vectors and strings (chapter 8)
- Handling errors (chapter 9)
- Using traits and lifetimes where appropriate (chapter 10)
- Writting tests (chapter 11)

## Minigrep Planning

We want to run something like the below code
`cargo run -- searchstring example-filename.txt`
`cargo run -- frog poem.txt`

Can also use environmental variables
`IGNORE_CASE=1 cargo run -- BoDy poem.txt`

Can pipe println!() into file etc.
`cargo run > output.txt`

Use `eprintln!()` to print out errors so that errors don't get piped into `output.txt`

## IO Project (topics)

- Accepting Command Line Arguments
- Reading a File
- Refactoring to Improve Modularity and Error Handling
- Adding Functionality with Test Driven Development
- Working with Environment Variables
- Redirecting errors to Standard Error
