use serde::{
	de::{Unexpected, Visitor, Error as DeserializationError},
	Deserializer, Deserialize
};
use serde_derive::{Deserialize, Serialize};

use core::fmt::{Formatter, Display, Result as FmtResult};





/// An object which can be held in a player's inventory.
pub trait Collectable:Sized {
	fn name(&self) -> &str;
	fn typ(&self)  -> CollectableType;
}





/// Test
#[doc(hidden)]
struct CollectableTypeVisitor;

impl<'de> Visitor<'de> for CollectableTypeVisitor {
	type Value = CollectableType;
	
	fn expecting(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result { f.write_str("the name of a CollectableType (variant)") }
	fn visit_str<E:DeserializationError>(self, v:&str) -> Result<Self::Value, E> {
		if v.eq_ignore_ascii_case("item") || v.eq_ignore_ascii_case("material") { return Ok(CollectableType::Item); }
		match v {
			_ if v.eq_ignore_ascii_case("block") => Ok(CollectableType::Block),
			_ if v.eq_ignore_ascii_case("tool")  => Ok(CollectableType::Tool),
			_ => Err(E::invalid_value(Unexpected::Other("unrecognized value"), &self))
		}
	}
}



/// The type of collectable that something is.
#[derive(Serialize, Clone, Debug, Copy)]
#[serde(rename_all="lowercase")]
pub enum CollectableType {
	Block,
	#[serde(rename="material")]
	Item,
	Tool
}

impl<'de> Deserialize<'de> for CollectableType {
	fn deserialize<D:Deserializer<'de>>(d:D) -> Result<Self, D::Error> { d.deserialize_enum("CollectableType", &["Block", "Item", "Tool"], CollectableTypeVisitor) }
}

impl Display for CollectableType {
	fn fmt(&self, f:&mut Formatter) -> FmtResult {
		f.write_str(match self {
			Self::Block => "block",
			Self::Item  => "material",
			Self::Tool  => "tool"
		})
	}
}



/// The standard set of items in 4D Miner.
#[non_exhaustive]
#[derive(Deserialize, Serialize, Clone, Debug, Hash, Copy)]
#[serde(rename_all="snake_case")]
pub enum Item {
	Stick,
	Hammer,
	IronPick,
	DeadlyPick,
	IronAxe,
	DeadlyAxe,
	Ultrahammer,
	SolenoidCollector,
	Rock,
	Hypersilk,
	IronBars,
	DeadlyBars,
	SolenoidBars,
	Compass,
	Glasses,
	KleinBottle,
	HealthPotion,
	RedLens,
	GreenLens,
	BlueLens,
	Alidade
}

impl Item {
	#[inline(always)] pub const fn as_str(&self) -> &'static str {
		match self {
			Self::SolenoidCollector => "Solenoid Collector",
			Self::SolenoidBars      => "Solenoid Bars",
			Self::HealthPotion      => "Health Potion",
			Self::KleinBottle       => "Klein Bottle",
			Self::Ultrahammer       => "Ultrahammer",
			Self::DeadlyBars        => "Deadly Bars",
			Self::DeadlyPick        => "Deadly Pick",
			Self::DeadlyAxe         => "Deadly Axe",
			Self::GreenLens         => "Green Lens",
			Self::Hypersilk         => "Hypersilk",
			Self::BlueLens          => "Blue Lens",
			Self::IronBars          => "Iron Bars",
			Self::IronPick          => "Iron Pick",
			Self::Alidade           => "Alidade",
			Self::Compass           => "Compass",
			Self::Glasses           => "4D Glasses",
			Self::IronAxe           => "Iron Axe",
			Self::RedLens           => "Red Lens",
			Self::Hammer            => "Hammer",
			Self::Stick             => "Stick",
			Self::Rock              => "Rock"
		}
	}
}

impl Collectable for Item {
	fn name(&self) -> &str { self.as_str() }
	fn typ(&self)  -> CollectableType {
		match self {
			Self::SolenoidCollector => CollectableType::Tool,
			Self::SolenoidBars      => CollectableType::Item,
			Self::HealthPotion      => CollectableType::Item,
			Self::KleinBottle       => CollectableType::Item,
			Self::Ultrahammer       => CollectableType::Tool,
			Self::DeadlyBars        => CollectableType::Item,
			Self::DeadlyPick        => CollectableType::Tool,
			Self::DeadlyAxe         => CollectableType::Tool,
			Self::GreenLens         => CollectableType::Item,
			Self::Hypersilk         => CollectableType::Item,
			Self::BlueLens          => CollectableType::Item,
			Self::IronBars          => CollectableType::Item,
			Self::IronPick          => CollectableType::Tool,
			Self::Alidade           => CollectableType::Item,
			Self::Compass           => CollectableType::Item,
			Self::Glasses           => CollectableType::Item,
			Self::IronAxe           => CollectableType::Tool,
			Self::RedLens           => CollectableType::Item,
			Self::Hammer            => CollectableType::Tool,
			Self::Stick             => CollectableType::Tool,
			Self::Rock              => CollectableType::Item
		}
	}
}

impl Display for Item {
	fn fmt(&self, f:&mut Formatter) -> FmtResult { f.write_str(self.as_str()) }
}