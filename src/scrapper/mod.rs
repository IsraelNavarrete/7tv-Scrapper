use crate::download;
use crate::seven_tv::emote_search_request_parameters::build_emote_search_body;
use crate::seven_tv::one_emote_request_parameters::build_one_emote_body;
use crate::seven_tv::response_one_emote::handle_single_emote_response;
use crate::seven_tv::response_page_search::handle_page_search_response;
use rand::RngExt;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

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

        let response = self
            .client
            .post(self.base_7tv_url.clone())
            .header("content-length", content_length)
            .header("host", "api.seven_tv.app")
            .json(&body)
            .send()
            .await?;

        println!("Respuesta recibida: {:#?}", response);

        let emote_data = handle_single_emote_response(response, &emote_id, size).await;

        download::download_image(emote_data, self.client.clone()).await;

        Ok(())
    }
    
    pub async fn download_emote_page(&self, filter: String, page: u32) -> Result<(), reqwest::Error>{
        
        println!("Descargando página {} filtrando por: {}",page,filter);
        
        let body = build_emote_search_body(filter.clone(), page);

        let content_length = json!(body).to_string().bytes().len();

        let response = self
            .client
            .post(self.base_7tv_url.clone())
            .header("content-length", content_length)
            .header("host", "api.seven_tv.app")
            .json(&body)
            .send()
            .await?;

        println!("Respuesta recibida: {:#?}", response);

        let emotes_data = handle_page_search_response(response, filter, page).await;

        for emote_data in emotes_data{
            let mut rng = rand::rng();
            let random_duration = rng.random_range(1..=2);

            download::download_image(emote_data, self.client.clone()).await;

            println!("Esperando {} segundos antes de hacer otra petición...",random_duration);

            sleep(Duration::from_secs(random_duration)).await;
        }
        
        Ok(())
    }


}


