use crate::strategy::ThroughputStrategy;
use serde::Deserialize;
use std::collections::HashMap;

#[repr(u8)]
#[derive(Debug, Clone, Deserialize)]
pub enum StrategyType {
    Throughput,
    Corrupt,
    SchemaChange,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigFile {
    pub librdkafka: HashMap<String, String>,
    pub strategy: StrategyConfig,
    pub schema_registry_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StrategyConfig {
    pub name: String,
    pub typ3: StrategyType,
    pub throughput: Option<ThroughputStrategy>,
}

pub struct AppConfig {
    pub topic: String,
    pub config_file: ConfigFile,
}
