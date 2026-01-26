use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "hinet-cli", version = "0.0.1", author = "xpan")]
pub struct HinetCli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Start(StartArgs),
    Listen,
}

#[derive(Args, Debug)]
pub struct StartArgs {}

// -- The runtime cli configuration
#[derive(Parser, Debug)]
#[command(name = "hinet-runtime", version = "0.0.1", author = "xpan")]
pub struct HinetRuntimeCli {
    #[command(subcommand)]
    pub commands: RuntimeCommands,
}

#[derive(Subcommand, Debug)]
pub enum RuntimeCommands {
    Connect,
}
