fn main() {
    let ok: Result<i8, &str> = Ok(5);
    println!("{ok:?}");
    let disaster: Result<i32, &str> = Err("Somethin went wrong");
    println!("{:?}", disaster);
}