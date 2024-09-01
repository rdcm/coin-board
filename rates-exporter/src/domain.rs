pub struct FetchDataQuery {
    pub coins_ids: String,
}

#[async_trait::async_trait]
pub trait FetchRatesQueryHandler {
    async fn handle(&self, query: &FetchDataQuery) -> Option<()>;
}
