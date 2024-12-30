use subtype::subtype_string;
use subtype::{Empty, Not};

// TODO: Add crate name validation
subtype_string! {
    pub struct CrateName(String | Not<Empty>)
}
