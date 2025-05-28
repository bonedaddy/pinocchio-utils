use {
    crate::commands,
    anyhow::Result,
    clap::{Parser, Subcommand},
};

#[derive(Parser)]
pub struct Cli {
    #[arg(long, default_value = "config.yaml", help = "configuration file")]
    config: String,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ConfigInit {},
}

impl Cli {
    pub async fn handle(self) -> Result<()> {
        match self.cmd {
            Commands::ConfigInit {} => commands::config_init::config_init(&self.config).await,
        }
    }
}
