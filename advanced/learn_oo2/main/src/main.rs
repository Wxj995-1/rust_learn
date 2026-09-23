use gui::{Screen, Button, SelectBox};

fn main() {
    let s = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("ok"),
            }),
            Box::new(SelectBox {
                width: 60,
                height: 40,
                option: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("MayBe"),
                ],
            }),
        ],
    };

    // 遍历屏幕上所有组件，统一绘制
    for comp in s.components {
        comp.draw();
    }
    
    println!("Hello, world!");
}
