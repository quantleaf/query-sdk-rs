pub enum ConditionElement {
    ConditionAnd(ConditionAnd),
    ConditionNot(ConditionNot),
    ConditionOr(ConditionOr),
}

pub struct QuerySchema {
    pub from: Vec<String>,
    pub condition: ConditionElement,
}
pub struct Unknown {
    pub offset: usize,
    pub length: usize,
}

pub struct QueryResult {
    pub query: Option<Vec<QuerySchema>>,
    pub unknown: Option<Vec<Unknown>>,
}

pub struct ConditionCompare {
    pub compare: Compare,
}
pub struct ConditionAnd {
    pub and: Vec<ConditionElement>,
}
pub struct ConditionNot {
    pub not: Box<ConditionElement>,
}
pub struct ConditionOr {
    pub or: Vec<ConditionElement>,
}

pub enum CompareValue {
    String(String),
    F64(f64),
    Bool(bool),
}
pub struct Compare {
    pub key: String,
    pub lt: Option<CompareValue>,
    pub lte: Option<CompareValue>,
    pub gt: Option<CompareValue>,
    pub gte: Option<CompareValue>,
    pub eq: Option<CompareValue>,
    pub neq: Option<CompareValue>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
