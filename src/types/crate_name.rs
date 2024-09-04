use subtype::newtype_string;
use subtype::{Empty, Not};

// TODO: Add crate name validation
newtype_string! {
    pub struct CrateName(String | Not<Empty>)
}
