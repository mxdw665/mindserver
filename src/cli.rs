use clap::Parser;
use log::{error, info};
use serde_json::{from_str, Value};
use std::{
    fs::{metadata, read_dir, File},
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Main {
    #[arg(long)]
    jdk_path: String,
    #[arg(long)]
    jdk_version: String,
    #[arg(long)]
    mc_json_file: String,
    #[arg(long)]
    mc_version: String,
    #[arg(long)]
    save_path: String,
}

impl Main {
    fn check_jdk(&self) {
        info!("Found jdk");
        let jdk = format!("{}/bin/java", &self.jdk_path);
        if metadata(jdk).is_err() {
            panic!("Unknown path to the jdk");
        }
    }
    fn check_json(&self) {
        info!("Check json");
        if metadata(&self.mc_json_file).is_err() {
            panic!("Unknown mc version json file");
        }
    }
    pub fn parse_json(&self) -> Option<String> {
        info!("Parser json");
        let mut file = match File::open(&self.mc_json_file) {
            Ok(file) => file,
            Err(_) => panic!("File open failed"),
        };
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            panic!("Read file failed");
        }
        let json: Value = match from_str(&contents) {
            Ok(json) => json,
            Err(_) => panic!("JSON parse failed"),
        };
        if let Some(url) = json.get(&self.mc_version) {
            return Some(url.to_string());
        }

        None
    }
    fn check_server_jar(&self) -> Option<PathBuf> {
        info!("Found Server jar");
        let save_path = Path::new(&self.save_path);
        if let Ok(entries) = read_dir(save_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "jar") {
                    return Some(path);
                }
            }
        }
        error!("Not Found Server jar");
        None
    }
    pub fn run(&self) {
        use crate::download;
        self.check_jdk();
        self.check_json();
        match self.check_server_jar() {
            Some(_) => {
                let java_path = format!("{}/bin/java", self.jdk_path);
                crate::run::run(
                    &java_path,
                    self.jdk_version.as_str(),
                    self.mc_version.as_str(),
                    self.save_path.as_str(),
                    &self
                        .check_server_jar()
                        .expect("Variable conversion failure")
                        .to_string_lossy(),
                );
            }
            None => {
                let url = self.parse_json();
                let java_path = format!("{}/bin/java", self.jdk_path);
                download::download(url.as_ref().unwrap(), self.save_path.as_str());
                crate::run::run(
                    &java_path,
                    self.jdk_version.as_str(),
                    self.mc_version.as_str(),
                    self.save_path.as_str(),
                    &self
                        .check_server_jar()
                        .expect("Variable conversion failure")
                        .to_string_lossy(),
                );
            }
        }
    }
}
