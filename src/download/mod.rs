use std::path::PathBuf;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::seven_tv::EmoteImage;

pub(crate) async fn download_image(emote_data: Option<(String, String)>, client: Client) {

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

    let emote_path = create_emote_save_path();

    println!("Descargando en ruta: {}{}", emote_path.clone().to_str().unwrap(), &emote_name);

    let mut file = File::create(String::from(emote_path.clone().to_str().unwrap()) + &emote_name)
        .await
        .unwrap();

    file.write_all(&image).await.unwrap();

    println!("Se ha descargado la imagen en: {}{}", emote_path.to_str().unwrap(), emote_name);
}

pub(crate) fn filter_correct_image(
    size: String,
    emote_name: String,
    images_emote: Vec<EmoteImage>,
) -> Option<(String, String)> {
    images_emote
        .iter()
        .find(|&image| has_gif(image, &size))
        .or_else(|| has_png(size, &images_emote))
        .map(|image| (emote_name.clone(), image.url.clone()))
}

fn has_png(size: String, images_emote: &Vec<EmoteImage>) -> Option<&EmoteImage> {
    images_emote
        .iter()
        .find(|image| image.mime.contains("png") && is_correct_scale(size.clone(), image))
}

fn has_gif(image: &EmoteImage, size: &String) -> bool {
    image.mime.contains("gif") && is_correct_scale(size.clone(), image)
}

fn is_correct_scale(size: String, image: &EmoteImage) -> bool {
    image.scale
        == size
        .chars()
        .next()
        .unwrap()
        .to_string()
        .parse::<i64>()
        .unwrap()
}

fn create_emote_save_path() -> PathBuf {
    let mut emote_path = std::env::current_dir().unwrap();

    emote_path.push("/emotes/");

    std::fs::create_dir_all(&emote_path).unwrap();
    emote_path
}