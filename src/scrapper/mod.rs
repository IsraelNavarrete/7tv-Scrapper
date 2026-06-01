pub(crate) mod query_7tv;
mod response_7tv;

use crate::scrapper::query_7tv::{OneEmoteVariables, QUERY_ONEEMOTE_BODY, QUERY_ONEEMOTE_HEADER};
use crate::scrapper::response_7tv::{Image, Root};
use reqwest::{Client, Response, StatusCode};
use serde::Serialize;
use std::time::Duration;
use serde_json::json;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Serialize)]
struct QueryEmoteSearch {
    operation_name: String,
    query: String,
    variables: query_7tv::EmoteSearchVariables,
}

#[derive(Serialize)]
struct QueryOneEmote {
    operation_name: String,
    query: String,
    variables: OneEmoteVariables,
}
#[derive(Clone)]
pub struct Scrapper {
    client: Client,
    base_7tv_url: String,
}

impl Scrapper {
    pub fn new(base_url: String) -> Result<Self, reqwest::Error> {
        let client = Client::builder().timeout(Duration::from_secs(20)).build()?;

        Ok(Self { client, base_7tv_url: base_url })
    }

    pub async fn download_single_emote(
        &self,
        url: String,
        size: String,
    ) -> Result<(), reqwest::Error> {
        let emote_id = url.split('/').last().unwrap().to_string();

        println!("Descargando emote con id: {}", emote_id);

        let body = build_one_emote_body(emote_id.clone());

        let content_length = json!(body).to_string().bytes().len();

        println!("length: {}", content_length);

        let response = self
            .client
            .post(self.base_7tv_url.clone())
            .header("content-length", content_length)
            .header("host", "api.7tv.app")
            .json(&body)
            .send()
            .await?;

        println!("Respuesta recibida: {:#?}", response);

        let emote_data = handle_response(response, &emote_id, size).await;

        download_image(emote_data, self.client.clone()).await;

        Ok(())
    }
}

async fn download_image(emote_data: Option<(String, String)>, client: Client) {
    if emote_data.is_none() {
        println!("No había emote que descargar");
        return;
    }

    let emote_url = emote_data.clone().unwrap().1;

    let pos_extension = emote_url.char_indices().nth_back(3).unwrap().0;

    let emote_name = String::from(emote_data.unwrap().0) + &emote_url[pos_extension..];

    let image = client
        .get(emote_url)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let mut emote_path = std::env::current_dir().unwrap();

    emote_path.push("/emotes/");

    std::fs::create_dir_all(&emote_path).unwrap();

    println!("Descargando en ruta: {}{}", emote_path.clone().to_str().unwrap(), &emote_name);

    let mut file = File::create(String::from(emote_path.clone().to_str().unwrap()) + &emote_name)
        .await
        .unwrap();

    file.write_all(&image).await.unwrap();

    println!("Se ha descargado la imagen en: {}{}", emote_path.to_str().unwrap(), emote_name);
}

async fn handle_response(
    response: Response,
    emote_id: &str,
    size: String,
) -> Option<(String, String)> {
    match response.status() {
        StatusCode::TOO_MANY_REQUESTS => {
            println!("Demasiadas peticiones a 7tv, espera un rato y vuelve a intentarlo");
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

            images_emote
                .iter()
                .find(|image| image.mime.contains("gif") && is_correct_scale(size.clone(), image))
                .or_else(|| {
                    images_emote
                        .iter()
                        .find(|image| image.mime.contains("png") && is_correct_scale(size.clone(), image))
                })
                .map(|image| (emote_name.clone(), image.url.clone()))
        }
        _ => {
            println!("No se ha podido obtener una respuesta.");
            None
        }
    }
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

fn build_one_emote_body(id_emote: String) -> QueryOneEmote {
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
