pub struct FetchDataQuery {
    pub source: String,
}

impl FetchDataQuery {
    pub fn new(source: String) -> Self {
        FetchDataQuery { source }
    }
}

#[async_trait::async_trait]
pub trait FetchRatesQueryHandler {
    async fn handle(&self, query: &FetchDataQuery) -> Option<()>;
}
