/*
trait	能调用几次	        对捕获变量的权限	调用方法（概念）
Fn	    任意次	            只能读（共享借用）	fn call(&self, ...)
FnMut	任意次	            可以改（可变借用）	fn call_mut(&mut self, ...)
FnOnce	只能一次	        可以消费/移动	fn call_once(self, ...)
*/


fn add_one(x:i32) -> i32
{
    x + 1
}

fn do_twice(f: fn(i32) -> i32, val: i32) -> i32
{
    f(val) + f (val)
}

fn wrapper_func<T>(t:T, v: i32) -> i32 where T: Fn(i32) -> i32
{
    t(v)
}

fn func(v: i32) -> i32
{
    v + 1

}


fn main() {
    let x = 5;
    let r = do_twice(add_one, x);
    println!("r = {}", r);

    // ******************************//
    let a = wrapper_func(|x| x + 1, 1);
    println!("a = {}", a);
    
    let b = wrapper_func(func, 1);
    println!("b = {}", b);
  


    println!("Hello, world!");
}

// 函数指针实现了Fn FnMut FnOnce