#![deny(warnings)]
#![cfg_attr(feature = "nightly", feature(new_uninit))]

pub mod branch_pred;
pub mod iter;
pub mod utils;

pub mod prelude {
    pub use crate::iter::*;
    pub use crate::utils::NightlyUtils;
}
