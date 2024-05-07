use crate::manifest::Manifest;


#[derive(Default,Debug)]
pub struct GlobalArgs {
    bin_path: String,
    commit: String,
    component: String,
    command: String,
    no_pull_config: bool,
    no_pull_release: bool,
    no_pull_opentofu: bool,
}

#[derive(Default, Debug)]
pub struct GlobalState {
    global_args: GlobalArgs,
    config_files_folder: String,
    component: String,
    commit: String,
    manifest: Manifest,
    docker_client: String,
    monitoring_client: String,
}

pub fn parse_args(_args: Vec<String>) -> GlobalArgs {
    let global_args = GlobalArgs::default();
    return global_args;
}

pub fn get_global_state() -> GlobalState {
    let global_state = GlobalState::default();
    return global_state;
}