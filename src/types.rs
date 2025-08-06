use tokio::sync::oneshot;

#[derive(Debug)]
pub struct QueryRequest {
    pub raw_sql: String,
    pub respond_to: oneshot::Sender<QueryResult>,
}

#[derive(Debug)]
pub struct ParsedQuery {
    pub ast: String,
    pub respond_to: oneshot::Sender<QueryResult>,
}

#[derive(Debug)]
pub struct PlannedQuery {
    pub plan: String,
    pub respond_to: oneshot::Sender<QueryResult>,
}

#[derive(Debug)]
pub struct QueryResult {
    pub output: String,
}
