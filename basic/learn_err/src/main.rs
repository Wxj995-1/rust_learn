use std::fs::File;
use std::io;

// 函数返回Result，所以内部可以使用 ? 问号运算符
fn read_file_demo(path: &str) -> Result<File, io::Error> {
    println!("尝试打开文件: {}", path);
    let file = File::open(path)?; // 如果Err，直接return错误给上层
    Ok(file)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ========== 1. panic! 不可恢复错误示例 ==========
    // 取消注释下面一行，程序直接崩溃
    // panic!("发生不可恢复的bug，程序panic！");

    // ========== 2. Result + match 处理可恢复错误 ==========
    let res = File::open("no_file.txt");
    match res {
        Ok(_f) => println!("文件打开成功"),
        Err(e) => println!("文件打开失败，错误信息：{}", e),
    }

    // ========== 3. ? 问号运算符示例 ==========
    let _file = read_file_demo("no_file.txt")?;

    // ========== 4. unwrap / expect 示例（Err时会panic） ==========
    // unwrap：Ok拿值，Err直接panic
    // let f = File::open("no_file.txt").unwrap();
    // expect：自定义panic提示
    // let f = File::open("no_file.txt").expect("文件找不到！");

    println!("程序正常结束");
    Ok(())
}
