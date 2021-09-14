use models::query_request::{QueryActions, QueryOptions, QueryRequest};
use models::query_result::QueryResult;
use models::query_schema::Schema;
use std::env;
mod models;

pub struct Client {
    web_client: reqwest::Client,
    api_key: String,
    request_template: String,
}
impl Client {
    pub fn new(
        schemas: Vec<Schema>,
        actions: QueryActions,
        options: Option<QueryOptions>,
    ) -> Client {
        dotenv::dotenv().ok();
        Client {
            web_client: reqwest::Client::new(),
            api_key: get_api_key(),
            request_template: serde_json::to_string(&QueryRequest {
                text: "".to_owned(),
                schemas: Some(schemas),
                options,
                actions,
                schemas_hash: None,
            })
            .unwrap()
            .chars()
            .skip(10)
            .collect(),
        }
    }

    pub async fn translate(&self, text: &str) -> Result<QueryResult, reqwest::Error> {
        // Build a request in a hacky but fast way (we do not need to re serialize the whole request only because the query has changed)
        let body = format!(
            "{{\"text\":\"{}\"{}",
            text,
            self.request_template.to_owned()
        );

        let resp = self
            .web_client
            .post("https://api.query.quantleaf.com/translate")
            .header("X-API-KEY", self.api_key.as_str())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await;

        match resp {
            Ok(result) => match result.error_for_status() {
                Ok(response) => {
                    return Ok(response.json::<QueryResult>().await.unwrap());
                }
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}

fn get_api_key() -> String {
    match env::var("QUANTLEAF_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("Failed to load API key"),
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        query_request::{QueryAction, SuggestionAction},
        query_result::{
            Compare, CompareEq, CompareValue, Condition, ConditionCompare, QuerySchema, Suggestion,
        },
        query_schema::{
            Domain, Field, KeyWithDescriptions, QueryOperations, SimpleDescription, SimpleDomain,
        },
    };

    use super::*;
    #[actix_rt::test]
    async fn it_can_create_client() {
        let client = Client::new(
            vec![Schema {
                fields: vec![Field {
                    description: SimpleDescription::String("field".into()),
                    domain: Domain::SimpleDomain(SimpleDomain::Text),
                    key: "key".into(),
                }],
                name: KeyWithDescriptions {
                    key: "name".into(),
                    description: SimpleDescription::String("name".into()),
                },
                operations: QueryOperations {
                    negative: false,
                    nesting: false,
                },
            }],
            QueryActions {
                query: Some(QueryAction {}),
                suggest: Some(SuggestionAction {
                    limit: 10,
                    offset: 0,
                }),
            },
            None,
        );

        let resp = client.translate("field equal hello").await.unwrap();
        let expected_result = QueryResult {
            query: Some(vec![QuerySchema {
                from: vec!["name".into()],
                condition: Condition::Compare(ConditionCompare {
                    compare: Compare::Eq(CompareEq {
                        eq: CompareValue::String("hello".into()),
                        key: "key".into(),
                    }),
                }),
            }]),
            schema_hash: Some("yAgJmHeX7gNMjtss4EKx/pFm65U=".into()),
            suggest: Some(vec![Suggestion {
                offset: 0,
                text: "field".into(),
            }]),
            unknown: None,
        };
        assert_eq!(resp, expected_result)
    }
}
