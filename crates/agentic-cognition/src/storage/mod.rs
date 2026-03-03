//! Storage layer — atomic writes, project isolation, locking

pub mod atomic;
pub mod isolation;

pub use atomic::AtomicWriter;
pub use isolation::{IsolationGuard, ProjectIdentity};
