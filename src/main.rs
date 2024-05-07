
use opencrane::commands::{get_global_state, parse_args};
use std::env;

//static OPENCRANE_CONFIG_FILE: &str = "opencrane.yaml";


fn run_command() -> i8 {
    let args: Vec<String> = env::args().collect();
    let global_args = parse_args(args);
    let global_state = get_global_state();
    println!("{:?}", global_args);
    println!("{:?}", global_state);
    return 0
}

fn main() {
    run_command();
}