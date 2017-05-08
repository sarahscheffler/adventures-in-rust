// all structs are avalid enums.

enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info {name: String, height: i32}
}

enum Work {
    Civilian,
    Soldier,
}
enum Color {
    Red,
    Blue,
}

fn main() {

    // use 'use' to name names without scoping
    use Work::*;
    use Color::{Red, Blue};
    let mycolor = Red; // this means Color::Red now
}

// we can also do normal C-like enums
enum Number {
    Zero,
    One,
    Two,
} // implicit discriminator starts at 0

enum Cereal {
    Cheerios = 1,
    HBOO = 10,
}

// constants:
// const - unchangeable value
// static = possible mut-able variable with static lifetime
// string is different - it can be assigned to static without modification beecause its type sig is
// &'static str.  all other freference types must be explicitly annotated so they fulfill 'static.
