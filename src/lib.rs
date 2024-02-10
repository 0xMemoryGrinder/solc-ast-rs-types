#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error;

pub mod types;

#[macro_use]
mod r#macro;

#[cfg(feature = "visit")]
pub mod visit;

/*#[cfg(feature = "visit-mut")]
pub mod visit_mut;*/
