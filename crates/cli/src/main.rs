use {anyhow::Result, clap::Parser, cli::Cli};

mod cli;
mod commands;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.handle().await
}
