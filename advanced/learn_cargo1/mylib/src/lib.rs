//! My Crate
//!
//! `mylib` is a collection of utilities to make performing certain
//! calculations more convenient.

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Add one to the number given.
///
/// # Example
///
/// ```
/// let five = 5;
/// assert_eq!(6, mylib::add_one(five));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
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