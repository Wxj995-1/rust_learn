// ========== 1、类似于c语言的方式定义枚举（C风格枚举） ==========
enum IpAddrKind {
    V4,
    V6,
}

// ========== 2、rust语言提倡的方式定义：枚举可以直接绑定不同类型数据 ==========
enum IpAddr {
    V4(u8, u8, u8, u8), // 元组，存4个u8
    V6(String),         // 存String字符串
}

// ==========3、可以是不同类型 ==========
enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },    // 结构体类型
    Write(String),              // String
    ChangeColor(i32,i32,i32),   // 元组
}

// ==========4、经典用法：Option 枚举（Rust标准库最核心枚举） ==========
// 标准库定义：enum Option<T> { Some(T), None }

// ==========5、枚举类型的方法以及match匹配 ==========
impl Message {
    fn call(&self) {
        println!("执行message方法");
    }
}

fn main() {
    // 1.C风格枚举使用
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 2.Rust推荐枚举（携带数据）
    let localhost = IpAddr::V4(127,0,0,1);
    let ipv6_loop = IpAddr::V6(String::from("::1"));

    //3.不同类型变体
    let m1 = Message::Quit;
    let m2 = Message::Move{x:10,y:20};
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255,0,0);

    // 调用枚举的方法
    m1.call();

    // ========= match匹配枚举 =========
    match localhost {
        IpAddr::V4(a,b,c,d) => println!("ipv4: {}.{}.{}.{}",a,b,c,d),
        IpAddr::V6(s) => println!("ipv6:{}",s),
    }

    // Option经典例子
    let some_num = Some(5);
    let none_num: Option<i32> = None;
    match some_num {
        Some(val) => println!("有值：{}", val),
        None => println!("空"),
    }

    println!("Hello, world!");
}
