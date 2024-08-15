pub struct FetchDataQuery {
    pub fetch_delay_ms: u64,
    pub page_size: i32,
}

impl FetchDataQuery {
    pub fn new(fetch_delay_ms: u64, page_size: i32) -> Self {
        Self {
            fetch_delay_ms,
            page_size,
        }
    }
}

#[async_trait::async_trait]
pub trait FetchRatesQueryHandler {
    async fn handle(&self, query: &FetchDataQuery) -> Option<()>;
}
