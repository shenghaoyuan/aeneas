//@ [!borrow-check] aeneas-args=-test-trans-units
//@ [coq,fstar] subdir=misc
//! The examples from the ICFP 2022 submission, all in one place.

// 2.1
pub fn ref_incr(x: &mut i32) {
    *x = *x + 1;
}

pub fn test_incr() {
    let mut x = 0i32;
    ref_incr(&mut x);
    assert!(x == 1);
}

/* expected result:

definition test_incr :: " unit result" where
  "test_incr = (x ← ref_incr (0 :: i32) ; massert (x = (1 :: i32)))"
*/

// choose is conflict with isabelle: it is a keyword
// 2.2
pub fn choose1<'a, T>(b: bool, x: &'a mut T, y: &'a mut T) -> &'a mut T {
    if b {
        return x;
    } else {
        return y;
    }
}

pub fn test_choose() {
    let mut x = 0;
    let mut y = 0;
    let z = choose1(true, &mut x, &mut y);
    *z = *z + 1;
    assert!(*z == 1);
    assert!(x == 1);
    assert!(y == 0);
}

// 2.3

pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
use List::Cons;
use List::Nil;

/* expected result
datatype 't List_t =
  List_Cons 't  "'t List_t" |
  List_Nil
*/ 

pub fn list_nth_mut<'a, T>(l: &'a mut List<T>, i: u32) -> &'a mut T {
    match l {
        Nil => {
            panic!()
        }
        Cons(x, tl1) => { /*tl is a function in isabelle, cannot use it as variable name*/
            if i == 0 {
                return x;
            } else {
                return list_nth_mut(tl1, i - 1);
            }
        }
    }
}

/* expected result:
  fun list_nth_mut
    :: "'t List_t ⇒ u32 ⇒ ('t × ('t ⇒ 't List_t)) result" where
    "list_nth_mut l i 
      = (
      case l of
      List_Cons x tl1 =>
        if i = (0 :: u32)
        then let back = λ (ret :: 't) . List_Cons ret tl1 in Ok (x, back)
        else (
          i1 <- u32_sub i (1 :: u32);
          (x1, list_nth_mut_back) <- list_nth_mut tl1 i1;
          let back =
            λ (ret :: 't) . let tl1 = list_nth_mut_back ret in List_Cons x tl1
          in
          Ok (x1, back))
      | List_Nil => Fail Failure
      )"

*/

pub fn sum(l: &List<i32>) -> i32 {
    match l {
        Nil => {
            return 0;
        }
        Cons(x, tl1) => {
            return *x + sum(tl1);
        }
    }
}

pub fn test_nth() {
    let mut l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let x = list_nth_mut(&mut l, 2);
    *x = *x + 1;
    assert!(sum(&l) == 7);
}

// 4.3
pub fn call_choose(mut p: (u32, u32)) -> u32 {
    let px = &mut p.0;
    let py = &mut p.1;
    let pz = choose1(true, px, py);
    *pz = *pz + 1;
    return p.0;
}

