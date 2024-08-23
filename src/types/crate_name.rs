use subtype::checkers::{Empty, Not};
use subtype::newtype_string;

// TODO: Add crate name validation
newtype_string! {
    pub struct CrateName(String | Not<Empty>)
}
