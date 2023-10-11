// enums1.rs
//
// No hints this time! ;)



#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit, Move, ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
