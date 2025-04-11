use crate::util::{CompressedChunkBytesIter, FillParams, Rect4};
use crate::err::ChunkReadError;
use crate::world::World;

use core::fmt::{Formatter, Display, Result as FmtResult, Debug};
use core::convert::{AsRef, From, Into};
use core::marker::PhantomData;
use core::iter::IntoIterator;
use core::default::Default;
use core::borrow::Borrow;
use core::mem::transmute;
use core::ops::Deref;
use std::sync::Arc;
use std::vec::Vec;





pub trait ChunkData {
	/// Deserializes a slice of bytes into chunk-data.
	fn from_bytes(bytes:&[u8]) -> Result<Self, ChunkReadError> where Self:Sized;
	
	/// Generate the equivalent decompressed chunk-data.
	fn decompressed(&self)                                  -> Chunk;
	/// (Try to) Get the ID of a block at a given coordinate in 4D space.
	fn get_block(&self, loc:(usize, usize, usize, usize)) -> Option<u8>;
}





/// Represents a 8*128*8*8 chunk of the world.
#[derive(PartialEq, Clone, Hash, Eq)]
#[repr(transparent)]
pub struct CompressedChunk<'a>(Arc<[BlockGroup]>, PhantomData<&'a ()>);

impl<'a> CompressedChunk<'a> {
	/// Creates a new [`CompressedChunk`] filled entirely with a block of the specified ID.
	pub fn filled_with(block_id:u8) -> Self { Self(Arc::new([]), PhantomData).with_remaining_filled(block_id) }
	
	/// Fills the remaining (uninitialized) space in the chunk with a block of the specified ID.
	pub fn with_remaining_filled(mut self, block_id:u8) -> Self {
		let mut remaining = Chunk::HYPERVOLUME;
		let mut vec       = Vec::new();
		
		for group in &*self.0 {
			remaining -= group.span as usize;
			vec.push(*group);
		}
		
		while remaining > 0 {
			let span = if remaining < 255 { let r = remaining as u8; remaining = 0; r }
			           else               { remaining -= 255; 255 };
			vec.push(BlockGroup {block_id, span});
		}
		
		self.0 = vec.into();
		self
	}
	
	/// Creates a new [`CompressedChunk`] from anything that can be turned into a [`CompressedChunk`],
	///  via [`Into`].
	/// 
	/// The benefit of using this method, over [`CompressedChunk::from`], is that any empty space remaining
	///  in the chunk-data is filled with air before the [`CompressedChunk`] is returned.
	pub fn new<T:Into<Self>>(v:T) -> Self { v.into().with_remaining_filled(0) }
	
	/// Returns an iterator over the bytes in this [`CompressedChunk`].
	pub const fn iter_bytes(&self) -> CompressedChunkBytesIter { CompressedChunkBytesIter::new(self) }
}

impl<'a> IntoIterator for &'a CompressedChunk<'a> {
	type IntoIter = ::core::slice::Iter<'a, BlockGroup>;
	type Item     = &'a BlockGroup;
	
	fn into_iter(self) -> Self::IntoIter { (&self.0).into_iter() }
}

impl<'a> ChunkData for CompressedChunk<'a> {
	/// Attempts to convert byte-sequence representing one or more [`BlockGroup`]s into a [`CompressedChunk`].
	fn from_bytes(bytes:&[u8]) -> Result<Self, ChunkReadError> {
		let l = bytes.len();
		if l % 2 != 0 { return Err(ChunkReadError::BrokenIdRunlengthPair(l)); }
		
		let mut ct:usize = 0;
		
		Ok(Self(
			bytes.chunks_exact(2)
				.map(|pair| {
					let pair = match pair {
						&[block_id, span] => BlockGroup {block_id, span},
						_ => unreachable!()
					};
					
					ct += pair.span as usize;
					if ct > Chunk::HYPERVOLUME {
						return Err(ChunkReadError::TooMuchData {
							last_group: pair,
							excess: ct-Chunk::HYPERVOLUME
						});
					}
					Ok(pair)
				}).collect::<Result<Arc<[BlockGroup]>, _>>()?,
				PhantomData
		).with_remaining_filled(0))
	}
	
	fn decompressed(&self) -> Chunk {
		let mut chunk = Chunk::filled_with(0);
		
		let mut pos   = 0;
		for group in &*self {
			let mut n = group.span;
			while n > 0 {
				chunk.0[pos / 8192][(pos/64) % 128][(pos/8) % 8][pos % 8] = group.block_id;
				
				pos += 1;
				n -= 1;
			}
		}
		
		chunk
	}
	
	fn get_block(&self, _loc:(usize, usize, usize, usize)) -> Option<u8> { todo!() }
}

impl<'a> Default for CompressedChunk<'a> {
	fn default() -> Self { Self::filled_with(0) }
}

impl<'a> Borrow<Arc<[BlockGroup]>> for CompressedChunk<'a> {
	fn borrow(&self) -> &Arc<[BlockGroup]> { &self.0 }
}

impl<'a> AsRef<Arc<[BlockGroup]>> for CompressedChunk<'a> {
	fn as_ref(&self) -> &Arc<[BlockGroup]> { &self.0 }
}

impl<'a> Borrow<[BlockGroup]> for CompressedChunk<'a> {
	fn borrow(&self) -> &[BlockGroup] { &self.0 }
}

impl<'a> AsRef<[BlockGroup]> for CompressedChunk<'a> {
	fn as_ref(&self) -> &[BlockGroup] { &self.0 }
}

impl<'a> Debug for CompressedChunk<'a> {
	fn fmt(&self, f:&mut Formatter) -> FmtResult {
		for group in &*self.0 { _ = <BlockGroup as Display>::fmt(group, f); }
		Ok(())
	}
}

impl<'a> Deref for CompressedChunk<'a> {
	type Target = [BlockGroup];
	
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'a> From<Arc<[BlockGroup]>> for CompressedChunk<'a> {
	fn from(v:Arc<[BlockGroup]>) -> Self { Self(v, PhantomData) }
}

impl<'a> From<Vec<BlockGroup>> for CompressedChunk<'a> {
	fn from(v:Vec<BlockGroup>) -> Self { Self(v.into(), PhantomData) }
}



/// A group of blocks in a chunk.  
///  (A block ID and length pair.)
#[derive(PartialEq, Default, Clone, Debug, Hash, Copy, Eq)]
#[repr(C)]
pub struct BlockGroup {
	pub block_id:u8,
	pub span:u8
}

impl Display for BlockGroup {
	fn fmt(&self, f:&mut Formatter) -> FmtResult {
		write!(f, "[{} * block#{}]", self.span, self.block_id)
	}
}

impl From<[u8; 2]> for BlockGroup {
	fn from(v:[u8; 2]) -> Self { unsafe { transmute(v) } }
}

impl Into<[u8; 2]> for BlockGroup {
	fn into(self) -> [u8; 2] { unsafe { transmute(self) } }
}



/// Uncompressed chunk-data.
#[derive(PartialEq, Clone, Debug, Hash, Eq)]
#[repr(transparent)]
pub struct Chunk([[[[u8; Self::WETH]; Self::LENGTH]; World::HEIGHT]; Self::WIDTH]);

impl Chunk {
	/// The hypervolume of a chunk.
	pub const HYPERVOLUME:usize = Self::WIDTH*World::HEIGHT*Self::LENGTH*Self::WETH;
	/// The size of a chunk along the Z axis,
	pub const LENGTH:usize      = 8;
	/// The size of a chunk along the X axis.
	pub const WIDTH:usize       = 8;
	/// The size of a chunk along the W axis.
	pub const WETH:usize        = 8;
	
	/// Creates a new [`Chunk`] filled entirely with a block of the specified ID.
	pub const fn filled_with(block_id:u8) -> Self { Self([[[[block_id; Self::WETH]; Self::LENGTH]; World::HEIGHT]; Self::WIDTH]) }
	
	pub fn compress(&self) -> CompressedChunk {
		let mut cbt_ct = 0;
		let mut data   = Vec::new();
		let mut cbt    = None;
		
		let mut pos = 0;
		loop {
			if pos >= Self::HYPERVOLUME { break; }
			
			let id = self.0[pos / 8192][(pos/64) % 128][(pos/8) % 8][pos % 8];
			match cbt {
				Some(pbt) if pbt == id => {
					cbt_ct += 1;
					if cbt_ct == 255 || pos == Self::HYPERVOLUME-1 {
						data.push(BlockGroup {block_id: id, span: cbt_ct});
						cbt_ct = 0;
					}
				}
				
				Some(_) | None => {
					if cbt_ct > 0 {
						data.push(BlockGroup {block_id: unsafe { cbt.unwrap_unchecked() }, span: cbt_ct});
					}
					
					cbt_ct = 1;
					cbt = Some(id);
				}
			}
			
			pos += 1;
		}
		
		CompressedChunk(data.into(), PhantomData)
	}
	
	/// Fills the specified area from point `a` to point `b` with a block of the specified ID.
	pub fn fill_with_params(&mut self, mut fill_params:FillParams) {
		#[inline(always)] const fn sort(a:usize, b:usize) -> (usize, usize) { if a > b { (b, a) } else { (a, b) } }
		
		let (sx, dx) = sort(fill_params.rect.start.0, fill_params.rect.end.0);
		let (sy, dy) = sort(fill_params.rect.start.1, fill_params.rect.end.1);
		let (sz, dz) = sort(fill_params.rect.start.2, fill_params.rect.end.2);
		let (sw, dw) = sort(fill_params.rect.start.3, fill_params.rect.end.3);
		
		let mut x = sx;
		loop {
			let mut y = sy;
			loop {
				let mut z = sz;
				loop {
					let mut w = sw;
					loop {
						self.0[x][y][z][w] = fill_params.determiner.as_mut()((x, y, z, w));
						
						if w >= dw { break; }
						w += 1;
					}
					
					if z >= dz { break; }
					z += 1
				}
				
				if y >= dy { break; }
				y += 1;
			}
			
			if x >= dx { break; }
			x += 1;
		}
	}
	
	pub fn fill(&mut self, block_id:u8, rect:Rect4) {
		self.fill_with_params(FillParams::solid(block_id, rect));
	}
}

impl ChunkData for Chunk {
	/// Generate the equivalent decompressed chunk-data.
	/// 
	/// Because the data stored in this struct is already decompressed,
	///  this method just returns a clone of the current [`Chunk`] object.
	fn decompressed(&self) -> Chunk { self.clone() }
	
	fn from_bytes(bytes:&[u8]) -> Result<Self, ChunkReadError> where Self:Sized { Ok(CompressedChunk::from_bytes(bytes)?.decompressed()) }
	
	fn get_block(&self, loc:(usize, usize, usize, usize)) -> Option<u8> { self.0.get(loc.0)?.get(loc.1)?.get(loc.2)?.get(loc.3).cloned() }
}

impl Default for Chunk {
	fn default() -> Self { Self::filled_with(0) }
}