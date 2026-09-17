/* 
// 定义两个trait，用来做约束演示
trait Speak {
    fn speak(&self);
}

trait Run {
    fn run(&self);
}

// 结构体，同时实现 Speak + Run
struct Dog;
impl Speak for Dog {
    fn speak(&self) {
        println!("汪汪");
    }
}
impl Run for Dog {
    fn run(&self) {
        println!("小狗在跑");
    }
}

struct Cat;
impl Speak for Cat {
    fn speak(&self) {
        println!("喵喵");
    }
}
impl Run for Cat {
    fn run(&self) {
        println!("小猫在跑");
    }
}

// ==========1、trait bound 基础语法==========
// T 必须实现 Speak
fn speak<T: Speak>(animal: T) {
    animal.speak();
}

// ==========2、多个 trait bound，用 + 连接==========
// T 必须同时实现 Speak + Run
fn speak_and_run<T: Speak + Run>(animal: T) {
    animal.speak();
    animal.run();
}

// 等价写法：where子句，代码长的时候推荐
fn speak_and_run_where<T>(animal: T)
where
    T: Speak + Run,
{
    animal.speak();
    animal.run();
}

// ==========3、返回 trait 类型 impl Trait==========
// 返回任意实现了 Speak 的类型，静态分发
fn get_animal() -> impl Speak {
    Dog
}

// 注意：同一个分支不能返回不同类型！下面这个是编译报错
// fn get_animal_bad(flag:bool) -> impl Speak {
//     if flag { Dog } else { Cat }
// }

fn main() {
    let dog = Dog;
    let cat = Cat;

    println!("=====基础trait bound=====");
    speak(dog);
    speak(cat);

    println!("=====多trait约束=====");
    let dog2 = Dog;
    speak_and_run(dog2);

    println!("=====返回impl Speak=====");
    let animal = get_animal();
    animal.speak();
}
*/

trait GetName
{
    fn get_name(&self) -> &String;
}
trait GetAge
{
    fn get_age(&self) -> u32;
}

// T 需要同时满足 GetName + GetAge 两个trait bound
/* 
fn print_information<T: GetName + GetAge>(item: T)
{
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}
*/

// 写法二
fn print_information<T>(item: T) where T: GetName + GetAge
{
    println!("name = {}", item.get_name());
    println!("age = {}", item.get_age());
}


#[derive(Debug)]
pub struct Student
{
    pub name: String,
    pub age: u32,
}

impl GetName for Student
{
    fn get_name(&self) -> &String
    {
        &self.name
    }
}

impl GetAge for Student
{
    fn get_age(&self) -> u32
    {
        self.age
    }
}


fn produce_item_with_age() -> impl GetAge + std::fmt::Debug
{
    Student{
        name : String::from("xiaoming"),
        age : 17,
    }
}


fn main()
{
    // 结构体实例化：大括号！！
    let s = Student { name: "xiaoming".to_string(), age:10 };
    print_information(s);

    let s = produce_item_with_age();
    println!("{:#?}",s);
    println!("hello world!");
}
