<<<<<<< HEAD:exercises/enums/enums1.rs
// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor
=======
#[derive(Debug)]
enum Message {
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
>>>>>>> main:solutions/08_enums/enums1.rs
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
