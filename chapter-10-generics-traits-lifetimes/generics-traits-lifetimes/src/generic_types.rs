pub fn run () {
    println!("generic_types.rs");

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest result is {}", *result); // result is a pointer.

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest result is {result}"); // result is converted to value in print fn

    let integer = Point { x: 5, y: 10 };
    println!("{integer:?}");
    let float = Point { x: 1.0, y: 4.0 };
    println!("{float:?}");
    // Won't work because values need to be the same type (given this struct instance)
    //let mix = Point { x: 5, y: 4.0 }; 
    // Has different generic instance, so this works
    let mix = MixedPoint { x: 5, y: 4.0 }; 
    println!("{mix:?}");

    let single = MixedPoint { x: 5, y: 10 }; 
    println!("{single:?}");

    println!("Using generic impl to get integer x: {}", integer.x());
    let p1 = MixedPoint { x: 4, y: 5.0 };
    let p2 = MixedPoint { x: 1.0, y: 'c' };
    let p3 = p1.mixup(p2);
    println!("Mixing up the types {p3:?}");
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

// Generic function
// won't compile due to > operator
// (not every type has a compare operator)
//fn largest<T>(list: &[T]) -> &T {
//    let mut largest = &list[0];
//
//    for item in list {
//        if item > largest {
//            largest = item
//        }
//    }
//
//    largest   
//}

// Structs can implement generics
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// We can now see the structure of an Option
enum Option<T> {
    Some(T),
    None,
}

// Compiler under the hood makes these single type enum from the generic enum
enum Option_i32 {
    Some(i32),
    None,
}
enum Option_f64 {
    Some(f64),
    None,
}

// We can now see the structure of an Result
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// There is also generic implementations
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can have specific implementations too
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> MixedPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MixedPoint<X2, Y2>) -> MixedPoint<X1, Y2> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}
