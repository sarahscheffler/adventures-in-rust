// You can print stuff with "Debug" even if you can't with other things.

use std::fmt;

#[derive(Debug)]
struct Structure(i32);

// use {} and implement fmt::Display
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write strictly first element into the given output stream f.  Returns fmt::Result which
        // indicates success/failure
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("Printing Structure(3): {:?}", Structure(3));
    println!("Printing Deep(Structure(5)): {:?}", Deep(Structure(5)));
    println!("Printing structure 3 with curly braces: {}", Structure(3));
}

