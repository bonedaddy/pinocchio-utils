use {
    anyhow::{Context, Result},
    serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
    pub rpc_url: String,
}

impl Configuration {
    pub async fn save(&self, path: &str) -> Result<()> {
        tokio::fs::write(
            path,
            serde_yaml::to_string(self).with_context(|| "failed to serialize configuration")?,
        )
        .await
        .with_context(|| "failed to write configuration file")
    }
    pub async fn load(path: &str) -> Result<Self> {
        serde_yaml::from_str(
            &tokio::fs::read_to_string(path)
                .await
                .with_context(|| "failed to read configuration file")?,
        )
        .with_context(|| "failed to deserialize configuration file")
    }
}
