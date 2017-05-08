use std::fmt;

// Note: these are equivalent - return error if it errors, else continue.
// try!(write!(f, "{}", value));
// write!(f, "{}", value)?;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?; // write starting bracket

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?; // write commas if this is not first elem
            }
            write!(f, "{}: {}", count, v)?; // write actual elem
        }

        write!(f, "]") // close opened bracket and return fmt::Reuslt
            //TODO: WHY NO CURLY BRACE HERE
    }
}

fn main() {
    let v = List(vec![1,2,3]);
    println!("{}", v);
}
