use candles_cache::CandlesBidAsk;

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_service_bus_macros::my_sb_entity_protobuf_model(topic_id = "bid-ask-new")]
pub struct BidAskProtobufModel {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub source_date_time: i64,
    #[prost(sint64, tag = "3")]
    pub our_date_time: i64,
    #[prost(double, tag = "4")]
    pub bid: f64,
    #[prost(double, tag = "5")]
    pub ask: f64,
}

impl Into<CandlesBidAsk> for &BidAskProtobufModel {
    fn into(self) -> CandlesBidAsk {
        CandlesBidAsk{
            date: self.our_date_time as u64,
            instrument: self.id.clone(),
            bid: self.bid,
            ask: self.ask,
        }
    }
}