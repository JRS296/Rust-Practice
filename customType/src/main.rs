// Utilize Rust's strong custom type  system to reduce for potential for errors, rather than verbosely addressing them all

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// Generics Implementation
#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let x = Guess::new(5);
    println!("{}", x.value());

    // Uncomment for Panic
    // let y = Guess::new(500);
    // println!("{}", y.value());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    println!("p1 = {:?}, p2 = {:?}, p3 = {:?}", p1,p2,p3);
}
