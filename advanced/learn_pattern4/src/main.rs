enum Message
{
    Quit,
    Move {x:i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}


fn main() {
    let msg = Message::ChangeColor(0,160,255);
    match msg
    {
        Message::Quit => {
            println!("quit");
        },
        Message::Move{x, y} => {
            println!("move x = {}, y = {}",x, y);
        },
        Message::Write(str) => println!("write msg = {}",str),
        Message::ChangeColor(R,G,B) => {
            println!("color, r = {}, g = {}, b = {}",R, G, B);
        }
    }
    println!("Hello, world!");
}
