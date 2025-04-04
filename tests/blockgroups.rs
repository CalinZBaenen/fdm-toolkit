use fdm_toolkit::world::{Block, Chunk};

use std::io::{Write, Read};
use std::fs::File;
use std::vec::Vec;





/// Tests to see if a (valid) chunk can be written to a file.
#[test] fn create_and_write_filled_chunk() {
	let stone_chunk = Chunk::filled_with(Block::Stone);
	println!("{stone_chunk:?}");
	
	if let Ok(mut out_chunk) = File::options().truncate(true).create(true).write(true).open("test_out_chunk.bin") {
		_ = out_chunk.write(stone_chunk.bytes());
	}
}



/// Tests to see if a chunk can be read from a file.
#[test] fn read_and_reexport_chunk() {
	let mut bytes = Vec::new();
	if let Ok(mut in_chunk) = File::options().read(true).open("test_in_chunk.bin") {
		_ = in_chunk.read_to_end(&mut bytes);
		
		let mut interpreted_chunk = Chunk::from_bytes(&bytes);
		println!("{interpreted_chunk:?}");
		
		if let Ok(interpreted_chunk) = &mut interpreted_chunk {
			for bg in &mut *interpreted_chunk {
				if bg.block_id == Block::Stone { bg.block_id = Block::Sandstone as u8; }
			}
			
			bytes = interpreted_chunk.bytes().into();
		}
	}
	
	if !bytes.is_empty() {
		if let Ok(mut out_chunk) = File::options().truncate(true).create(true).write(true).open("test_reexported_chunk.bin") {
			_ = out_chunk.write(&bytes);
		}
	}
}