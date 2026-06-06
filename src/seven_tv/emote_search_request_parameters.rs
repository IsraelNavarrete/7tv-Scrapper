use serde_derive::Serialize;

pub(crate) const QUERY_EMOTE_SEARCH_HEADER: &str = "query EmoteSearch($query: String, $tags: [String!]!, $sortBy: SortBy!, $filters: Filters, $page: Int, $perPage: Int!, $isDefaultSetSet: Boolean!, $defaultSetId: Id!)";
pub(crate) const QUERY_EMOTE_SEARCH_BODY: &str = "{\n  emotes {\n    search(\n      query: $query\n      tags: {tags: $tags, match: ANY}\n      sort: {sortBy: $sortBy, order: DESCENDING}\n      filters: $filters\n      page: $page\n      perPage: $perPage\n    ) {\n      items {\n        id\n        defaultName\n        owner {\n          mainConnection {\n            platformDisplayName\n            __typename\n          }\n          style {\n            activePaint {\n              id\n              name\n              data {\n                layers {\n                  id\n                  ty {\n                    __typename\n                    ... on PaintLayerTypeSingleColor {\n                      color {\n                        hex\n                        __typename\n                      }\n                      __typename\n                    }\n                    ... on PaintLayerTypeLinearGradient {\n                      angle\n                      repeating\n                      stops {\n                        at\n                        color {\n                          hex\n                          __typename\n                        }\n                        __typename\n                      }\n                      __typename\n                    }\n                    ... on PaintLayerTypeRadialGradient {\n                      repeating\n                      stops {\n                        at\n                        color {\n                          hex\n                          __typename\n                        }\n                        __typename\n                      }\n                      shape\n                      __typename\n                    }\n                    ... on PaintLayerTypeImage {\n                      images {\n                        url\n                        mime\n                        size\n                        scale\n                        width\n                        height\n                        frameCount\n                        __typename\n                      }\n                      __typename\n                    }\n                  }\n                  opacity\n                  __typename\n                }\n                shadows {\n                  color {\n                    hex\n                    __typename\n                  }\n                  offsetX\n                  offsetY\n                  blur\n                  __typename\n                }\n                __typename\n              }\n              __typename\n            }\n            __typename\n          }\n          highestRoleColor {\n            hex\n            __typename\n          }\n          __typename\n        }\n        deleted\n        flags {\n          defaultZeroWidth\n          private\n          publicListed\n          __typename\n        }\n        imagesPending\n        images {\n          url\n          mime\n          size\n          scale\n          width\n          frameCount\n          __typename\n        }\n        ranking(ranking: TRENDING_WEEKLY)\n        inEmoteSets(emoteSetIds: [$defaultSetId]) @include(if: $isDefaultSetSet) {\n          emoteSetId\n          emote {\n            id\n            alias\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      totalCount\n      pageCount\n      __typename\n    }\n    __typename\n  }\n}";
#[derive(Serialize, Debug)]
pub(crate) struct QueryEmoteSearch {
    operation_name: String,
    query: String,
    variables: EmoteSearchVariables,
}

#[derive(Serialize, Debug)]
pub(crate) struct EmoteSearchVariables {
    #[serde(rename = "defaultSetId")]
    default_set_id: String,
    filters: Filters,
    #[serde(rename = "isDefaultSetSet")]
    is_default_set_set: bool,
    page: u32,
    #[serde(rename = "perPage")]
    per_page: u32,
    query: String,
    #[serde(rename = "sortBy")]
    sort_by: String,
    tags: Vec<String>,
}

#[derive(Serialize, Debug)]
struct Filters {}

pub(crate) fn build_emote_search_body(filter: String, page: u32) -> QueryEmoteSearch {
    let full_query = String::from(QUERY_EMOTE_SEARCH_HEADER) + QUERY_EMOTE_SEARCH_BODY;

    QueryEmoteSearch {
        operation_name: String::from("EmoteSearch"),
        query: full_query,
        variables: EmoteSearchVariables {
            default_set_id: String::new(),
            filters: Filters {},
            is_default_set_set: false,
            page,
            per_page: 72,
            query: String::new(),
            sort_by: filter,
            tags: Vec::new(),
        },
    }
}
