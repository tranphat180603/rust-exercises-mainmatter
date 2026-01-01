// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value. x 
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.x
//   It should be possible to compare it with another `SaturatingU16` or a `u16`. x
//   It should be possible to print its debug representation.x
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct SaturatingU16{
    value: u16,
}

//u16
impl From<u16> for SaturatingU16{
    fn from(value: u16) -> Self {
        Self{value}
    }
}

//u8
impl From<u8> for SaturatingU16{
    fn from(value: u8) -> Self {
        Self{value: value as u16}
    }
}

//&u16
impl From<&u16> for SaturatingU16{
    fn from(value: &u16) -> Self {
        Self{value: *value}
    }
}

//&u8
impl From<&u8> for SaturatingU16{
    fn from(value: &u8) -> Self {
        Self{value: *value as u16}
    }
}
//Add SaturatingU16
impl Add for SaturatingU16{
    type Output = Self;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        Self{value: self.value.saturating_add(rhs.value)}
    }
}
//Add u16
impl Add<u16> for SaturatingU16{
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self{
            value: self.value.saturating_add(rhs)
        }
    }
}
//Add &u16 
impl Add<&u16> for SaturatingU16{
    type Output = Self;

    fn add(self, rhs: &u16) -> Self::Output {
        Self{
            value: self.value.saturating_add(*rhs)
        }
    }
}
//Add &SaturatingU16
impl Add<&SaturatingU16> for SaturatingU16{
    type Output = Self;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        Self{
            value: self.value.saturating_add(rhs.value)
        }
    }
}

//Compare SaturatingU16
impl PartialEq<u16> for SaturatingU16{
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
//Compare u16
