//@ [fstar] aeneas-args=-decreases-clauses
//@ [fstar,coq] subdir=misc
pub trait BoolTrait {
    // Required method
    fn get_bool(&self) -> bool;

    // Provided method
    fn ret_true(&self) -> bool {
        true
    }
}

impl BoolTrait for bool {
    fn get_bool(&self) -> bool {
        *self
    }
}

pub fn test_bool_trait_bool(x: bool) -> bool {
    x.get_bool() && x.ret_true()
}

#[allow(clippy::redundant_pattern_matching)]
impl<T> BoolTrait for Option<T> {
    fn get_bool(&self) -> bool {
        match self {
            Option::Some(_) => true,
            Option::None => false,
        }
    }
}

pub fn test_bool_trait_option<T>(x: Option<T>) -> bool {
    x.get_bool() && x.ret_true()
}

/* expected result:

record 'self BoolTrait_t =
  get_bool :: "'self ⇒ bool result"
  ret_true :: "'self ⇒ bool result"
  
definition boolTrait_ret_true_default :: "'self ⇒ bool result" where
  "boolTrait_ret_true_default self  = (
    Ok True
    )"

definition boolTraitBool_get_bool :: "bool ⇒ bool result" where
  "boolTraitBool_get_bool self  = (
    Ok self
    )"

definition boolTraitBool_ret_true :: "bool ⇒ bool result" where
  "boolTraitBool_ret_true self  = (
    Ok True
    )"

definition boolTraitBool :: "bool BoolTrait_t" where "boolTraitBool = (|
  get_bool = boolTraitBool_get_bool,
  ret_true = boolTraitBool_ret_true
|)"

definition test_bool_trait_bool :: "bool ⇒ bool result" where
  "test_bool_trait_bool x 
    = (
    b <- boolTraitBool_get_bool x;
    if b then boolTraitBool_ret_true x else Ok False
    )"

definition boolTraitOption_get_bool :: "'t option ⇒ bool result" where
  "boolTraitOption_get_bool self 
    = (
    case self of None => Ok False | Some _ => Ok True
    )"

definition boolTraitOption_ret_true :: "'t option ⇒ bool result" where
  "boolTraitOption_ret_true self  = (
    Ok True
    )"

definition boolTraitOption :: "('t option) BoolTrait_t"
  where "boolTraitOption = (|
  get_bool = boolTraitOption_get_bool,
  ret_true = boolTraitOption_ret_true
|)"

definition test_bool_trait_option :: "'t option ⇒ bool result" where
  "test_bool_trait_option x 
    = (
    b <- boolTraitOption_get_bool x;
    if b then boolTraitOption_ret_true x else Ok False
    )"
*/