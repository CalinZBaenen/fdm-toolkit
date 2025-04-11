use crate::chunk::{CompressedChunk, BlockGroup};

use core::marker::PhantomData;
use core::iter::Iterator;
use core::convert::Into;
use core::ops::FnMut;
use std::boxed::Box;
use std::sync::Arc;
use core::cmp::Eq;





#[derive(Clone, Debug)]
pub struct CompressedChunkBytesIter<'a> {
	chunk:&'a CompressedChunk<'a>,
	idx:usize
}

impl<'a> CompressedChunkBytesIter<'a> {
	#[inline(always)] pub const fn new(chunk:&'a CompressedChunk) -> Self {
		Self {
			chunk,
			idx:0
		}
	}
}

impl<'a> Iterator for CompressedChunkBytesIter<'a> {
	type Item = u8;
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.idx >= self.chunk.len()*2 { return None; }
		
		let b = unsafe { <BlockGroup as Into<[u8; 2]>>::into(*self.chunk.get_unchecked(self.idx/2))[self.idx % 2] };
		self.idx += 1;
		Some(b)
	}
}



/// Parameters that can be supplied to someone filling an area with blocks.
pub struct FillParams<'a> {
	pub(crate) determiner:Box<dyn FnMut((usize, usize, usize, usize))->u8>,
	pub(crate) rect:Rect4,
	           _lt:PhantomData<&'a ()>
}

impl<'a> FillParams<'a> {
	#[inline(always)] pub fn with_determiner<F:FnMut((usize, usize, usize, usize))->u8+'static>(determiner:F, rect:Rect4) -> Self {
		Self {
			determiner: Box::new(determiner),
			rect,
			_lt: PhantomData
		}
	}
	
	pub fn dither(block_ids:&'a [u8], rect:Rect4) -> Self {
		let     block_ids = Arc::<[u8]>::from(block_ids);
		let mut n = 0;
		Self {
			determiner: Box::new(move |_| {
				n += 1;
				
				let block = block_ids[n % block_ids.len()];
				n %= usize::MAX;
				
				block
			}),
			rect,
			_lt: PhantomData
		}
	}
	
	#[inline(always)] pub fn hollow(block_id:u8, rect:Rect4) -> Self {
		Self {
			determiner: Box::new(move |loc| if rect.has_on_perimeter(loc) { block_id } else { 0 }),
			rect,
			_lt: PhantomData
		}
	}
	
	#[inline(always)] pub fn solid(block_id:u8, rect:Rect4) -> Self {
		Self {
			determiner: Box::new(move |_| block_id),
			rect,
			_lt: PhantomData
		}
	}
}



/// A (4D) rectangular area.
#[derive(PartialEq, Clone, Debug, Copy, Eq)]
#[repr(C)]
pub struct Rect4 {
	pub start:(usize, usize, usize, usize),
	pub end:(usize, usize, usize, usize)
}

impl Rect4 {
	#[inline(always)] pub const fn new(start:(usize, usize, usize, usize), end:(usize, usize, usize, usize)) -> Self { Self {start, end} }
	
	pub const fn has_on_perimeter(&self, point:(usize, usize, usize, usize)) -> bool {
		((point.0 == self.start.0 || point.0 == self.end.0) && (
			point.1 >= self.start.1 && point.1 <= self.end.1 && point.2 >= self.start.2 && point.2 <= self.end.2 &&
			point.3 >= self.start.3 && point.3 <= self.end.3
		)) || ((point.1 == self.start.1 || point.1 == self.end.1) && (
			point.0 >= self.start.0 && point.0 <= self.end.0 && point.2 >= self.start.2 && point.2 <= self.end.2 &&
			point.3 >= self.start.3 && point.3 <= self.end.3
		)) || ((point.2 == self.start.2 || point.2 == self.end.2) && (
			point.0 >= self.start.0 && point.0 <= self.end.0 && point.1 >= self.start.1 && point.1 <= self.end.1 &&
			point.3 >= self.start.3 && point.3 <= self.end.3
		)) || ((point.3 == self.start.3 || point.3 == self.end.3) && (
			point.0 >= self.start.0 && point.0 <= self.end.0 && point.1 >= self.start.1 && point.1 <= self.end.1 &&
			point.2 >= self.start.2 && point.2 <= self.end.2
		))
	}
}