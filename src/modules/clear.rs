use clap::Command;
use tokio::fs;

use crate::errors;

pub fn clear_command() -> Command {
    Command::new("clear")
        .alias("cl")
        .about("Clear backup dir")
}

pub async fn clear_handle() -> Result<(), errors::Errors> {
    let homedir = dirs::home_dir().unwrap();
    let fmthomedir = homedir.to_string_lossy();

    let mut backupdir = fs::read_dir(format!("{}/backups/", &fmthomedir)).await.unwrap(); 

    while let Some(entry) = backupdir.next_entry().await? {
         let file = entry.path();

        if file.is_dir() {
            fs::remove_dir_all(&file).await?
        } else {
            fs::remove_file(&file).await?
        }
       
    }
    println!("Successfully");
    Ok(())
}
