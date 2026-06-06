use std::fmt::Display;
use serde_derive::Serialize;

#[derive(PartialEq, Clone)]
#[warn(dead_code)]
pub enum Size {
    SIZE1,
    SIZE2,
    SIZE3,
    SIZE4,
}

impl Display for Size{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Size::SIZE1 => "1x".to_string(),
            Size::SIZE2 => "2x".to_string(),
            Size::SIZE3 => "3x".to_string(),
            Size::SIZE4 => "4x".to_string(),
        };
        write!(f, "{}", str)
    }
}

// #[derive(Serialize)]
// pub(crate) enum Filter {
//     ANIMADO,
//     ESTATICO,
//     SUPERPOSICION,
//     USOPERSONAL,
//     COINCIDENCIAEXACTA,
// }

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
