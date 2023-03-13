fn main() {
    // cannot be reassigned only shadowing is allowed
    let immutable = 3;
    println!("The value of immutable is {immutable}");

    // shadowed ^ let immutable = 4;

    // cannot be reassigned to another type
    let mut x = 5;

    println!("The value of x is {x}");

    x = 6;

    println!("The value of x is {x}");

    const I_CANNOT_HAVE_MUT_MODIFIER: u32 = 60 * 60 * 3;

    println!("const value: {I_CANNOT_HAVE_MUT_MODIFIER}");

    let s = 5;

    let s = s + 1;

    {
        // outer s is unaffected by this shadow
        let s = s * 2;

        println!("The value of s in the inner scope is: {s}");
    }

    println!("The value of s is: {s}");

    /*
       The difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again,
       we can change the type of the value but reuse the same name.
    */
}
