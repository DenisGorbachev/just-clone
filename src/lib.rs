#![deny(clippy::arithmetic_side_effects)]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]

use tokio as _;

mod crates_io;
mod docs_rs;
mod functions;
mod github_com;
mod traits;
mod types;

pub use crates_io::*;
pub use docs_rs::*;
pub use functions::*;
pub use github_com::*;
pub use traits::*;
pub use types::*;
