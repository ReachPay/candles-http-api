use my_settings_reader::SettingsModel;
use serde::{Deserialize, Serialize};

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel{
    #[serde(rename = "CandlesGrpc")]
    pub candles_grpc: String,
    #[serde(rename = "SbTcp")]
    pub sb_tcp: String,
}

#[async_trait::async_trait]
impl my_service_bus_tcp_client::MyServiceBusSettings for SettingsModel {
    async fn get_host_port(&self) -> String {
        self.sb_tcp.clone()
    }
}