//@ [!isabelle] skip
// `default` is not support
fn test_unwrap_or<T>(x: Option<T>, d: T) -> T {
    x.unwrap_or(d)
}

/*
axiomatization core_option_Option_unwrap_or ::
  " 't option ⇒ 't ⇒  't result"
  
definition test_unwrap_or
   :: "'t option => 't => 't result " where
"test_unwrap_or x default = (
  core_option_Option_unwrap_or x default
) "

*/