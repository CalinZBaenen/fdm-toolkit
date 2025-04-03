use crate::world::{BlockGroup, Chunk};

use core::fmt::{Formatter, Display, Result as FmtResult};
use core::error::Error;





#[non_exhaustive]
#[derive(PartialEq, Clone, Debug, Eq)]
pub enum ChunkReadError {
	BrokenIdRunlengthPair(usize),
	TooMuchData {
		/// What the block index would be if the last blockgroup was fully formed.
		theoretical_index:usize,
		/// The last blockgroup in the sequence (as it was provided).
		last_group:BlockGroup,
		/// The chunk that was created in the process of reading the chunk.
		chunk:Chunk
	}
}

impl Display for ChunkReadError {
	#[allow(deprecated)]
	fn fmt(&self, f:&mut Formatter) -> FmtResult {
		write!(f, "{}: {}", self.description(), match self {
			Self::BrokenIdRunlengthPair(b)               => format!("expected an even number of bytes, but found an odd amount ({b})"),
			Self::TooMuchData {theoretical_index: b, ..} => format!("data for, at most, {} blocks was expected, but data for {} blocks was found", Chunk::HYPERVOLUME, b+1)
		})
	}
}

impl Error for ChunkReadError {
	fn description(&self) -> &'static str { "a chunk-reading error occurred" }
}