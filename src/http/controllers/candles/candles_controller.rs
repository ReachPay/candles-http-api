use std::{sync::Arc, str};

use candles_cache::CandleType;
use my_http_server::{HttpContext, HttpOkResult, HttpFailResult, HttpOutput};
use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Serialize, Deserialize};

use crate::AppContext;


#[derive(MyHttpInput)]
pub struct CancelOrderHttpRequest {
    #[http_form(name = "orderId"; description = "Id of transaction")]
    pub order_id: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct CandleApiModel{
    #[serde(rename = "d")]
    pub d: u64,
    #[serde(rename = "o")]
    pub o: f64,
    #[serde(rename = "c")]
    pub c: f64,
    #[serde(rename = "h")]
    pub h: f64,
    #[serde(rename = "l")]
    pub l: f64,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct CandleApiResponse {
    pub candles: Vec<CandleApiModel>,
}
 
#[derive(MyHttpInput)]
pub struct GetCandlesHttpRequest {
    #[http_query(name = "InstrumentId"; description = "Id of transaction")]
    pub instrument_id: String,
    #[http_query(name = "BidOrAsk"; description = "Bid or ask")]
    pub bid_or_ask: u8,
    #[http_query(name = "From"; description = "From date")]
    pub from: u64,
    #[http_query(name = "To"; description = "To date")]
    pub to: u64,
    #[http_query(name = "CandleType"; description = "Candle type")]
    pub candle_type: u8,
}

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/candles/v1",
    description: "Request candles",
    controller: "Candles",
    input_data: "GetCandlesHttpRequest",
    summary: "Get history",
    result:[
        {status_code: 200, description: "Ok response", model: "CandleApiResponse"},
    ]
)]

pub struct GetCandlesHttpAction {
    app: Arc<AppContext>,
}

impl GetCandlesHttpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}


async fn handle_request(
    action: &GetCandlesHttpAction,
    http_input: GetCandlesHttpRequest,
    _: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {

    let candle_type = map_api_contract_to_internal_candle_type(http_input.candle_type);
    let is_bid = map_api_bid_or_ask_to_is_bid(http_input.bid_or_ask);

    let start_date = format_date(http_input.from);
    let end_date = format_date(http_input.to);

    let candles = action.app.cache.get_by_date_range(http_input.instrument_id, candle_type, is_bid, start_date, end_date).await;

    let result = candles.iter().map(|x| CandleApiModel{
        d: x.datetime * 1000,
        o: x.open,
        c: x.close,
        h: x.high,
        l: x.low,
    }).collect();

    let response = CandleApiResponse{
        candles: result,
    };

    return HttpOutput::as_json(response).into_ok_result(true).into();
}

fn format_date(src: u64) -> u64{
    if 10 == src.to_string().len() {
        return src;
    }

    return src / 1000;
}

fn map_api_contract_to_internal_candle_type(src: u8) -> CandleType {
    match src {
        0 => CandleType::Minute,
        1 => CandleType::Hour,
        2 => CandleType::Day,
        3 => CandleType::Month,
        _ => panic!("MIssmatch candle type"),
    }
}

fn map_api_bid_or_ask_to_is_bid(src: u8) -> bool {
    match src {
        0 => false,
        1 => true,
        _ => panic!("MIssmatch candle type"),
    }
}
