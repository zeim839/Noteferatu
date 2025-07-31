//! Local [Filesystem](crate::Filesystem) with optional cloud sync.
mod file;
pub use file::*;

mod local;
pub use local::*;

mod schema;
pub use schema::*;

mod tags;
pub use tags::*;
