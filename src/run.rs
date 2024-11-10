use log::info;
use std::{
    fs::File,
    io::{Read, Write},
    os::unix::process::CommandExt,
    path::Path,
    process::Command,
};

pub fn run(java_path: &str, jdk_version: &str, mc_version: &str, work_path: &str, jar_path: &str) {
    info!("Match jdk version");
    match jdk_version {
        "21" => {
            if mc_version < "1.20" {
                panic!("Thec version is too early");
            }
        }
        "17" => {
            if mc_version >= "1.20" || mc_version <= "1.16.5" {
                panic!("The mc version is too early or high");
            }
        }
        "11" => {
            if mc_version != "1.16.5" {
                panic!("The mc version does not match");
            }
        }
        _ => panic!("Unknown java version"),
    }

    let eula_path = Path::new(work_path).join("eula.txt");
    let mut file = File::open(eula_path.clone()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if contents.contains("eula=false") {
        println!("Change in {}eula.txt", work_path);
        let new_contents = contents.replace("eula=false", "eula=true");
        let _ = file /*.expect("File write failure")*/
            .write_all(new_contents.as_bytes());
    }

    println!("Startup server");
    info!("Startup server");

    Command::new(java_path)
        .arg("-jar")
        .arg(jar_path)
        .arg("--nogui")
        .current_dir(work_path)
        .exec();
}
