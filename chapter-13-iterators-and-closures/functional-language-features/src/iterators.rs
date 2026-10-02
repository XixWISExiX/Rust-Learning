pub fn run() {
    println!("iterators.rs");
    example1();
}


// NOTE: Iterator class example
//pub trait Iterator {
//    type Item;
//    fn next(&mut self) -> Option<Self::item>;
//}


fn example1() {
    let v1 = vec![1, 2, 3];
    // This does nothing untill the iterator is used
    let v1_iter = v1.iter();

    // iterators allow you to skip the indexing logic
    // NOTE: iterator made mutable behind the scenes
    for val in v1_iter {
        println!("Got: {val}");
    }

    // NOTE: Cannot do this, iterator is used up.
    //for val in v1_iter {
    //    println!("Got: {val}");
    //}

    println!("v1 = {v1:?}");
    // map calls a new iterator
    // collect consumes the iterator to make a vector
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2 (mapping a closure) = {v2:?}");
    
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter takes ownership of the vector (so it doesn't return a reference)
    // filter takes in a closure that returns a bool
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: u32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}

