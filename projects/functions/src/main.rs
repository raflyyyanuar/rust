fn main() {
    println!("Hello, world!");

    another_function(127);

    let six = six();

    println!("Output is {six}");
}

fn another_function(x : i8) {
    println!("This is from another function");
    println!("I received parameter {x}");
}

fn six() -> i32 {
    6
}