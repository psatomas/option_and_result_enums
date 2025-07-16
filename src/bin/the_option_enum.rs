fn main() {
    let a = Option::Some(5);
    let b = Option::Some("hello");
    let c = Option::Some(true);

    let a: Option<i32> = Option::Some(5);
    let a = Option::<i16>::Some(5);

    let d: Option<&str> = Option::None;
}