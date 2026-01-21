//@ [!isabelle] skip
#![allow(clippy::needless_lifetimes)]

/* Simple functions */

pub fn choose1<'a, T>(b: bool, x: &'a mut T, y: &'a mut T) -> &'a mut T {
    if b {
        x
    } else {
        y
    }
}

pub fn mul2_add1(x: u32) -> u32 {
    (x + x) + 1
}

pub fn use_mul2_add1(x: u32, y: u32) -> u32 {
    mul2_add1(x) + y
}

pub fn incr<'a>(x: &'a mut u32) {
    *x += 1;
}

pub fn use_incr() {
    let mut x = 0;
    incr(&mut x);
    incr(&mut x);
    incr(&mut x);
}

/* Recursion, loops */

pub enum CList<T> {
    CCons(T, Box<CList<T>>),
    CNil,
}

pub fn list_nth<'a, T>(l: &'a CList<T>, i: u32) -> &'a T {
    match l {
        CList::CCons(x, tl1) => {
            if i == 0 {
                x
            } else {
                list_nth(tl1, i - 1)
            }
        }
        CList::CNil => {
            panic!()
        }
    }
}

pub fn list_nth1<'a, T>(mut l: &'a CList<T>, mut i: u32) -> &'a T {
    while let CList::CCons(x, tl1) = l {
        if i == 0 {
            return x;
        }
        i -= 1;
        l = tl1;
    }
    panic!()
}

pub fn list_nth_mut<'a, T>(mut l: &'a mut CList<T>, mut i: u32) -> &'a mut T {
    match l {
        CList::CCons(x, tl1) => {
            if i == 0 {
                x
            } else {
                list_nth_mut(tl1, i - 1)
            }
        }
        CList::CNil => {
            panic!()
        }
    }
}

pub fn list_tail<'a, T>(l: &'a mut CList<T>) -> &'a mut CList<T> {
    match l {
        CList::CCons(_, tl1) => list_tail(tl1),
        CList::CNil => l,
    }
}


/* Cryptographic example */
fn mod_add(a: u32, b: u32) -> u32 {
    assert!(a < 3329);
    assert!(b < 3329);
    let sum = a + b; // 0 <= a + b <= 2 * 3329
    let res = sum.wrapping_sub(3329);
    let mask = res >> 16; // mask = 0xffff if a + b < 3329, mask = 0 otherwise
    let q = 3329 & mask; // q = 3329 if a + b < 3329, q = 0 otherwise
    res.wrapping_add(q)
}
