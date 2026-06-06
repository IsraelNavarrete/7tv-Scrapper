use serde_derive::{Deserialize, Serialize};

pub(crate) mod response_one_emote;
pub mod one_emote_request_parameters;
pub mod emote_search_request_parameters;
pub mod response_page_search;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmoteImage {
    pub url: String,
    pub mime: String,
    pub size: i64,
    pub scale: i64,
    pub width: i64,
    pub frame_count: i64,
    #[serde(rename = "__typename")]
    pub typename: String,
}