use std::fmt::Display;
use serde::Serialize;

pub(crate) const QUERY_ONEEMOTE_HEADER: &str = "query OneEmote($id: Id!, $isDefaultSetSet: Boolean!, $defaultSetId: Id!)";
pub(crate) const QUERY_ONEEMOTE_BODY: &str = "{\n  emotes {\n    emote(id: $id) {\n      id\n      defaultName\n      owner {\n        id\n        mainConnection {\n          platformDisplayName\n          platformAvatarUrl\n          __typename\n        }\n        style {\n          activeProfilePicture {\n            images {\n              url\n              mime\n              size\n              width\n              height\n              scale\n              frameCount\n              __typename\n            }\n            __typename\n          }\n          activePaint {\n            id\n            name\n            data {\n              layers {\n                id\n                ty {\n                  __typename\n                  ... on PaintLayerTypeSingleColor {\n                    color {\n                      hex\n                      __typename\n                    }\n                    __typename\n                  }\n                  ... on PaintLayerTypeLinearGradient {\n                    angle\n                    repeating\n                    stops {\n                      at\n                      color {\n                        hex\n                        __typename\n                      }\n                      __typename\n                    }\n                    __typename\n                  }\n                  ... on PaintLayerTypeRadialGradient {\n                    repeating\n                    stops {\n                      at\n                      color {\n                        hex\n                        __typename\n                      }\n                      __typename\n                    }\n                    shape\n                    __typename\n                  }\n                  ... on PaintLayerTypeImage {\n                    images {\n                      url\n                      mime\n                      size\n                      scale\n                      width\n                      height\n                      frameCount\n                      __typename\n                    }\n                    __typename\n                  }\n                }\n                opacity\n                __typename\n              }\n              shadows {\n                color {\n                  hex\n                  __typename\n                }\n                offsetX\n                offsetY\n                blur\n                __typename\n              }\n              __typename\n            }\n            __typename\n          }\n          __typename\n        }\n        highestRoleColor {\n          hex\n          __typename\n        }\n        editors {\n          editorId\n          permissions {\n            emote {\n              manage\n              __typename\n            }\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      tags\n      flags {\n        animated\n        approvedPersonal\n        defaultZeroWidth\n        deniedPersonal\n        nsfw\n        private\n        publicListed\n        __typename\n      }\n      attribution {\n        user {\n          mainConnection {\n            platformDisplayName\n            platformAvatarUrl\n            __typename\n          }\n          style {\n            activeProfilePicture {\n              images {\n                url\n                mime\n                size\n                width\n                height\n                scale\n                frameCount\n                __typename\n              }\n              __typename\n            }\n            __typename\n          }\n          highestRoleColor {\n            hex\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      imagesPending\n      images {\n        url\n        mime\n        size\n        width\n        height\n        scale\n        frameCount\n        __typename\n      }\n      ranking(ranking: TRENDING_WEEKLY)\n      inEmoteSets(emoteSetIds: [$defaultSetId]) @include(if: $isDefaultSetSet) {\n        emoteSetId\n        emote {\n          id\n          alias\n          __typename\n        }\n        __typename\n      }\n      deleted\n      __typename\n    }\n    __typename\n  }\n}";
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
#[derive(Serialize)]
pub(crate) struct OneEmoteVariables {
    pub(crate) default_set_id: String,
    pub(crate) id: String,
    pub(crate) is_default_set_set: bool,
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
