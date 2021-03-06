//! `Ipld` error definitions.
pub use libipld_core::error::*;
use thiserror::Error;

/// Result alias.
pub type Result<T> = core::result::Result<T, Error>;

/// Ipld error.
#[derive(Debug, Error)]
pub enum Error {
    /// Block exceeds MAX_BLOCK_SIZE.
    #[error("Block size {0} exceeds MAX_BLOCK_SIZE.")]
    BlockTooLarge(usize),
    /// Hash does not match the CID.
    #[error("Hash does not match the CID.")]
    // It is a Multihash, but it's already converted into bytes when reported, so that we
    // don't need to bubble up all the generics into `Error`
    InvalidHash(Vec<u8>),
    /// The codec is unsupported.
    #[error("Unsupported codec {0:?}.")]
    UnsupportedCodec(u64),
    /// The multihash is unsupported.
    #[error("Unsupported multihash {0:?}.")]
    UnsupportedMultihash(crate::multihash::Code),
    /// Type error.
    #[error("{0}")]
    TypeError(#[from] TypeError),
    /// The codec returned an error.
    #[error("Codec error: {0}")]
    CodecError(Box<dyn std::error::Error + Send>),
    /// The store returned an error.
    #[error("{0}")]
    StoreError(#[from] StoreError),
}

/// Store error.
#[derive(Debug, Error)]
pub enum StoreError {
    /// The block wasn't found. The supplied string is a CID.
    #[error("failed to retrive block {0}")]
    // The string is an actual CID. Transform the CID into a string so that the generics don't
    // bleed into this error enum.
    BlockNotFound(String),
    /// The batch was empty.
    #[error("empty batch")]
    EmptyBatch,
    /// Other failure.
    #[error("{0}")]
    Other(#[from] Box<dyn std::error::Error + Send>),
}
