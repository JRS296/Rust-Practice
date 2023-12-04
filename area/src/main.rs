
// Refactoring via Structs
#[derive(Debug)] // Enables format for println via {:?} for new datatypes, called an outer attribute via #
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn square(size: u32) -> Self { // Constructor - Associated Func WITHOUT self as first parameter, therefore NOT method
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels. Using Implemented Function - Methods",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));


    println!("Rectangle Content: {:?}", &rect1);

    dbg!(&rect1); //Another way of debugging values present in struct instance

    dbg!(Rectangle::square(5));

    rect1.set_width(5);

    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


// Refactoring via Tuples
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }