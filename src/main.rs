mod modules;
mod errors;

#[tokio::main]
async fn main() {
    let matches = modules::build_cli().get_matches();

    match matches.subcommand() {
        Some(("make", sub)) => {
            if let Some(name) = sub.get_one::<String>("name") {
                let ignoredot = sub.get_flag("ignoredot");
                modules::make::make_handle(name, ignoredot).await.unwrap();      
            } else {
                println!("Argument NAME not found."); 
            }
        }
        Some(("clear", _)) => {
            modules::clear::clear_handle().await.unwrap();
        }

        _ => println!("Command not found"),
    }
}

