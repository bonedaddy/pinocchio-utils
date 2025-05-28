use anyhow::Result;
use config::Configuration;

pub async fn config_init(path: &str) -> Result<()> {
    Configuration::default().save(path).await
}
