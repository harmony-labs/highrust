// Demonstrates variables and basic expressions in HighRust
// Transpiled to Rust by HighRust

fn calculate_area(width: i32, height: i32) -> i32 {
    let area = width * height;
    return area;
}

fn main() {
    let x = 5;
    let y = 10;
    let result = calculate_area(x, y);
    println!("The area is: {}", result);
}