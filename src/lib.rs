//! `fdm-toolkit` is a crate for reading and writing 4D Miner (0.2.1.4 Alpha) game-data.

/// Data-types and functionality for working with [`Collectable`] items.
pub mod collectable;
/// Data-types and functionality for working with 4D Miner chunk-data.
pub mod chunk;
/// Data-types and functionality for handling [`World`]-wide data.
pub mod world;
/// Utilities for supported functionality.
pub mod util;
/// Error-types for `fdm-toolkit`.
pub mod err;