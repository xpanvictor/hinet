use clap::Parser;
use cli::commands::{Commands, HinetCli};
use root::runtime;

#[tokio::main]
async fn main() {
    println!("System initialize..");
    let rt = runtime::Runtime::run();
    let hinet_cli = HinetCli::parse();

    match &hinet_cli.commands {
        Commands::Start(sargs) => match rt.await {
            Ok(_) => {
                println!("stable run")
            }
            Err(e) => {
                eprintln!("error running hinet: {}", e);
            }
        },
    }
}
