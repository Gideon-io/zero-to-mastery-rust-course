// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait PerimCalc{
    fn calculate(&self) -> i32;
}

struct Triangle {
    a: i32,
    b: i32,
    c: i32
}
impl PerimCalc for Triangle {
    fn calculate(&self) -> i32 {
        self.a + self.b + self.c
    }
}

struct Square {
    side: i32
}
impl PerimCalc for Square {
    fn calculate(&self) -> i32 {
        self.side * 4
    }
}

fn calc_perim(x: impl PerimCalc) {
    let print = x.calculate();
    println!("{}", print);
}
fn main() {
   calc_perim(Triangle {a: 5, b: 6, c: 7});
   calc_perim(Square {side: 4});

}
