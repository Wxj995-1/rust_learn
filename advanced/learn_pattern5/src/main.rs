enum Message
{
    Quit,
    Move {x:i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

enum Color
{
    Rgb(i32, i32, i32),  
    Hsv(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg
    {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("color, r = {}, g = {}, b = {}",r, g, b);
        }

        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("color, h = {}, s = {}, v = {}",h, s, v);
        }
        
        _ => (),
    }
    println!("Hello, world!");
}
