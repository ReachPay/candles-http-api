use std::sync::Arc;

use candles_cache::CandlesInstrumentsCache;
use my_service_bus_tcp_client::MyServiceBusClient;
use rust_extensions::AppStates;

use crate::SettingsModel;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext{
    pub cache: CandlesInstrumentsCache,
    pub app_states: Arc<AppStates>,
    pub my_sb_connection: MyServiceBusClient,
}

impl AppContext {
    pub fn new_with_cache(cache: CandlesInstrumentsCache, settings: Arc<SettingsModel>) -> Self{

        let my_sb_connection = MyServiceBusClient::new(
            APP_NAME,
            APP_VERSION,
            settings,
            my_logger::LOGGER.clone(),
        );

        Self { cache, app_states: Arc::new(
            AppStates::create_initialized()),
             my_sb_connection }
    }

}