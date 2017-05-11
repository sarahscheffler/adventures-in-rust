fn main() {
    // 'let' binds values and literals to variables
    // you can not specify the type and it'll infer from context but i want to do this as little as
    // possible because i hate things that aren't explicitly typed.
    let an_int = 1u32;
    let _a_boolean = true;
    let _unit = ();

    let _copied_int = an_int;

    // unused variables prefixed with _
    let _unused_var = 3u32;

    // variables are IMMUTABLE by defualt; you have to make them mut explicitly
    let mut mutable_binding = 1;
    mutable_binding += 1;
    
    // variable bindings have scope, e.g. within a block { }
    {
        let short_lived = 2;
        let an_int = 10u32; // this one "shadows" the other one
        mutable_binding = 5;
    }

    println!("look, mutable_binding changed: {}", mutable_binding);
    println!("the 'let' stayed the same tho: {}", an_int);

    let an_int = 100u32; // this one "shadows" the other one

    println!("then we change it again to 100: {}", an_int);

    // you CAN declare first, init later.  but you shouldn't, and the
    // compiler won't let you use it uninitialized.

}
