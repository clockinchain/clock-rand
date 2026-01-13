//! Error types for clock-rand

/// Error type for all clock-rand operations
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Seed-related error
    Seed(SeedError),
    /// Entropy-related error
    Entropy(EntropyError),
    /// Fork detection error
    Fork(ForkError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Seed(e) => write!(f, "Seed error: {}", e),
            Error::Entropy(e) => write!(f, "Entropy error: {}", e),
            Error::Fork(e) => write!(f, "Fork error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Seed-related errors
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SeedError {
    /// Invalid seed size
    InvalidSize {
        /// Expected size in bytes
        expected: usize,
        /// Actual size received
        got: usize,
    },
    /// Seed contains all zeros (weak seed)
    AllZeros,
    /// Seed validation failed
    ValidationFailed(&'static str),
}

impl core::fmt::Display for SeedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SeedError::InvalidSize { expected, got } => {
                write!(f, "Invalid seed size: expected {}, got {}", expected, got)
            }
            SeedError::AllZeros => write!(f, "Seed contains all zeros (weak seed)"),
            SeedError::ValidationFailed(msg) => write!(f, "Seed validation failed: {}", msg),
        }
    }
}

/// Entropy-related errors
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EntropyError {
    /// Insufficient entropy available
    InsufficientEntropy,
    /// Failed to read from entropy source
    ReadFailed(&'static str),
    /// Entropy source unavailable
    SourceUnavailable,
}

impl core::fmt::Display for EntropyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EntropyError::InsufficientEntropy => write!(f, "Insufficient entropy available"),
            EntropyError::ReadFailed(msg) => write!(f, "Failed to read entropy: {}", msg),
            EntropyError::SourceUnavailable => write!(f, "Entropy source unavailable"),
        }
    }
}

/// Fork detection errors
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ForkError {
    /// Fork detected but reseeding failed
    ReseedFailed(&'static str),
    /// Invalid block hash format
    InvalidBlockHash(&'static str),
    /// Fork detection state corrupted
    StateCorrupted,
}

impl core::fmt::Display for ForkError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ForkError::ReseedFailed(msg) => write!(f, "Reseed failed after fork: {}", msg),
            ForkError::InvalidBlockHash(msg) => write!(f, "Invalid block hash: {}", msg),
            ForkError::StateCorrupted => write!(f, "Fork detection state corrupted"),
        }
    }
}

/// Result type alias for clock-rand operations
pub type Result<T> = core::result::Result<T, Error>;

impl From<SeedError> for Error {
    fn from(err: SeedError) -> Self {
        Error::Seed(err)
    }
}

impl From<EntropyError> for Error {
    fn from(err: EntropyError) -> Self {
        Error::Entropy(err)
    }
}

impl From<ForkError> for Error {
    fn from(err: ForkError) -> Self {
        Error::Fork(err)
    }
}
