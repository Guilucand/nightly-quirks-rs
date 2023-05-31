#![cfg_attr(debug_assertions, deny(warnings))]
#![cfg_attr(feature = "nightly", feature(new_uninit))]

pub mod branch_pred;
pub mod int_roundings;
pub mod iter;
pub mod slice_group_by;
pub mod slice_partition_dedup;
pub mod utils;

pub mod prelude {
    pub use crate::int_roundings::IntRoundings;
    pub use crate::iter::*;
    pub use crate::slice_group_by::SliceGroupBy;
    pub use crate::utils::NightlyUtils;
}
