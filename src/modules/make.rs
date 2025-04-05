use clap::{Command, Arg};
use tokio::fs;
use std::path::PathBuf;

use crate::errors;

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

pub async fn make_handle(name: &str, ignoredot: bool) -> Result<(), errors::Errors> {
    let pathbuf: PathBuf = PathBuf::from(name);
    let homedir = dirs::home_dir().unwrap();
    let fmthomedir = homedir.to_string_lossy();

    if pathbuf.is_dir() {
        fs::create_dir(format!("{}/backups/{}", &fmthomedir, name)).await?;
        backup_dir(pathbuf, name, ignoredot)?;
    } else if pathbuf.is_file() {
        fs::copy(name, format!("{}/backups/{}", &fmthomedir, name)).await?; 
    }
    println!("Successfully");
    Ok(())
}

fn backup_dir(path: PathBuf, name: &str, ignoredot: bool) -> Result<(), errors::Errors> {
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
            std::fs::copy(&file, format!("{}/backups/{}", &fmthomedir, &file.display()))?;
        }

        if file.is_dir() {
            std::fs::create_dir(format!("{}/backups/{}", &fmthomedir, &file.display()))?;
            backup_dir(file, name, ignoredot)?;
        }
    }
    Ok(())
}



