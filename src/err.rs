use crate::chunk::{BlockGroup, Chunk};

use core::fmt::{Formatter, Display, Result as FmtResult};
use core::error::Error;





#[non_exhaustive]
#[derive(PartialEq, Clone, Debug, Eq)]
pub enum ChunkReadError {
	BrokenIdRunlengthPair(usize),
	TooMuchData {
		/// The last blockgroup in the sequence (as it was provided).
		last_group:BlockGroup,
		/// How many additional blocks
		excess:usize
	}
}

impl Display for ChunkReadError {
	#[allow(deprecated)]
	fn fmt(&self, f:&mut Formatter) -> FmtResult {
		write!(f, "{}: {}", self.description(), match self {
			Self::BrokenIdRunlengthPair(b) => format!("expected an even number of bytes, but found an odd amount ({b})"),
			Self::TooMuchData {excess, ..} => format!("data for, at most, {} blocks was expected, but data for {} more block(s) was found", Chunk::HYPERVOLUME, excess)
		})
	}
}

impl Error for ChunkReadError {
	fn description(&self) -> &'static str { "a chunk-reading error occurred" }
}