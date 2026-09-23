use std::fmt;

trait OutPrint: fmt::Display {
    fn out_print(&self) {
        let output = self.to_string();
        println!("output = {}", output);
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutPrint for Point {}

fn main() {
    let p = Point { x: 1, y: 2 };
    p.out_print();               // 想看到效果需要真正调用它
    println!("Hello, world!");
}