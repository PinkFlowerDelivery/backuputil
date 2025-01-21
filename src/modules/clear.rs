use clap::Command;
use tokio::fs;
use tracing::{info, error, debug};

pub fn clear_command() -> Command {
    Command::new("clear")
        .alias("cl")
        .about("Clear backup dir")
}

pub async fn clear_handle() {
    let homedir = dirs::home_dir().unwrap();
    let fmthomedir = homedir.to_string_lossy();

    let mut backupdir = fs::read_dir(format!("{}/backups/", &fmthomedir)).await.unwrap(); 
    debug!("DIR | {:?}", backupdir);

    while let Some(entry) = backupdir.next_entry().await.unwrap() {
         let file = entry.path();

        if file.is_dir() {
            if let Err(e) = fs::remove_dir_all(&file).await {
                error!("Failed to remove dir {}", e);
            }
        } else {
            if let Err(_) = fs::remove_file(&file).await {
                error!("Failed to remove file");
            }
        }
       
    }

    info!("Backup cleared!");
}
