const fn calculate_pi(_iterations: usize) -> f64 {
    return 3.14159;
}

const PI_APPROXIMATION: f64 = calculate_pi(0);

fn main() {
    println!("Our approximation: {}", PI_APPROXIMATION);
}
