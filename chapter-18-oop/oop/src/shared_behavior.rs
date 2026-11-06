// NOTE: This is a polymorphism example

pub fn run() {
    println!("shared_behavior");

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    println!("=====");
    screen.run();
    println!("=====");
}

pub trait Draw {
    fn draw(&self);
}

// NOTE: when using trait, rust must use dynamic dispatching (runtime)
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // NOTE: dyn is for a dynamic trait
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// NOTE: The not dynamic version of the code above.
//pub struct Screen<T: Draw> {
//    pub componets: Vec<T>, // NOTE: only a single trait in array :(
//}
//
//impl<T> Screen<T>
//where
//    T: Draw
//{
//    pub fn run(&self) {
//        for component in self.components.iter() {
//            component.draw();
//        }
//    }
//    
//}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("button draw");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("select box draw");
    }
}
