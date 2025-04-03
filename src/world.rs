use crate::collectable::{CollectableType, Collectable};
use crate::err::ChunkReadError;

use serde_derive::{Deserialize, Serialize};

use core::fmt::{Formatter, Display, Result as FmtResult};
use core::convert::{AsRef, From, Into};
use std::collections::HashMap;
use core::default::Default;
use core::borrow::Borrow;
use core::mem::transmute;
use core::ops::Deref;
use std::vec::Vec;





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



#[non_exhaustive]
#[derive(Deserialize, PartialEq, Serialize, Clone, Debug, Copy, Hash, Eq)]
#[serde(rename_all="snake_case")]
#[repr(u8)]
pub enum Block {
	/// Air.
	Air,
	/// Grass – a generic block of soil with green grass on top, used as the floor for most of the world.
	Grass,
	/// Dirt – a generic block of soil which can be found under Grass.
	Dirt,
	Stone,
	Wood,
	Leaf,
	Lava,
	IronOre,
	DeadlyOre,
	/// Chest – a block that can store items.
	Chest,
	/// Midnight Grass – the Midnight biome's version of Grass.
	MidnightGrass,
	MidnightSoil,
	MidnightStone,
	MidnightWood,
	MidnightLeaf,
	Bush,
	MidnightBush,
	RedFlower,
	WhiteFlower,
	BlueFlower,
	TallGrass,
	Sand,
	Sandstone,
	Cactus,
	Snow,
	Ice,
	SnowyBush,
	Glass,
	SolenoidOre,
	SnowyLeaf,
	Pumpkin,
	JackOLantern,
	/// Barrier – a special block which, presumably, represents an impassible block
	Barrier,
	/// A special block which represents the border of a chunk.
	ChunkBorder
}

impl Block {
	#[inline(always)] pub const fn as_str(&self) -> &'static str {
		match self {
			Self::MidnightGrass => "Midnight Grass",
			Self::MidnightStone => "Midnight Stone",
			Self::JackOLantern  => "Jack o'Lantern",
			Self::MidnightBush  => "Midnight Bush",
			Self::MidnightLeaf  => "Midnight Leaf",
			Self::MidnightSoil  => "Midnight Soil",
			Self::MidnightWood  => "Midnight Wood",
			Self::SolenoidOre   => "Solenoid Ore",
			Self::WhiteFlower   => "White Flower",
			Self::BlueFlower    => "Blue Flower",
			Self::DeadlyOre     => "Deadly Ore",
			Self::RedFlower     => "Red Flower",
			Self::Sandstone     => "Sandstone",
			Self::SnowyBush     => "Snowy Bush",
			Self::SnowyLeaf     => "Snowy Leaf",
			Self::TallGrass     => "Tall Grass",
			Self::Barrier       => "Barrier",
			Self::IronOre       => "Iron Ore",
			Self::Pumpkin       => "Pumpkin",
			Self::Cactus        => "Cactus",
			Self::Chest         => "Chest",
			Self::Glass         => "Glass",
			Self::Grass         => "Grass",
			Self::Stone         => "Stone",
			Self::Bush          => "Bush",
			Self::Dirt          => "Dirt",
			Self::Lava          => "Lava",
			Self::Leaf          => "Leaf",
			Self::Sand          => "Sand",
			Self::Snow          => "Snow",
			Self::Wood          => "Wood",
			Self::Air           => "Air",
			Self::Ice           => "Ice",
			
			_ => ""
		}
	}
}

impl Collectable for Block {
	fn name(&self) -> &str { self.as_str() }
	#[inline(always)] fn typ(&self)  -> CollectableType { CollectableType::Block }
}

impl Default for Block {
	#[inline(always)] fn default() -> Self { Self::Air }
}

impl Display for Block {
	fn fmt(&self, f:&mut Formatter) -> FmtResult { f.write_str(self.as_str()) }
}

impl Into<u8> for Block {
	fn into(self) -> u8 { self as u8 }
}



/// Represents a 8*128*8*8 chunk of the world.
#[derive(PartialEq, Default, Clone, Debug, Hash, Eq)]
#[repr(transparent)]
pub struct Chunk(Vec<BlockGroup>);

impl Chunk {
		/// The hypervolume of a chunk.
	pub const HYPERVOLUME:usize = Self::WIDTH*World::HEIGHT*Self::LENGTH*Self::WETH;
	/// The size of a chunk along the Z axis,
	pub const LENGTH:usize      = 8;
	/// The size of a chunk along the X axis.
	pub const WIDTH:usize       = 8;
	/// The size of a chunk along the W axis.
	pub const WETH:usize        = 8;
	
	/// Creates a chunk filled entirely with the specified block.
	pub fn filled_with(b:Block) -> Self {
		let mut times = 0;
		let mut left  = Self::HYPERVOLUME;
		let     add   = u8::MAX;
		let mut r     = 0;
		
		while left > 0 {
			let sub = if add as usize > left { r = left; left } else { times += 1; add as usize };
			left -= sub;
		}
		
		let mut chunk = Vec::with_capacity(times + if r > 0 { 1 } else { 0 });
		while times > 0 {
			chunk.push(BlockGroup {block_id: b as u8, span: add});
			times -= 1;
		}
		if r > 0 { chunk.push(BlockGroup {block_id: b as u8, span: r as u8}); }
		
		Self(chunk)
	}
	
	/// Attempts to convert a contiguous sequence of bytes to a [`Chunk`].
	pub fn from_bytes(b:&[u8]) -> Result<Self, ChunkReadError> {
		let l = b.len();
		if l % 2 != 0 { return Err(ChunkReadError::BrokenIdRunlengthPair(l)); }
		
		let mut bytes = b.into_iter().cloned();
		let mut chunk = Vec::with_capacity(l/2);
		let mut idx   = 0;
		while idx < Self::HYPERVOLUME && 2*idx < l {
			let block_id = bytes.next().unwrap_or_default();
			let span     = bytes.next().unwrap_or_default();
			
			let group = BlockGroup {block_id, span};
			let nidx  = idx+span as usize;
			
			if nidx >= Self::HYPERVOLUME {
				chunk.push(BlockGroup {block_id, span: (span as usize-(nidx-Self::HYPERVOLUME) & 255) as u8});
				chunk.shrink_to_fit();
				
				return Err(ChunkReadError::TooMuchData {
					theoretical_index: nidx,
					last_group: group,
					chunk: Self(chunk)
				});
			}
			
			chunk.push(group);
			idx = nidx;
		}
		
		chunk.shrink_to_fit();
		Ok( Self(chunk) )
	}
	
	/// Returns a new [`Chunk`] without any block-data.
	#[inline(always)] pub const fn new_empty() -> Self { Self(Vec::new()) }
	
	/// The count of how many blocks there are in the current chunk.
	pub fn block_count(&self) -> usize {
		let mut ct = 0;
		for group in &self.0 { ct += group.span as usize; }
		ct
	}
	
	/// Returns this [`Chunk`] as a stream of bytes.
	pub fn bytes(&self) -> &[u8] { unsafe { transmute(self.0.as_slice()) } }
}

impl Display for Chunk {
	fn fmt(&self, f:&mut Formatter) -> FmtResult {
		for group in &self.0 { _ = group.fmt(f); }
		Ok(())
	}
}

impl Borrow<Vec<BlockGroup>> for Chunk {
	fn borrow(&self) -> &Vec<BlockGroup> { &self.0 }
}

impl AsRef<Vec<BlockGroup>> for Chunk {
	fn as_ref(&self) -> &Vec<BlockGroup> { &self.0 }
}

impl Borrow<[BlockGroup]> for Chunk {
	fn borrow(&self) -> &[BlockGroup] { &self.0 }
}

impl AsRef<[BlockGroup]> for Chunk {
	fn as_ref(&self) -> &[BlockGroup] { &self.0 }
}

impl Deref for Chunk {
	type Target = [BlockGroup];
	
	fn deref(&self) -> &Self::Target { &self.0 }
}



#[derive(PartialEq, Clone, Debug, Eq)]
#[repr(transparent)]
pub struct World(HashMap<(i64, i64, i64), Chunk>);

impl World {
	/// The size of the (whole) world along the Y axis.
	pub const HEIGHT:usize = 128;
}