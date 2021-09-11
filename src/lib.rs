use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Condition {
    ConditionAnd(ConditionAnd),
    ConditionNot(ConditionNot),
    ConditionOr(ConditionOr),
    ConditionCompare(ConditionCompare),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QuerySchema {
    pub from: Vec<String>,
    pub condition: Condition,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Unknown {
    pub offset: usize,
    pub length: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryResult {
    pub query: Option<Vec<QuerySchema>>,
    pub unknown: Option<Vec<Unknown>>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct ConditionCompare {
    pub compare: Compare,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct ConditionAnd {
    pub and: Vec<Condition>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConditionNot {
    pub not: Box<Condition>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ConditionOr {
    pub or: Vec<Condition>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]

pub enum CompareValue {
    String(String),
    F64(f64),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]

pub enum Compare {
    CompareEq(CompareEq),
    CompareLt(CompareLt),
    CompareLte(CompareLte),
    CompareGt(CompareGt),
    CompareGte(CompareGte),
    CompareNot(CompareNot),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct CompareEq {
    pub key: String,
    pub eq: CompareValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct CompareLt {
    pub key: String,
    pub lt: CompareValue,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CompareLte {
    pub key: String,
    pub lte: CompareValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct CompareGt {
    pub key: String,
    pub gt: CompareValue,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct CompareGte {
    pub key: String,
    pub gte: CompareValue,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct CompareNot {
    pub key: String,
    pub not: CompareValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_deserialization() {
        let query = QueryResult {
            query: Some(vec![QuerySchema {
                from: vec!["from".to_string()],
                condition: Condition::ConditionAnd(ConditionAnd {
                    and: vec![Condition::ConditionOr(ConditionOr {
                        or: vec![Condition::ConditionCompare(ConditionCompare {
                            compare: Compare::CompareEq(CompareEq {
                                key: "key".to_string(),
                                eq: CompareValue::String("eq".to_string()),
                            }),
                        })],
                    })],
                }),
            }]),
            unknown: None,
        };
        let json = serde_json::to_string(&query).unwrap();
        let query_from_json = serde_json::from_str::<QueryResult>(json.as_str()).unwrap();
        assert_eq!(query, query_from_json);
    }
}
