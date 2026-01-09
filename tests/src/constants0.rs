//@ [!borrow-check] aeneas-args=-test-trans-units
//@ [coq,fstar] subdir=misc
//! Tests with constants

// Integers

pub const X0: u32 = 0;

pub const X1: u32 = u32::MAX;

#[allow(clippy::let_and_return)]
pub const X2: u32 = {
    let x = 3;
    x
};

pub const X3: u32 = incr(32);

pub const fn incr(n: u32) -> u32 {
    n + 1
}