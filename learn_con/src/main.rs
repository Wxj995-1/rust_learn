fn main() {
    let y = 99;
    // if
    if y == 1
    {
        println!("y == 1");
    }

    // if-else
    if y == 1
    {
        println!("y == 1");
    }else
    {
        println!("y != 1");
    }

    // if-else if -else
    if y == 1
    {
        println!("y == 1");
    }else if y == 0
    {
        println!("y == 0");
    }
    else
    {
        println!("y != 1 && y != 0");
    }

    // let中使用if
    let conditon = false;
    let x = if conditon{
        5
    }else
    {
        6
    };
    println!("x = {}", x);

    // loop
    let mut counter = 0;
    loop{
        println!("In loop");
        if counter == 10
        {
            break;
        }
        counter += 1;
    }

    let result = loop 
    {
        counter += 1;
        if counter == 20{
            break counter * 2;
        }
    };

    println!("counter = {}",result);

    // while
    let mut i = 0;
    while i != 10
    {
        i += 1;
    }
    println!("i = {}",i);

    // for
    let arr: [u32;5] = [1,2,3,4,5];
    for element in arr.iter()
    {
        println!("element = {}",element);
    }
    println!("Hello, world!");
}
