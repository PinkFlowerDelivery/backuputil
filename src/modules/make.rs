use clap::{Command, Arg};
use tokio::fs;
use tracing::{info, error, debug};
use std::path::PathBuf;


pub fn make_command() -> Command {
    Command::new("make")
        .alias("mk")
        .about("Make backup")
        .arg(Arg::new("name")
            .value_name("NAME") 
            .required(true)
            )
        .arg(Arg::new("ignoredot")
            .long("ignrdot")
            .short('i')
            .action(clap::ArgAction::SetTrue)
            )
}

pub async fn make_handle(name: &str, ignoredot: bool) {
    let pathbuf: PathBuf = PathBuf::from(name);

    if pathbuf.is_dir() {
        if let Ok(_) = fs::create_dir(format!("{}/backups/{}", dirs::home_dir().unwrap().to_string_lossy(), name)).await {
            info!("Backup created.");
        } else {
            error!("Faile to create backup :c");
        }
        backup_dir(pathbuf, name, ignoredot);
    } else if pathbuf.is_file() {
        if let Ok(_) = fs::copy(name, format!("{}/backups/{}", dirs::home_dir().unwrap().to_string_lossy(), name)).await {
            info!("Backup created.");
        } else {
            error!("Failed to create backup :c");
        }
    } else {
        error!("Failed...");
    }
}

fn backup_dir(path: PathBuf, name: &str, ignoredot: bool) {
    debug!("Backup dir function init...");
    let dir = std::fs::read_dir(&path).unwrap();

    for entry in dir {
        let entry = entry.unwrap();
        let dir = entry.path();

        if ignoredot && entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        
        if dir.is_file() {
            if let Err(_) = std::fs::copy(&dir, format!("{}/backups/{}", dirs::home_dir().unwrap().to_string_lossy(), &dir.display())) {
                error!("Failed to copy files.");
            }
        }

    if dir.is_dir() {
        if let Err(_) = std::fs::create_dir(format!("{}/backups/{}", dirs::home_dir().unwrap().to_string_lossy(), &dir.display())) {
            error!("Failed to copy dirs.");
        } 
        backup_dir(dir, name, ignoredot);
    }
}
}



