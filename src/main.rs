mod data_loader;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let default_name = "data.win".to_owned();
    let program_name = args.get(1).unwrap_or(&default_name);

    env_logger::init();
    let file_data: Vec<u8> =
        fs::read(program_name.clone()).expect(&format!("Cannot read {}", program_name));

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
        println!("{:#?}", data.vari.variables[2]);
        println!("{:#?}", data.code.code_entries.get(150));
        // println!("{:#?}", data.scpt);
        // for (k, v) in data.strg.string_map.into_iter() {
        //     println!("{}: \"{}\"", k, v)
        // }
    } else if let Err(error) = load_res {
        println!("{}", error);
    }
}
