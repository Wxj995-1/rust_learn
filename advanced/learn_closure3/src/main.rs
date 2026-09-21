/* 
fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;

    assert!(equal_to_x(y));
    println!("Hello, world!");
}
*/

//闭包可以通过三种方式捕获其环境，它们对应函数的三种获取参数的方式，分别是获取所有权、可变借用、不可变借用。
//这三种捕获值的方式被编码为如下三个Fn trait：
//（1）FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其环境。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移进闭包。其名称的Once部分代表了闭包不能多次获取相同变量的所有权。
//（2）FnMut 获取可变的借用值，所以可以改变其环境。
//（3）Fn 从其环境获取不可变的借用值。
//当创建一个闭包时，rust会根据其如何使用环境中的变量来推断我们希望如何引用环境。由于所有闭包都可以被调用至少一次，因此所有闭包都实现了FnOnce。没有移动被捕获变量的所有权到闭包的闭包也实现了FnMut，而不需要对捕获的变量进行可变访问的闭包实现了Fn。
// 接收 Fn 闭包：不可变借用，可多次调用
fn call_fn<F: Fn()>(f: F) {
    f();
    f();
}

fn call_fn_mut<F: FnMut()>(f: &mut F) {   // 改为接收可变引用
    f();
    f();
}

fn call_fn_once<F: FnOnce()>(f: F) {
    f();
}

fn main() {
    // ========== 1. Fn：只读借用，可多次调用 ==========
    let s1 = String::from("Fn: hello");
    let closure_fn = || println!("{}", s1);
    call_fn(closure_fn);      // 闭包是 Copy，传入即复制
    closure_fn();             // 仍可用
    println!("s1还可用：{}", s1);
    println!("----------------");

    // ========== 2. FnMut：修改捕获变量，经过可变引用 ==========
    let mut val = 10;
    let mut closure_fnmut = || {
        val += 1;
        println!("FnMut val = {}", val);
    };
    call_fn_mut(&mut closure_fnmut);  // 只借可变引用，不移动
    closure_fnmut();                  // 借用已结束，可再用
    println!("val最后值：{}", val);
    println!("----------------");

    // ========== 3. FnOnce：消耗捕获的所有权，只能调用一次 ==========
    let s2 = String::from("FnOnce move string");
    let closure_fnonce = move || drop(s2);  // 消耗 s2 → 真 FnOnce
    call_fn_once(closure_fnonce);           // 移动进函数后即被消耗
    // closure_fnonce();  // 已被移动，且 FnOnce 只能调用一次
    // println!("{}", s2); // s2 所有权已 move 进闭包
}