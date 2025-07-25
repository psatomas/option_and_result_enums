fn main() {
    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];

    while let Some(sauce) =sauces.pop() {
        println!("The next sauce is {sauce}");
    }
}