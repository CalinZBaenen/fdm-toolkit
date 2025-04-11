use fdm_toolkit::chunk::{CompressedChunk, ChunkData};
use fdm_toolkit::util::{FillParams, Rect4};
use fdm_toolkit::world::Block;

use std::io::{Write, Read};
use std::fs::File;
use std::vec::Vec;





/// Tests to see if a (valid) chunk can be written to a file using [`CompressedChunk`].
#[test] fn create_and_write_filled_chunk() {
	let stone_chunk = CompressedChunk::filled_with(Block::Stone as u8);
	
	if let Ok(mut out_chunk) = File::options().truncate(true).create(true).write(true).open("test_out_chunk.bin") {
		let bytes = Vec::<u8>::from_iter(stone_chunk.iter_bytes());
		_ = out_chunk.write_all( &bytes );
	}
}



/// Tests to see if a chunk can be read from a file.
#[test] fn draw_a_house() {
	let mut bytes = Vec::new();
	if let Ok(mut in_chunk) = File::options().read(true).open("test_in_chunk.bin") {
		_ = in_chunk.read_to_end(&mut bytes);
		
		let interpreted_chunk = CompressedChunk::from_bytes(&bytes);
		assert!(interpreted_chunk.is_ok());
		
		let mut chunk = interpreted_chunk.unwrap().decompressed();
		chunk.fill(Block::Sandstone as u8, Rect4 {start: (0, 0, 0, 0), end: (7, 4, 7, 7)});
		chunk.fill_with_params( FillParams::hollow(Block::Sandstone as u8, Rect4::new((0, 4, 0, 0), (7, 8, 7, 7))) );
		
		let compressed = chunk.compress();
		bytes = compressed.iter_bytes().collect();
	}
	
	if !bytes.is_empty() {
		if let Ok(mut out_chunk) = File::options().truncate(true).create(true).write(true).open("test_reexported_chunk.bin") {
			_ = out_chunk.write_all(&bytes);
		}
	}
}