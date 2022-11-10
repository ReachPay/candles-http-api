use candles_cache::{CandlesInstrumentsCache, CandleModel, CandleType};
use my_telemetry::MyTelemetryContext;
use prost::encoding::bool;
use tonic::{transport::Channel, codegen::InterceptedService};

use crate::{MyInterceptor, orders_grpc::{simple_trading_engine_grpc_service_client::SimpleTradingEngineGrpcServiceClient, GetAllFromCacheGrpcRequest}};

pub struct OrdersFlowsGrpcClient {
    channel: Channel
}

impl OrdersFlowsGrpcClient {
    pub async fn new(grpc_address: String) -> Self {
        let channel = Channel::from_shared(grpc_address)
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            channel
        }
    }

    async fn create_grpc_service(
        &self,
        my_telemetry_context: &MyTelemetryContext,
    ) -> SimpleTradingEngineGrpcServiceClient<InterceptedService<Channel, MyInterceptor>> {
        let client: SimpleTradingEngineGrpcServiceClient<InterceptedService<Channel, MyInterceptor>> =
        SimpleTradingEngineGrpcServiceClient::with_interceptor(
                self.channel.clone(),
                MyInterceptor::new(my_telemetry_context),
            );

        client
    }

    pub async fn init_cache_for_candles(&self, is_bid: bool, my_telemetry_context: &MyTelemetryContext, cache: CandlesInstrumentsCache) -> CandlesInstrumentsCache{
        let mut client = self.create_grpc_service(my_telemetry_context).await;

        let result = client.get_all_from_cache(GetAllFromCacheGrpcRequest{
            is_bids: is_bid,
            source: "ST".to_string(),
        }).await.unwrap();

        let mut inner = result.into_inner();

        while let Some(candle) = inner.message().await.unwrap() {
            println!("is bid: {} {:?}", is_bid, candle.clone());
            
            let candle_model = candle.candle.unwrap();
            
            cache.init(candle.instrument_id, is_bid, 
                cast_candle_type(candle.candle_type as u8), 
                CandleModel { open: candle_model.open, close: candle_model.close, high: candle_model.high, low: candle_model.low, datetime: candle_model.date_time}).await;
        }

        return cache;
    }
}

fn cast_candle_type(source: u8) -> CandleType{
    match source {
        0 => CandleType::Minute,
        1 => CandleType::Hour,
        2 => CandleType::Day,
        3 => CandleType::Month,
        _ => panic!("not found candle type: {}", source),
    }
}