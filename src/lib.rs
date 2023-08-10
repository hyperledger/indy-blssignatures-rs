//! BLS signatures for Indy

#![deny(missing_debug_implementations, rust_2018_idioms, unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "log")]
#[macro_use]
extern crate log;

/// Error type
#[macro_use]
mod error;

/// Wrapper around BN254 library
mod amcl;

/// Signatures implementation
mod bls;

#[cfg(feature = "ffi")]
/// Foreign function interface (C library)
mod ffi;

pub use self::bls::{
    Bls, Generator, MultiSignature, ProofOfPossession, SignKey, Signature, VerKey,
};
pub use self::error::{Error, Result};
