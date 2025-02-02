// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

use std::ops::Add;

#[derive(Debug)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Add for WrappingU32 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value.wrapping_add(other.value),
        }
    }
}

impl PartialEq for WrappingU32 {
    fn eq(&self, other: &WrappingU32) -> bool {
        self.value == other.value
    }
}

impl Clone for WrappingU32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for WrappingU32 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
