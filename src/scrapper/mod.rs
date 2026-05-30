use std::io;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::emote_values::{Format, Size};

pub async fn download_emote(
    url: String,
    size: Option<String>,
    file_format: Option<String>,
) -> io::Result<()> {
    let size = size.unwrap_or(Size::SIZE4.to_string());
    let file_format = file_format.unwrap_or(Format::WEBP.to_string());

    let image_url = get_image_url(url.clone(), size, file_format.clone());

    println!("Downloading image from: {image_url}");

    let response = reqwest::get(&image_url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let mut image_file = File::create(Path::new(&get_file_path(url, file_format))).await?;

    image_file.write_all(&response).await?;

    println!("Downloaded image: {image_url}");

    Ok(())
}

fn get_file_path(base_url: String, file_format: String) -> String {

    let path = String::from("C:/Users/tingl/RustroverProjects/EmotesScrapper/emotes/")
        + base_url.split('/').last().unwrap()
        + &file_format;

    println!("Ruta donde se va a guardar el emote: {:?}", path);

    return path;
}

fn get_image_url(base_url: String, image_size: String, file_format: String) -> String {
    String::from(base_url)
        .replace("7tv.app", "cdn.7tv.app")
        .replace("emotes", "emote")
        + "/"
        + &image_size
        + &file_format
}
