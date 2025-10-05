mod data_loader;

use std::fs;

fn main() {
    env_logger::init();
    let FILE_DATA: Vec<u8> = fs::read("data.win").expect("Cannot read data.win");

    let load_res = data_loader::form::deserialize_form(&FILE_DATA);

    if let Ok(data) = load_res {
        println!("Size: {}", data.size);
        println!("GEN8 Size: {}", data.gen8.size);
        println!("Filename: {}", data.gen8.filename);
        // println!("FPS: {}", data.gen8.gms2_data.unwrap().fps);
        println!("{:#?}", data.gen8);
    } else if let Err(error) = load_res {
        println!("{}", error);
    }
}
