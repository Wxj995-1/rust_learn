mod modA{
    #[derive(Debug)]
    pub struct A{
        pub number:i32,
        name:String,
    }

    impl A {
        pub fn new_a() -> A {
            A
            {
                number: 1111,
                name: String::from("A"),
            }
        }

        pub fn println_a(&self)
        {
            println!("number = {}, name = {}", self.number, self.name);
        }
    }


    pub mod modB
    {
        pub fn println_B()
        {
            println!("B!!!!");
        }
        
        pub mod modC
        {
            pub fn println_C()
            {
                println!("C!!!!!");
                super::println_B();
            }
        }
    }


}

use modA::A;
fn main() {
    let a = A::new_a();
    a.println_a();

    println!("number = {}",a.number);

    modA::modB::modC::println_C();


    println!("Hello, world!");
}
