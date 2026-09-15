//1 rust通过所有权机制来管理内存，编译器在编译就会根据所有权规则对内存的使用进行检查
//2 堆和栈: 编译的时候数据的类型和大小是固定的 就是分配在堆上的
//3 作用域
//4 String内存回收
//5 移动 move
//6 clone
//7 栈上数据拷贝
//8 函数和作用域

fn take_ownership(s: String) {
    println!("函数内:{}", s);
} // s离开作用域，自动释放堆内存

fn borrow_string(s: &String) { // 引用，只是借用，不拿所有权
    println!("{}", s);
}
fn main() {
    // ==========3. 作用域==========
    {
        let s_scope = "scope test";
        println!("作用域内:{}", s_scope);
    }
    // println!("{}",s_scope); // 离开{}，s_scope失效

    // ==========4.String 堆内存 ==========
    {
        let mut s_str = String::from("hello");
        s_str.push_str(" string");
        println!("String:{}", s_str);
    } // 自动drop回收堆内存

    // ==========5. Move 移动 ==========
    let s1 = String::from("hello move");
    let s2 = s1;
    println!("move s2:{}", s2);
    // println!("move s1:{}", s1); // s1所有权被移走，编译报错

    // ==========6. Clone深度拷贝 ==========
    let c1 = String::from("clone test");
    let c2 = c1.clone();
    println!("clone c1:{}, c2:{}", c1, c2);

    // ==========7. 栈上Copy类型 ==========
    // 常用的具有copy trait有：bool char 浮点 字符 元组
    let x = 100;
    let y = x;
    println!("copy x={},y={}",x,y);

    // ==========8. 函数所有权转移 ==========
    let s_func = String::from("function move");
    borrow_string(&s_func);
    println!("{}",s_func); // 所有权移进函数，外面不能再使用
}
