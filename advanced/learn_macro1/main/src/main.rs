use mac;

fn main() {
    let v = mac::my_vec![1,2,3];
    println!("v = {:?}", v);

    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    println!("temp_vec = {:?}", temp_vec);


    println!("Hello, world!");
}
