pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod factory
{
    pub mod produce_refrigerator
    {
        pub fn produce_re()
        {
            println!("produce refrigerator!");
        }
    }
    pub mod produce_washing_machine
    {
        pub fn produce_washing_machine()
        {
            println!("produce washing machine!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
