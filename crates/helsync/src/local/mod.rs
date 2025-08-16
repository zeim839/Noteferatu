//! Local virtual [FileSystem](crate::core::FileSystem) implementation.

mod client;
pub use client::*;

mod file;
pub use file::*;

mod schema;
pub use schema::*;

mod tags;
pub use tags::*;
