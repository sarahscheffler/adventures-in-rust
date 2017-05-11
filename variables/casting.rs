#![allow(overflowing_literals)]

fn main() {

    // no coercion, but there is casting.
    let decimal = 65.123_f32;

    let integer = decimal as u8;
    let character = integer as char;

    println!("casting: {} -> {} -> {}", decimal, integer, character);

    // if you cast, e.g., a big thing to u8, it'll subtract 2^8 until it fits
    // if you cast to signed, the bits'll be the same as if to unsigned,
    // and then it'll be negative if first bit is 1 (two's complement)
}
