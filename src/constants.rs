//! Constants for blockchain runtime operations

/// Default timeout for runtime operations in seconds
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 300;

/// Default memory limit in MB
pub const DEFAULT_MEMORY_LIMIT_MB: u64 = 1024;

/// Default maximum call depth for security
pub const DEFAULT_MAX_CALL_DEPTH: u32 = 1024;

/// Default maximum external calls per transaction
pub const DEFAULT_MAX_EXTERNAL_CALLS: u32 = 100;

/// Default maximum gas limit
pub const DEFAULT_MAX_GAS_LIMIT: u64 = 10_000_000;

/// Default maximum memory usage in bytes (100MB)
pub const DEFAULT_MAX_MEMORY_BYTES: u64 = 100 * 1024 * 1024;

/// Maximum execution time for runtime capabilities in seconds
pub const DEFAULT_MAX_EXECUTION_TIME_SECONDS: u64 = 300;

/// Maximum path length for security validation
pub const MAX_PATH_LENGTH: usize = 4096;

/// Maximum symlink chain length to prevent infinite loops
pub const MAX_SYMLINK_CHAIN_LENGTH: usize = 100;
