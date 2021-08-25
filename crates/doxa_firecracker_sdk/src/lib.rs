//! An unofficial rust based SDK for Firecracker VM.
//! This SDK is designed entirely for use in Doxa but it does not depend on any Doxa component and
//! in future it could be extracted into it's own repository.
//!
//! This is designed to handle both spawning containers a child processes and communicate with
//! running Firecracker VMs for configuration.

pub mod error;
pub mod spawn;

mod net;
mod request;

pub use spawn::{VMOptions, VM};
