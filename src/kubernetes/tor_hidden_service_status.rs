#[derive(Clone, Debug, schemars::JsonSchema, serde::Serialize, serde::Deserialize)]
pub struct TorHiddenServiceStatus {
    pub hostname: Option<String>,
}
