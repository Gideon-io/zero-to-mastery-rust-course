// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl Box {
    fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Self {dimensions, weight, color}
    }

    fn print(&self) {
        self.dimensions.print(); //self is the struct we created back in the main function, this is a shortcut way compared to line 26's commented out part. Main difference is there is no param input into the print function, instead the param is initialised in the start with self.dim...
        println!("Weight: {}", self.weight);
        self.color.print();//Color::print(&self.color);
    }
}

struct Dimensions {
    length: i32,
    height: i32,
    width: i32,
}

impl Dimensions {
    fn print(&self) {
        println!("length: {}", self.length);
        println!("height: {}", self.height);
        println!("width: {}", self.width);
    }
}

enum Color {
    Blue,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Blue => println!("Color: Blue"),
            Color::Red => println!("Color: Red"),
        }
    }
}

fn main() {

    let small_box_d = Dimensions {
        length: 5,
        height: 6,
        width: 7
    };

    let small_box = Box::new(small_box_d, 23.0,Color::Blue);

    small_box.print();
}
