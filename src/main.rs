mod chunk_handler;
mod utils;

use chunk_handler::{chunk::locate_chunks, gen8_chunk::{read_game_info, GameInfo}, glob_chunk::read_global_entries};
use lazy_static::lazy_static;
use utils::read::read_pointer_map;
use std::{collections::HashMap, fs};

lazy_static! {
    pub static ref FILE_DATA: Vec<u8> = fs::read("data.win").expect("Cannot read data.win");
}

fn main() {
    let chunks = locate_chunks().unwrap();
    let mut game_info: GameInfo;
    let mut string_map: HashMap<u32, String>;
    let mut global_entries: Vec<u32>;

    for chunk in chunks {
        println!("Name: {} | Size: {} | Location: {}", chunk.ident, chunk.size, chunk.start);
        match chunk.ident.as_str() {
            "GEN8" => { game_info = read_game_info(chunk); },
            "STRG" => { string_map = read_pointer_map(chunk.start as usize + 8, 4); },
            "GLOB" => { global_entries = read_global_entries(chunk); }
            _ => {}
        };
    }
}
