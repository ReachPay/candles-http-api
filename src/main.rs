use std::sync::Arc;

use candles_cache::CandlesInstrumentsCache;
use candles_http_api::{SettingsModel, OrdersFlowsGrpcClient, AppContext, BidAskListener, setup_server};
use my_service_bus_abstractions::subscriber::TopicQueueType;
use my_telemetry::MyTelemetryContext;

const MY_SB_QUEUE_NAME: &str = "candles-http-rust";

#[tokio::main]
async fn main() {
    let settings = SettingsModel::load(".reachpay").await;
    let grpc = OrdersFlowsGrpcClient::new(settings.candles_grpc.to_string()).await;
    let mut cache = CandlesInstrumentsCache::new();
    cache = grpc.init_cache_for_candles(false, &MyTelemetryContext::new(), cache).await;
    cache = grpc.init_cache_for_candles(true, &MyTelemetryContext::new(), cache).await;

    let app = Arc::new(AppContext::new_with_cache(cache, Arc::new(settings)));

    app.my_sb_connection
    .subscribe(
        MY_SB_QUEUE_NAME.to_string(),
        TopicQueueType::Permanent,
        Arc::new(BidAskListener::new(app.clone())),
    )
    .await;

    setup_server(app.clone(), 9876);
    app.my_sb_connection.start().await;
    app.app_states.wait_until_shutdown().await;
}
