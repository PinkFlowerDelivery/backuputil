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
    let homedir = dirs::home_dir().unwrap();
    let fmthomedir = homedir.to_string_lossy();

    if pathbuf.is_dir() {
        if let Err(_) = fs::create_dir(format!("{}/backups/{}", &fmthomedir, name)).await {
            error!("Failed to create backup :c");
        }
        backup_dir(pathbuf, name, ignoredot);
        debug!("Back up dir function working!");

    } else if pathbuf.is_file() {
        if let Err(_) = fs::copy(name, format!("{}/backups/{}", &fmthomedir, name)).await {
            error!("Failed to create backup :c");
        }
        debug!("Back up file function working!");
    } else {
        error!("Failed...");
    }
    info!("Backup succefully created.");
}

fn backup_dir(path: PathBuf, name: &str, ignoredot: bool) {
    let dir = std::fs::read_dir(&path).unwrap();
    let homedir = dirs::home_dir().unwrap();
    let fmthomedir = homedir.to_string_lossy();

    for entry in dir {
        let entry = entry.unwrap();
        let file = entry.path();

        if ignoredot && entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        
        if file.is_file() {
            if let Err(_) = std::fs::copy(&file, format!("{}/backups/{}", &fmthomedir, &file.display())) {
                error!("Failed to copy files.");
            }
        }

    if file.is_dir() {
        if let Err(_) = std::fs::create_dir(format!("{}/backups/{}", &fmthomedir, &file.display())) {
            error!("Failed to copy dirs.");
        } 
        backup_dir(file, name, ignoredot);
    }
}
}



