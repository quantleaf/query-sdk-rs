use crate::models::query_schema::Schema;
use serde::Serialize;
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct QueryRequest {
    pub text: String,
    pub schemas: Option<Vec<Schema>>,
    pub schemas_hash: Option<String>,
    pub actions: QueryActions,
    pub options: Option<QueryOptions>,
}
#[derive(Serialize, Debug)]
pub struct SuggestionAction {
    pub limit: usize,
    pub offset: usize,
}

#[derive(Serialize, Debug)]

pub struct QueryActions {
    pub query: Option<QueryAction>,
    pub suggest: Option<SuggestionAction>,
}

#[derive(Serialize, Debug)]

pub struct QueryAction {}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]

pub struct QueryOptions {
    pub concurrency_size: Option<usize>,
    pub fuzzy: bool,
    pub nested_conditions: bool,
    pub negative_conditions: bool,
}
