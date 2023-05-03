//! BLS signatures for Indy

#![deny(missing_debug_implementations, rust_2018_idioms, unsafe_code)]
#![warn(missing_docs)]

/// Wrapper around BN254 library
mod amcl;

/// Signatures implementation
mod bls;

/// Error type
mod error;

pub use self::bls::{
    Bls, Generator, MultiSignature, ProofOfPossession, SignKey, Signature, VerKey,
};
pub use self::error::{Error, Result};
