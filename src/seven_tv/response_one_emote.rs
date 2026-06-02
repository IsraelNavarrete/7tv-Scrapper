use reqwest::{Response, StatusCode};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
    pub extensions: Extensions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub emotes: Emotes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emotes {
    pub emote: Emote,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emote {
    pub id: String,
    pub default_name: String,
    pub owner: Owner,
    pub tags: Vec<Value>,
    pub flags: Flags,
    pub attribution: Vec<Value>,
    pub images_pending: bool,
    pub images: Vec<Image>,
    pub ranking: Option<i64>,
    pub deleted: bool,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub id: String,
    pub main_connection: MainConnection,
    pub style: Style,
    pub highest_role_color: Value,
    pub editors: Vec<Editor>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainConnection {
    pub platform_display_name: String,
    pub platform_avatar_url: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub active_profile_picture: Value,
    pub active_paint: Value,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Editor {
    pub editor_id: String,
    pub permissions: Permissions,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    pub emote: Emote2,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emote2 {
    pub manage: bool,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    pub animated: bool,
    pub approved_personal: bool,
    pub default_zero_width: bool,
    pub denied_personal: bool,
    pub nsfw: bool,
    pub private: bool,
    pub public_listed: bool,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub mime: String,
    pub size: i64,
    pub width: i64,
    pub height: i64,
    pub scale: i64,
    pub frame_count: i64,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    pub analyzer: Analyzer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Analyzer {
    pub complexity: i64,
    pub depth: i64,
}

pub(crate) async fn handle_singe_emote_response(
    response: Response,
    emote_id: &str,
    size: String,
) -> Option<(String, String)> {
    match response.status() {
        StatusCode::TOO_MANY_REQUESTS => {
            println!("Demasiadas peticiones a seven_tv, espera un rato y vuelve a intentarlo");
            None
        }
        StatusCode::NOT_FOUND => {
            println!("No se ha encontrado el emote con ID: {}", emote_id);
            None
        }
        StatusCode::OK => {
            let body: Root = response.json().await.unwrap();

            let emote_name = body.data.emotes.emote.default_name;
            let images_emote = body.data.emotes.emote.images;

            filter_correct_image(size, emote_name, images_emote)
        }
        _ => {
            println!("No se ha podido obtener una respuesta.");
            None
        }
    }
}

fn filter_correct_image(
    size: String,
    emote_name: String,
    images_emote: Vec<Image>,
) -> Option<(String, String)> {
    images_emote
        .iter()
        .find(|&image| has_gif(image, &size))
        .or_else(|| has_png(size, &images_emote))
        .map(|image| (emote_name.clone(), image.url.clone()))
}

fn has_png(size: String, images_emote: &Vec<Image>) -> Option<&Image> {
    images_emote
        .iter()
        .find(|image| image.mime.contains("png") && is_correct_scale(size.clone(), image))
}

fn has_gif(image: &Image, size: &String) -> bool {
    image.mime.contains("gif") && is_correct_scale(size.clone(), image)
}

fn is_correct_scale(size: String, image: &Image) -> bool {
    image.scale
        == size
            .chars()
            .next()
            .unwrap()
            .to_string()
            .parse::<i64>()
            .unwrap()
}
