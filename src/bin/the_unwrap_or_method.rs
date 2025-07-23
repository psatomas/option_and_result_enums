fn main() {
    let present_value = Some(13);
    let missing_value: Option<bool> = None;

    println!("{}", present_value.unwrap_or(0));
    println!("{}", missing_value.unwrap_or(true));
}