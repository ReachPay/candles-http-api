use std::sync::Arc;
use candles_cache::CandlesBidAsk;
use my_service_bus_abstractions::subscriber::{MessagesReader, SubscriberCallback, MySbSubscriberHandleError};

use crate::{BidAskProtobufModel, AppContext};


pub struct BidAskListener {
    pub app: Arc<AppContext>,
}

impl BidAskListener {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl SubscriberCallback<BidAskProtobufModel> for BidAskListener {
    async fn handle_messages(
        &self,
        messages_reader: &mut MessagesReader<BidAskProtobufModel>,
    ) -> Result<(), MySbSubscriberHandleError>{
        while let Some(message) = messages_reader.get_next_message() { 
            let mut message: CandlesBidAsk = message.get_messsage().into();
            println!("Handled bid ask: {:?}", message);
            message.date = message.date / 1000000;
            self.app.cache.handle_new_bid_ask(vec![message]).await;
        }

        Ok(())
    }

}
