use fdm_toolkit::world::{Block, Chunk};

use std::io::{Write, Read};
use std::fs::File;
use std::vec::Vec;





/// Tests to see if a (valid) chunk can be written to a file.
#[test] fn create_and_write_filled_chunk() {
	let stone_chunk = Chunk::filled_with(Block::Stone);
	println!("{stone_chunk}");
	
	if let Ok(mut out_chunk) = File::options().truncate(true).create(true).write(true).open("test_out_chunk.bin") {
		_ = out_chunk.write(stone_chunk.bytes());
	}
}



/// Tests to see if a chunk can be read from a file.
#[test] fn read_chunk_from_file() {
	if let Ok(mut in_chunk) = File::options().read(true).open("test_in_chunk.bin") {
		let mut bytes = Vec::new();
		_ = in_chunk.read_to_end(&mut bytes);
		
		let interpreted_chunk = Chunk::from_bytes(&bytes);
		println!("{interpreted_chunk:?}");
	}
}