/*fn say_hello()
{
    println!("hello, world!");

}*/

include! {concat!(env!("OUT_DIR"), "/hello.rs")}


fn main() {
    println!("{}", say_hello());
}
