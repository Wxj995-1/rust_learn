// 返回闭包
/*
fn return_clo() -> impl Fn(i32) ->i32
{
    |x| x + 1
}
 */

fn return_clo() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let f = return_clo();
    println!("{}", f(10));   // 11
}