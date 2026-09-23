// @运算符 允许我们在创建一个存放值的变量的同时，测试这个变量是否匹配模式
enum Message
{
    Hello{id: i32},
}

fn main() {
    let msg = Message::Hello{id: 5};
    match msg
    {
        Message::Hello{id: id_va @ 3..= 7} => {println!("id_va = {}",id_va);},
        Message::Hello{id: id_va @ 10..= 20} => {println!("large")},
        Message::Hello{id} => {println!("id = {}", id);},
    }

    println!("Hello, world!");
}
