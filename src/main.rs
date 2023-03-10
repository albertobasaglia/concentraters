mod hostsmanager;

use std::{fs, path::Path, vec};

use hostsmanager::hostsfile::HostsFile;
use serde_derive::{Deserialize, Serialize};

static HOSTSFILE: &'static str = "/etc/hosts";
static HOSTSFILEBCK: &'static str = "/etc/hosts.bck";
static APPNAME: &'static str = "concentraters";

#[derive(Serialize, Deserialize)]
struct Config {
    block: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self { block: vec![] }
    }
}

fn status(_hf: HostsFile, cfg: &Config) {
    if check_enabled() {
        println!("status: ACTIVE");
    } else {
        println!("status: INACTIVE");
    }
    // hf.list_entries();
    println!("Blacklist:");
    for blocked in &cfg.block {
        println!("\t{}", blocked);
    }
}

fn add_block(cfg: &mut Config, name: &str) {
    cfg.block.push(name.to_string());
    confy::store(APPNAME, None, cfg).expect("Can't save config");
}

fn remove_block(cfg: &mut Config, name: &str) {
    cfg.block.retain(|blocked| blocked != name);
    confy::store(APPNAME, None, cfg).expect("Can't save config");
}

fn check_enabled() -> bool {
    let path = Path::new(HOSTSFILEBCK);
    path.exists()
}

fn enable(mut hf: HostsFile, cfg: &mut Config) {
    if check_enabled() {
        panic!("Already enabled");
    }
    for block in &cfg.block {
        hf.block_name_www(block);
    }
    fs::copy(HOSTSFILE, HOSTSFILEBCK).expect("Can't backup hosts");
    hf.write_to_file(HOSTSFILE).expect("Can't write file");
}

fn disable() {
    fs::copy(HOSTSFILEBCK, HOSTSFILE).expect("Can't restore hosts");
    fs::remove_file(HOSTSFILEBCK).expect("Can't delete backup");
}

fn refresh(hf: HostsFile, cfg: &mut Config) {
    if check_enabled() {
        disable();
        enable(hf, cfg);
    }
}

fn main() {
    let mut hf = HostsFile::new();
    hf.load_from_file(HOSTSFILE).expect("Can't read from file");

    let mut cfg: Config = confy::load(APPNAME, None).expect("Can't load config");

    match std::env::args().nth(1).expect("Arg error").as_str() {
        "status" => status(hf, &cfg),
        "add" => {
            let name = std::env::args().nth(2).expect("Arg error");
            add_block(&mut cfg, &name);
            refresh(hf, &mut cfg);
        }
        "remove" => {
            let name = std::env::args().nth(2).expect("Arg error");
            remove_block(&mut cfg, &name);
            refresh(hf, &mut cfg);
        }
        "enable" => enable(hf, &mut cfg),
        "disable" => disable(),
        "help" => {
            let path = confy::get_configuration_file_path(APPNAME, None)
                .expect("Can't get configuration path");
            println!(
                "Path is {}",
                path.to_str().expect("Can't convert path to string")
            );
        }
        _ => todo!(),
    }
}
