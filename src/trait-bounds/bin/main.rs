use std::time::Duration;

fn add<T: std::ops::Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}
fn main() {
    println!("Trait bounds for addition");
    // Add i32
    println!("Add 5 + 5: {}", add(5, 5));
    // Add f64
    println!("Add 5.5 + 5.5: {}", add(5.5, 5.5));
    // Add Durantion(60) + Duration(35)
    println!("Add 60 seconds + 35 seconds: {:?}", add(Duration::from_secs(60), Duration::from_secs(35)));
}