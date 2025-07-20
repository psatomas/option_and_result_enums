fn main() {
    let music_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = music_instruments.get(2);
    play(bass);
    println!("{:?}", bass);

    let invalid_instrument= music_instruments.get(100);
    play(invalid_instrument);
}

fn play(instrument_option: Option<&String> ) {
    match instrument_option {
        Option::Some(instrument) => println!("Playing the {instrument}"),
        Option::None => println!("Singing with my voice"),
    }
}