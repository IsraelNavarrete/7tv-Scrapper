use crate::download::filter_correct_image;
use crate::seven_tv::EmoteImage;
use reqwest::{Response, StatusCode};
use serde_derive::Deserialize;
use serde_derive::Serialize;

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
    pub search: Search,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub items: Vec<Item>,
    pub total_count: i64,
    pub page_count: i64,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub default_name: String,
    pub owner: Owner,
    pub deleted: bool,
    pub flags: Flags,
    pub images_pending: bool,
    pub images: Vec<EmoteImage>,
    pub ranking: Option<i64>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub main_connection: MainConnection,
    pub style: Style,
    pub highest_role_color: Option<HighestRoleColor>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainConnection {
    pub platform_display_name: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub active_paint: Option<ActivePaint>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivePaint {
    pub id: String,
    pub name: String,
    pub data: Data2,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data2 {
    pub layers: Vec<Layer>,
    pub shadows: Vec<Shadow>,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub id: String,
    pub ty: Ty,
    pub opacity: f64,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ty {
    #[serde(rename = "__typename")]
    pub typename: String,
    #[serde(default)]
    pub images: Vec<Image>,
    pub angle: Option<i64>,
    pub repeating: Option<bool>,
    #[serde(default)]
    pub stops: Vec<Stop>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub url: String,
    pub mime: String,
    pub size: i64,
    pub scale: i64,
    pub width: i64,
    pub height: i64,
    pub frame_count: i64,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub at: f64,
    pub color: Color,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub hex: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shadow {
    pub color: Color2,
    pub offset_x: f64,
    pub offset_y: f64,
    pub blur: f64,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color2 {
    pub hex: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighestRoleColor {
    pub hex: String,
    #[serde(rename = "__typename")]
    pub typename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    pub default_zero_width: bool,
    pub private: bool,
    pub public_listed: bool,
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

pub(crate) async fn handle_page_search_response(
    response: Response,
    filter: String,
    page: u32,
) -> Vec<Option<(String, String)>> {
    match response.status() {
        StatusCode::TOO_MANY_REQUESTS => {
            println!("Demasiadas peticiones a 7tv, espera un rato y vuelve a intentarlo");
            Vec::new()
        }
        StatusCode::NOT_FOUND => {
            println!(
                "No se ha encontrado la página: {}, con este filtro {}",
                page, filter
            );
            Vec::new()
        }
        StatusCode::OK => {
            let body: Root = response.json().await.unwrap();
            let items: Vec<Item> = body.data.emotes.search.items;
            let mut emotes_data = Vec::new();

            for item in items {
                let emote_name = item.default_name;
                let images_emote = item.images;

                emotes_data.push(filter_correct_image(
                    String::from("4x"),
                    emote_name,
                    images_emote,
                ));
            }

            emotes_data
        }
        _ => {
            println!("No se ha podido obtener una respuesta.");
            Vec::new()
        }
    }
}
