fn main() {
    // {} makes stringified arguments
    println!("{} days", 31);

    // you can use positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments
    println!("{subject} {verb} {object}", 
             object="a thing", 
             verb="could be", 
             subject="this");

    // special formatting after a :
    println!("{:b} is {} in binary", 5, 5);

    // right-align with width
    println!("{number:>width$}", number=1, width=6);
    println!("{number:0>width$}", number=1, width=6);

    // create a structure with an i32
    #[allow(dead_code)] // what's going on here?
    struct Structure(i32);
    // can't print structs without more work

    println!("pi is roughly {:.*}", 4, 3.14159);
}
