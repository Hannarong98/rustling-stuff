fn main() {
    println!("Hello, world!");

    do_math(2, 3);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn do_math(a: i32, b: i32) {
    let result: (i32, i32, i32) = (add(a, b), sub(a, b), mul(a, b));

    let (sum, diff, product) = result;

    println!("Calculate: {a} and {b}");
    println!("sum: {sum} | diff {diff} | product {product}");
}
