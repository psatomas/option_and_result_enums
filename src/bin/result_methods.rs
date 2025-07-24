fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let result = divide(10.0, 5.0);
    println!("{}", result.is_ok());
    println!("{}", result.is_err());
}