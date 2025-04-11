use crate::collectable::{CollectableType, Collectable};
use crate::chunk::Chunk;

use serde_derive::{Deserialize, Serialize};

use core::fmt::{Formatter, Display, Result as FmtResult};
use std::collections::HashMap;
use core::default::Default;
use core::cmp::PartialEq;
use core::convert::Into;





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
	/// Stone – a generic block of (cobbled) stone which can be found under Dirt.
	Stone,
	/// Wood – a generic block to be used as a tree-trunk in non-Midnight biomes.
	Wood,
	///
	Leaf,
	/// Lava – a (purely decorative) placeholder block found at the bottom of the world.
	Lava,
	///
	IronOre,
	/// Deadly Ore – a glowing ore which produces the most valuable resource, Deadly Bars.
	DeadlyOre,
	/// Chest – a block that can store items.
	Chest,
	/// Midnight Grass – the Midnight biome's variant of [`Block::Grass`].
	MidnightGrass,
	///
	MidnightSoil,
	///
	MidnightStone,
	///
	MidnightWood,
	///
	MidnightLeaf,
	///
	Bush,
	///
	MidnightBush,
	/// A generic red flower.
	RedFlower,
	/// A generic white flower.
	WhiteFlower,
	/// A generic blue flower.
	BlueFlower,
	///
	TallGrass,
	/// Sand – a generic block of sand, used as the floor for the Desert biome.
	Sand,
	/// Sandstone – a generic block of (cobbled) sandstone which can be found under Sand.
	Sandstone,
	///
	Cactus,
	///
	Snow,
	///
	Ice,
	/// Snowy Bush – the Snow biome's variant of [`Block::Bush`].
	SnowyBush,
	/// Glass – a generic, see-through, block of glass crafted from Sand and Wood.
	Glass,
	///
	SolenoidOre,
	///
	SnowyLeaf,
	/// Pumpkin – a naturally, but infrequently, occurring block in grasslands.
	Pumpkin,
	///
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

impl PartialEq<u8> for Block {
	fn eq(&self, other:&u8) -> bool { *self as u8 == *other }
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



/// A world.
#[derive(PartialEq, Clone, Debug, Eq)]
#[repr(transparent)]
pub struct World(HashMap<(i64, i64, i64), Chunk>);

impl World {
	/// The size of the (whole) world along the Y axis.
	pub const HEIGHT:usize = 128;
}





impl PartialEq<Block> for u8 {
	fn eq(&self, other:&Block) -> bool { *self == *other as u8 }
}