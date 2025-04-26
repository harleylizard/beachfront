#![allow(private_bounds)]
use std::ops::{Add, Sub};

pub use discriminant_proc_macro::Enum;

/// A trait representing the minimum and maximum values for discriminants of an enum, as well as
/// their type.
///
/// This trait *should* not be implemented by hand, instead, it should be derived with
/// `#[derive(Enum)]`.
pub unsafe trait Enum {
    type Discriminant: Ord + Add + Sub;

    const MIN: <Self as Enum>::Discriminant;
    const MAX: <Self as Enum>::Discriminant;
}

/// Operations for discriminant operations on field-less, well behaved enums.
///
/// This trait should not be implemented by hand, instead, it should be implemented with
/// `#[derive(Enum)]`.
pub unsafe trait FromDiscriminant: Enum + Sized {
    /// Create an enum variant from a discriminant.
    /// If the discriminant is not valid, the function returns `None`.
    fn from_discriminant(d: <Self as Enum>::Discriminant) -> Option<Self>;
    /// Returns the enum variant with the discriminant which is 1 bigger than the current.
    /// This is usually the variant that is defined after the current one.
    ///
    /// If there is no discriminant with a bigger value or a next variant, the function returns
    /// `None`.
    fn next(self) -> Option<Self>;
    /// The inverse of [`FromDiscriminant::next`].
    fn previous(self) -> Option<Self>;
}

#[cfg(test)]
mod tests {
    use crate::{Enum, FromDiscriminant};

    #[allow(dead_code)]
    #[derive(Enum, Debug, PartialEq)]
    #[repr(u8)]
    enum E {
        A,
        B,
        C,
    }

    #[test]
    fn try_from_discriminant() {
        let d = E::A as u8;

        assert_eq!(E::from_discriminant(d).unwrap(), E::A);
    }
}
