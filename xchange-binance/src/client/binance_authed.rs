use retrofit_rs::{Body, Header, Query, Retrofit, RetrofitError, api, get, post};

#[api("https://api.binance.com")]
pub trait BinanceAuthenticated {
    // #[post("/api/v3/order")]
    // async fn new_order(
    //     &self,
    //     symbol: Body<&str>,
    //     side: Body<&str>,
    //     type_: Body<&str>,
    //     time_in_force: Body<Option<&str>>,
    //     quantity: Body<Option<f64>>,
    //     quote_order_qty: Body<Option<f64>>,
    //     price: Body<Option<f64>>,
    //     new_client_order_id: Body<Option<&str>>,
    //     stop_price: Body<Option<f64>>,
    //     trailing_delta: Body<Option<u64>>,
    //     iceberg_qty: Body<Option<f64>>,
    //     new_order_resp_type: Body<Option<&str>>,
    //     recv_window: Body<Option<u64>>,
    //     timestamp: Body<u64>,
    //     #[header("X-MBX-APIKEY")] api_key: Header<&str>,
    //     #[query("signature")] signature: Query<&str>,
    // ) -> Result<BinanceNewOrder, retrofit_rs::Error>;

    #[get("/api/v3/order")]
    async fn order_status(
        &self,
        symbol: Query<&str>,
        order_id: Query<u64>,
        orig_client_order_id: Query<&str>,
        recv_window: Query<u64>,
        timestamp: Query<u64>,
    ) -> Result<serde_json::Value, RetrofitError>;

    // DELETE, PUT 等方法同理
}

impl BinanceAuthenticatedClient {
    pub fn retrofit(&self) -> &Retrofit {
        &self.client
    }
}
