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

/* expected result 


(* [constants0::X0]
   Source: 'tests/src/constants0.rs', lines 7:0-7:22 *)
definition x0_body ::  "u32 result" where "x0_body = Ok (0 :: u32)"
definition x0 :: u32 where "x0 = get_result x0_body"

(* [constants0::X1]
   Source: 'tests/src/constants0.rs', lines 9:0-9:29 *)
definition x1_body ::  "u32 result" where "x1_body = Ok core_num_U32_MAX"
definition x1 :: "u32" where "x1 = get_result x1_body"

(* [constants0::X2]
   Source: 'tests/src/constants0.rs', lines 12:0-15:2 *)
definition x2_body ::  "u32 result" where "x2_body = Ok (3 :: u32)"
definition x2 :: "u32" where "x2 = get_result x2_body"

(* [constants0::incr]:
   Source: 'tests/src/constants0.rs', lines 19:0-21:1 *)
definition incr :: "u32 ⇒ u32 result" where
"incr n = (
  u32_add n (1 :: u32)
)"

(* [constants0::X3]
   Source: 'tests/src/constants0.rs', lines 17:0-17:29 *)
definition x3_body ::  "u32 result" where "x3_body = incr (32 :: u32)"
definition x3 :: "u32" where "x3 = get_result x3_body"
*/