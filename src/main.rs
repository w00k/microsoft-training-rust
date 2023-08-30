use std::fmt;

// Debug is for print on consola for "{:?}"
// PartialEq is for comparison in the if "==" 
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// for p1 in println "{}", implement this function for fix it 
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 { // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1); // can't print using the '{}' format specifier!
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!

}