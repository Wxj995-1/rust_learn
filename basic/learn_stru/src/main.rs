#[derive(Debug)]
//1、定义结构体
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

#[derive(Debug)]
//6、元组结构体
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
//7、没有任何字段的类单元结构体
struct UnitStruct;

fn build_user(email: String, username: String) -> User {
    //4、参数名字和字段名字同名的简写方法
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    //2、创建结构体实例
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };

    //3、修改结构体字段
    let mut user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        active: false,
        sign_in_count: 2,
    };
    user2.email = String::from("new_user2@example.com");
    println!("user2 new email: {}", user2.email);

    let user3 = build_user(String::from("user3@xxx.com"), String::from("user3"));
    println!("user3 username: {}", user3.username);

    //5、从其它结构体创建实例
    let user4 = User {
        email: String::from("user4@xxx.com"),
        username: String::from("user4"),
        ..user1
    };
    println!("user4 active: {}", user4.active);

    //6、元组结构体
    let red = Color(255,0,0);
    let origin = Point(0,0);
    println!("red: {} {} {}", red.0, red.1, red.2);
    println!("origin: {} {}", origin.0, origin.1);

    //7、单元结构体
    let unit = UnitStruct;
    println!("单元结构体实例 {:?}", unit);

    //8、打印结构体
    println!("user1 = {:#?}", user1);
}
