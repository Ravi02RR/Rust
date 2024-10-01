fn main() {
    // ! is not a function, it's a macro

    println!("Hello, world!");
    println!("{}", sum(2, 3));
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}
