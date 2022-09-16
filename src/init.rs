use std::{
    fs::{create_dir, File},
    io::Write,
    path::PathBuf,
};

use crate::utils::{get_user_input, print_ascii_art, print_version};

pub(crate) fn init() {
    let (config, config_path) = create_config();
    print_ascii_art();
    print_version();
    println!();
    println!("Configuration creation process has started...");
    write_config(config);

    println!(
        "Configuration has been generated at {}",
        config_path.display()
    );

    println!("Ready for lift off! Run the follow command to begin:");
    println!("'subspace farm'");
}

// TODO: validate user inputs
// TODO: use the default values if user pressed enter
fn write_config(mut config: File) {
    // get user inputs
    let reward_address = get_user_input("Enter your farmer/reward address: ");
    let hostname = get_user_input("Enter your node name to be identified on the network(defaults to HOSTNAME, press enter to use the default): ");
    let plot_location = get_user_input(
        "Specify a sector location (whatever the default was, press enter to use the default): ",
    );
    let plot_size =
        get_user_input("Specify a sector size (defaults to 1GB, press enter to use the default): ");
    let chain = get_user_input(
        "Specify the chain to farm(defaults to `gemini-1`, press enter to use the default): ",
    );

    let config_text = construct_config(
        &reward_address,
        &plot_location,
        &plot_size,
        &chain,
        &hostname,
    );

    // write to config
    if let Err(why) = config.write(config_text.as_bytes()) {
        panic!("could not write to config file, because: {why}");
    }
}

/// Creates a config file at the location
/// - **Linux:** `$HOME/.config/subspace-cli/settings.toml`.
/// - **macOS:** `$HOME/Library/Application Support/subspace-cli/settings.toml`.
/// - **Windows:** `{FOLDERID_RoamingAppData}/subspace-cli/settings.toml`.
pub(crate) fn create_config() -> (File, PathBuf) {
    let config_path = match dirs::config_dir() {
        Some(path) => path,
        None => panic!("couldn't get the default config directory!"),
    };
    let config_path = config_path.join("subspace-cli");
    let _ = create_dir(config_path.clone()); // if folder already exists, ignore the error

    match File::create(config_path.join("settings.toml")) {
        Err(why) => panic!("couldn't create the config file because: {}", why),
        Ok(file) => (file, config_path),
    }
}

fn construct_config(
    reward_address: &str,
    plot_location: &str,
    plot_size: &str,
    chain: &str,
    hostname: &str,
) -> String {
    format!(
        "[farmer]
address = \"{}\"
sector_directory = \"{}\"
sector_size = \"{}\"
opencl = false

[node]
chain = \"{}\"
execution = \"wasm\"
blocks-pruning = 1024
state-pruning = 1024
validator = true
name = \"{}\"
port = 30333
unsafe-ws-external = true # not sure we need this

[chains]
gemini-1 = \"rpc://1212312\"
gemini-2= \"rpc://\"
leo-3 = \"myown-network\"
dev = \"that local node experience\"
",
        reward_address, plot_location, plot_size, chain, hostname
    )
}
