use clap::Parser;
use cli::commands::{Commands, HinetCli};
use cli::user_handler;
use root::runtime;
use tokio::select;

#[tokio::main]
async fn main() {
    println!("System initialize..");
    let rt = runtime::Runtime::run();
    let hinet_cli = HinetCli::parse();

    match &hinet_cli.commands {
        Commands::Start(sargs) => {
            // run the system in background and listen to user inputs from stdin
            select! {
                _ = user_handler::handle_user_command() => {
                    println!("User input handler exited.");
                }
                _ = rt => {
                    println!("System runtime exited.");
                }
            }
        }
        Commands::Listen => {}
    }
}
