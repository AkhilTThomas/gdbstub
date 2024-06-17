//! `Register` structs for various Tricore architectures.

/// `RegId` definitions for Tricore architectures.
pub mod id;

mod tricore;

pub use tricore::TricoreCoreRegs;
