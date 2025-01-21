use tracing_subscriber::FmtSubscriber;
use tracing::{error, warn, debug};

mod modules;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder() 
        //.with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_file(false)
        .with_ansi(true)
        .with_thread_ids(false)
        .without_time()
        .compact()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Error to set subscriber.");

    let matches = modules::build_cli().get_matches();

    match matches.subcommand() {
        Some(("make", sub)) => {
            if let Some(name) = sub.get_one::<String>("name") {
                let ignoredot = sub.get_flag("ignoredot");
                modules::make::make_handle(name, ignoredot).await;      
                debug!("[Make] Launched..."); 
            } else {
                error!("Argument NAME not found."); 
            }
        }
        Some(("clear", _)) => {
            modules::clear::clear_handle().await;
            debug!("[Clear] Laucnhed...");
        }

        _ => warn!("Command not found"),
    }
}

