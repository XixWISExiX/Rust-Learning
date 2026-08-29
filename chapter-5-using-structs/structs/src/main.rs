fn main() {
    defining_and_instantiating_structs();
    println!();
    an_example_program_using_structs();
    println!();
    methods();
}

// Structs allow you to name a group of data that
// correspond to one another.
struct User {
    active: bool,
    username: String,
    //username: &str, // Cannot do this yet, requires the use of lifetimes
    email: String,
    sign_in_count: u64,
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
// NOTE: These are basically a compiler enfored single value type
struct AlwaysEqual;

fn defining_and_instantiating_structs() {
    // immutable struct
    let user1 = User {
        active: true,
        username: String::from("Username123"),
        email: String::from("someone@gmail.com"),
        sign_in_count: 1,
    };
    print_user(&user1);

    // mutable struct
    let mut user2 = User {
        active: true,
        username: String::from("zUsername123"),
        email: String::from("zsomeone@gmail.com"),
        sign_in_count: 2,
    };
    // can assign individual params for mutable structs
    user2.email = String::from("anothersomeone@gmail.com");
    print_user(&user2);

    // Moves use other user args and assigns to a new user
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: user2.email,
        sign_in_count: user1.sign_in_count,
    };
    print_user(&user2);

    let user2 = User {
        email: String::from("yes@gmail.com"),
        ..user2 // Move remaining fields from user2
    };
    print_user(&user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    //User {
    //    active: true,
    //    username: username,
    //    email: email,
    //    sign_in_count: 1, 
    //}

    // Same as the above statement
    User {
        active: true,
        username, // NOTE: variable and parameter name need to be spelt the same
        email,
        sign_in_count: 1, 
    }
}

fn print_user(user: &User) {
    println!("-----");
    println!("Username: {}", user.username);
    println!("Active status: {}", user.active);
    println!("Email: {}", user.email);
    println!("Sign in count: {}", user.sign_in_count);
    println!("-----");
}

// This is a Trait, here the Trait value is "Debug"
#[derive(Debug)] // Enables Debug mode on the struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn an_example_program_using_structs() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    // Or you could write the above as
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area_tuple(rect1));

    // Or you could write the above as
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area_struct(&rect1));

    println!("rect1 is {rect1:?}"); // :? or :#? is the Debug output format
    println!("rect1 is {rect1:#?}"); // :? or :#? is the Debug output format

    // NOTE: dbg!() is a macro that takes ownership of an expression
    // unlike println!(), and prints to the stderr stream.
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // can debug just this statement
        height: 50,
    };
    dbg!(&rect1); // can also debug the object
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// This is an implementation block, this needs the same name
// as the struct which it's going to be assigned to.
impl Rectangle {
    // &self lets you get a reference to all the arguments
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    // Associated function example (doesn't need self, yet in impl)
    // Can operate like a constructor as shown below.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

// You can have multiple impl (implementation blocks) if desired
impl Rectangle {
    // Another example of an associated function
    fn hello() {
        println!("Rectangle says hello!");
    }
}

fn methods() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    rect1.set_height(67);
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Rectangle::func() is how to use associated functions
    let sq = Rectangle::square(3);
    println!("The area of the square is {} square pixels.", sq.area());
    Rectangle::hello();
}
