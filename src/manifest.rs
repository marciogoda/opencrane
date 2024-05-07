use std::collections::HashMap;
#[derive(Debug,Default)]
pub struct Manifest {
    version: i8,
    config_files_folder: String,
    config: ImageWithParams,
    builds: HashMap<String,String>,
    opentofu: Opentofu,
}

#[derive(Debug,Default)]
struct ImageWithParams {
    image: String,
    params: HashMap<String,String>,
}

#[derive(Debug,Default)]
struct Opentofu {
    image: String,
}

fn load(path: String) -> Manifest {
    let manifest = Manifest::default();
    return manifest;
}
