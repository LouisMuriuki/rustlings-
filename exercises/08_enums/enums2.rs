#![allow(dead_code)]

#[derive(Debug)]
 // TODO: define the different variants used below

enum Message {
    Move {x:u32, y:u32},
    Echo (String),
    ChangeColor(u32,u32,u32),
    Quit,
   
}
impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
