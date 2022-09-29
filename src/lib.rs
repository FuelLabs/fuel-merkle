#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::bool_assert_comparison, clippy::identity_op)]

#[cfg_attr(test, macro_use)]
extern crate alloc;

pub mod binary;
pub mod common;
pub mod sparse;
pub mod sum;

trait Binary {
    fn prev_p2(self) -> Self;

    fn next_p2(self) -> Self;
}

impl Binary for u64 {
    fn prev_p2(self) -> Self {
        1 << (63 - self.leading_zeros())
    }

    fn next_p2(self) -> Self {
        let mut x = self;
        if x & (x - 1) != 0 {
            x = 1 << (64 - x.leading_zeros());
        }
        x
    }
}
