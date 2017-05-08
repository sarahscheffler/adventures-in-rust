// Note: chars are 4 bytes because they're unicode
// bools are 'true' or 'false'
// unit type ()
// variables can be type annotated
let a_float: f64 = 1.0;
let or_this_way = 5i32;
let defaulting = 3.0; // f64, ints would be i32

let mut mutable = 12; // mutable gets set to i32
// mutable = true; // ERROR!  this can't change later!

// literals
// 'a' char
// "a" string
// 15 int
// 0xf - also an int
// 0o17 - octal int!
// 0b1111 - binary int
// you can stick underscores in willy nilly like 1_000 = 1000 and 0.000_001 = 0.000001
// 2.3 float
// true bool
// operators are &&, ||, !
// () unit
// bitwise operations &, |, ^, <<, >>

// tuples like normal, access members by .0, .1, etc.

use std::mem;

// arrays have weird definitions!
// type signature is [T; size]
let xs: [i32; 5] = [1,2,3,4,5]; // oh i super DO NOT LIKE THIS
let ys: [i32; 500] = [0; 500]; // initialize all to 0
println!("size: {}", xs.len());
println!("you can see how many bytes it takes up: {}", mem::size_of_val(&xs));

// slices are like arrays but size is not known at compile time. first word is pointer to data,
// second is length of slice in words (which have size usize, as given by architecture)
// type signature is &[T]
fn look_a_slice(slice: &[i32]) {
    println!("slice has size {}", slice.len());
}
// can slice arrays
look_a_slice(&ys);

// if we go out of bound we panic!
// println!("{}", xs[10]);


