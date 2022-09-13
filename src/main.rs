use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();
    // if there is an error unwrapping, program will terminate
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

/* Rules of ownership in Rust:
1. Each value is owned by a variable
2. When the owner goes out of scope, the value will be deallocated
3. There can only be ONE owner at a time
*/