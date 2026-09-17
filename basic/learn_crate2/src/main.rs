use mylib::factory::produce_refrigerator;
use mylib::factory::produce_washing_machine as A;

fn main() {
    mylib::factory::produce_refrigerator::produce_re(); // 绝对路径
    produce_refrigerator::produce_re();

    A::produce_washing_machine();

    println!("Hello, world!");
}
