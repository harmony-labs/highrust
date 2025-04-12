// Demonstrates pattern matching in HighRust
// Transpiled to Rust by HighRust

fn process_value(value: i32) {
    match value {
        0 => println!("Zero"),
        1 => println!("One"),
        n if n < 10 => println!("Less than ten: {}", n),
        _ => println!("Ten or greater"),
    }
}

fn process_pair(pair: (i32, i32)) {
    match pair {
        (x, 0) => println!("Pair with second value zero: {}, 0", x),
        (0, y) => println!("Pair with first value zero: 0, {}", y),
        (x, y) if x == y => println!("Same values: {}, {}", x, y),
        (x, y) => println!("Different values: {}, {}", x, y),
    }
}

fn main() {
    process_value(5);
    process_pair((3, 3));
}