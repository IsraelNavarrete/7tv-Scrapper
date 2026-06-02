use std::fmt::Display;
use serde_derive::Serialize;

#[derive(Serialize)]
pub(crate) enum Filter {
    ANIMADO,
    ESTATICO,
    SUPERPOSICION,
    USOPERSONAL,
    COINCIDENCIAEXACTA,
}
#[derive(Serialize)]
pub(crate) struct EmoteSearchVariables {
    default_set_id: String,
    filters: Vec<Filter>,
    is_default_set_set: bool,
    page: u32,
    per_page: u32,
    query: String,
    sort_by: Sort,
    tags: Vec<String>,
}

#[derive(PartialEq, Clone, Serialize)]
#[warn(dead_code)]
pub enum Sort {
    TOPALLTIME,
    TRENDINGWEEKLY,
    UPLOADDATE,
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Sort::TOPALLTIME => "TOP_ALL_TIME".to_string(),
            Sort::TRENDINGWEEKLY => "TRENDING_WEEKLY".to_string(),
            Sort::UPLOADDATE => "UPLOAD_DATE".to_string(),
        };
        write!(f, "{}", str)
    }
}