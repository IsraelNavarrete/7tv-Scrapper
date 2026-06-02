use crate::download;
use crate::seven_tv::emote_search_request_parameters::EmoteSearchVariables;
use crate::seven_tv::one_emote_request_parameters::build_one_emote_body;
use crate::seven_tv::response_one_emote::handle_singe_emote_response;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;
use std::time::Duration;

#[derive(Serialize)]
struct QueryEmoteSearch {
    operation_name: String,
    query: String,
    variables: EmoteSearchVariables,
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
            .header("host", "api.seven_tv.app")
            .json(&body)
            .send()
            .await?;

        println!("Respuesta recibida: {:#?}", response);

        let emote_data = handle_singe_emote_response(response, &emote_id, size).await;

        download::download_image(emote_data, self.client.clone()).await;

        Ok(())
    }


}


