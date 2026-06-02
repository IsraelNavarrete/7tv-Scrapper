use serde_derive::Serialize;

pub(crate) const QUERY_ONEEMOTE_HEADER: &str = "query OneEmote($id: Id!, $isDefaultSetSet: Boolean!, $defaultSetId: Id!)";
pub(crate) const QUERY_ONEEMOTE_BODY: &str = "{\n  emotes {\n    emote(id: $id) {\n      id\n      defaultName\n      owner {\n        id\n        mainConnection {\n          platformDisplayName\n          platformAvatarUrl\n          __typename\n        }\n        style {\n          activeProfilePicture {\n            images {\n              url\n              mime\n              size\n              width\n              height\n              scale\n              frameCount\n              __typename\n            }\n            __typename\n          }\n          activePaint {\n            id\n            name\n            data {\n              layers {\n                id\n                ty {\n                  __typename\n                  ... on PaintLayerTypeSingleColor {\n                    color {\n                      hex\n                      __typename\n                    }\n                    __typename\n                  }\n                  ... on PaintLayerTypeLinearGradient {\n                    angle\n                    repeating\n                    stops {\n                      at\n                      color {\n                        hex\n                        __typename\n                      }\n                      __typename\n                    }\n                    __typename\n                  }\n                  ... on PaintLayerTypeRadialGradient {\n                    repeating\n                    stops {\n                      at\n                      color {\n                        hex\n                        __typename\n                      }\n                      __typename\n                    }\n                    shape\n                    __typename\n                  }\n                  ... on PaintLayerTypeImage {\n                    images {\n                      url\n                      mime\n                      size\n                      scale\n                      width\n                      height\n                      frameCount\n                      __typename\n                    }\n                    __typename\n                  }\n                }\n                opacity\n                __typename\n              }\n              shadows {\n                color {\n                  hex\n                  __typename\n                }\n                offsetX\n                offsetY\n                blur\n                __typename\n              }\n              __typename\n            }\n            __typename\n          }\n          __typename\n        }\n        highestRoleColor {\n          hex\n          __typename\n        }\n        editors {\n          editorId\n          permissions {\n            emote {\n              manage\n              __typename\n            }\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      tags\n      flags {\n        animated\n        approvedPersonal\n        defaultZeroWidth\n        deniedPersonal\n        nsfw\n        private\n        publicListed\n        __typename\n      }\n      attribution {\n        user {\n          mainConnection {\n            platformDisplayName\n            platformAvatarUrl\n            __typename\n          }\n          style {\n            activeProfilePicture {\n              images {\n                url\n                mime\n                size\n                width\n                height\n                scale\n                frameCount\n                __typename\n              }\n              __typename\n            }\n            __typename\n          }\n          highestRoleColor {\n            hex\n            __typename\n          }\n          __typename\n        }\n        __typename\n      }\n      imagesPending\n      images {\n        url\n        mime\n        size\n        width\n        height\n        scale\n        frameCount\n        __typename\n      }\n      ranking(ranking: TRENDING_WEEKLY)\n      inEmoteSets(emoteSetIds: [$defaultSetId]) @include(if: $isDefaultSetSet) {\n        emoteSetId\n        emote {\n          id\n          alias\n          __typename\n        }\n        __typename\n      }\n      deleted\n      __typename\n    }\n    __typename\n  }\n}";

#[derive(Serialize)]
pub(crate) struct QueryOneEmote {
    operation_name: String,
    query: String,
    variables: OneEmoteVariables,
}

#[derive(Serialize)]
pub(crate) struct OneEmoteVariables {
    pub(crate) default_set_id: String,
    pub(crate) id: String,
    pub(crate) is_default_set_set: bool,
}

pub(crate) fn build_one_emote_body(id_emote: String) -> QueryOneEmote {
    let full_query = String::from(QUERY_ONEEMOTE_HEADER)
        + QUERY_ONEEMOTE_BODY;

    QueryOneEmote {
        operation_name: String::from("OneEmote"),
        query: full_query,
        variables: OneEmoteVariables {
            default_set_id: String::new(),
            id: id_emote,
            is_default_set_set: false,
        },
    }
}