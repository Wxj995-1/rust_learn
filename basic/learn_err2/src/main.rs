use std::fs::File;
use std::io::Read;

// 1. 原始写法：手动match，传播错误（不推荐，繁琐）
#[allow(dead_code)]
fn read_content_manual() -> Result<String, std::io::Error> {
    let file_res = File::open("test.txt");
    let mut file = match file_res {          // 修正：必须加 mut
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

// 2. 推荐简写：? 问号运算符，就是上面match的语法糖【错误传播简写】
#[allow(dead_code)]
fn read_content_question() -> Result<String, std::io::Error> {
    let mut file = File::open("test.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 3. 更进一步简写：链式调用
#[allow(dead_code)]
fn read_content_chain() -> Result<String, std::io::Error> {
    let mut content = String::new();
    File::open("test.txt")?.read_to_string(&mut content)?;
    Ok(content)
}

// 4. 什么时候用panic!，什么时候用Result
/*
使用 panic!：
- 程序出现不可恢复bug，没办法补救
- 开发阶段，明确这个错误一定不会发生（断言）
- 示例：数组越界、配置硬编码一定合法

使用 Result<T,E>：
- 预期内可能失败的IO、网络、文件操作
- 失败是正常业务场景，调用者可以重试/提示用户
*/

// 5. Option 和 Result 的对比
/*
Option<T>：代表【有值/无值】，没有错误信息
    enum Option<T> { Some(T), None }
    场景：查询元素，有可能不存在，不存在不是错误，只是空。

Result<T,E>：代表【成功/失败】，失败携带错误详情E
    enum Result<T,E> { Ok(T), Err(E) }
    场景：操作可能失败，失败需要携带错误原因。

转换：
Option可以转Result：ok_or("错误信息")
Result可以转Option：ok() / err()
*/
#[allow(dead_code)]
fn option_demo() -> Option<i32> {
    let v = vec![1, 2, 3];
    v.get(2).copied()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 调用错误传播函数
    match read_content_question() {
        Ok(text) => println!("文件内容：{}", text),
        Err(e) => println!("读取失败，调用者处理错误：{}", e),
    }

    // Option示例
    let opt = option_demo();
    match opt {
        Some(val) => println!("option值：{}", val),
        None => println!("找不到值"),
    }

    Ok(())
}