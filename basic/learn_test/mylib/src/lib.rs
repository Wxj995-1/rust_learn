pub mod animal;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use animal::Cat;
    use animal::Dog;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn use_cat()
    {
        Cat::hello();
        assert_eq!(Cat::is_cat(),true);
    }
    #[test]
    fn use_dog()
    {
        Dog::hello();
        assert_eq!(Dog::is_dog(),true);
    }
}
