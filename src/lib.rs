//! # take_ref
//! 
//! take_ref provides the `TakeRef`, `TakeSlice`, and `TakeString` traits
//! to enable treating values and their reference/slice types interchangeably
//! until the moment taking ownership is required.
//! If taking ownership is required sometimes but not always,
//! this can eliminate the need to always copy data from reference types.

#![crate_name = "take_ref"]

#![cfg_attr(not(any(feature="use_std", doc)), no_std)]

mod take_ref;
pub use crate::take_ref::TakeRef;

#[cfg(any(feature = "use_std", doc))]
mod take_slice;
#[cfg(any(feature = "use_std", doc))]
pub use crate::take_slice::TakeSlice;

#[cfg(any(feature = "use_std"))]
mod take_string;
#[cfg(any(feature = "use_std"))]
pub use crate::take_string::TakeString;
