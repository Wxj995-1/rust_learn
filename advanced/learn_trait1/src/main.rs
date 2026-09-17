// ==========2、定义 trait ==========
// 定义一个 trait：描述“可以打招呼”的行为
trait Greet {
    // 方法签名：没有实现，需要类型自己实现
    fn say_hello(&self);

    // ==========4、默认实现 ==========
    // 有函数体，实现这个trait的类型可以直接用，不用重写
    fn say_bye(&self) {
        println!("Good bye! (默认实现)");
    }
}

// ==========3、为结构体实现 trait ==========    
#[derive(Debug)]
struct Student {
    name: String,
}

impl Greet for Student {
    // 必须实现没有默认体的方法 say_hello
    fn say_hello(&self) {
        println!("Hello, 我是学生：{}", self.name);
    }
    // say_bye 不写，就直接使用 trait 的默认实现
}

#[derive(Debug)]
struct Teacher {
    name: String,
}

impl Greet for Teacher {
    fn say_hello(&self) {
        println!("Hello, 我是老师：{}", self.name);
    }
    // 重写默认方法：覆盖trait里的say_bye
    fn say_bye(&self) {
        println!("Teacher {}: 下课再见！", self.name);
    }
}

// ==========5、trait作为参数 ==========
// 写法1：trait bound 泛型写法（静态分发，单态化，推荐）
fn greet_someone<T: Greet>(item: T) {
    item.say_hello();
    item.say_bye();
}

// 写法2：impl trait 语法糖（同样静态分发）
fn greet_another(item: impl Greet) {
    item.say_hello();
}

fn main() {
    let stu = Student {
        name: String::from("小明"),
    };
    let tea = Teacher {
        name: String::from("张老师"),
    };

    greet_someone(stu);
    println!("-------");
    greet_someone(tea);
}
