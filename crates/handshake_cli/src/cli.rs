use std::time::Duration;

use clap::Parser;
use handshake::bitcoin::Node;

#[derive(Debug, Parser)]
#[command(name = "handshake")]
pub enum Cli {
    Bitcoin(HandshakeConfig),
}

#[derive(clap::Args, Debug)]
#[command(version, about, long_about = None)]
pub struct HandshakeConfig {
    #[arg(name = "Adress")]
    #[arg(help = "Remote node address")]
    remote: String,

    #[arg(long, default_value = "0")]
    #[arg(help = "Start height of a local node")]
    start_height: i32,

    #[arg(long,default_value_t=handshake::bitcoin::Network::Bitcoin)]
    #[arg(help = "Local node network")]
    network: handshake::bitcoin::Network,

    #[arg(long, default_value = "0")]
    #[arg(help = "Local node service flags")]
    services: u64,

    #[arg(long, default_value = "5000")]
    #[arg(help = "Handshake time limit in ms")]
    timeout: u64,
}

impl Cli {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let Cli::Bitcoin(args) = Cli::parse();
        let node = Node::new(args.services, args.start_height, args.network);
        let _ = node
            .connect(args.remote, Duration::from_millis(args.timeout))
            .await?;
        Ok(())
    }
}
