mod data_loader;

use std::fs;

fn main() {
    env_logger::init();
    let file_data: Vec<u8> = fs::read("data.win").expect("Cannot read data.win");

    let load_res = data_loader::form::deserialize_form(&file_data);

    if let Ok(data) = load_res {
        println!("Size: {}", data.size);
        println!("GEN8 Size: {}", data.gen8.size);
        println!("Filename: {}", data.gen8.filename);
        println!(
            "Version: {}.{} (build: {}, release: {}, format: {})",
            data.gen8.version_info.major,
            data.gen8.version_info.minor,
            data.gen8.version_info.build,
            data.gen8.version_info.release,
            data.gen8.version_info.format,
        );
        println!("{:#?}", data.agrp);
        // println!("{:#?}", data.scpt);
        // for (k, v) in data.strg.string_map.into_iter() {
        //     println!("{}: \"{}\"", k, v)
        // }
    } else if let Err(error) = load_res {
        println!("{}", error);
    }
}
