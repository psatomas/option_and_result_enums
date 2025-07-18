fn main() {
    let music_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = music_instruments.get(2);
    println!("{:?}", bass);
    let valid_instrument = bass.expect("Unable to retrieve element");
    println!("{valid_instrument}");

    let invalid_instrument= music_instruments.get(100);
    println!("{:?}", invalid_instrument);
    invalid_instrument.expect("Unable to retrieve element");
}