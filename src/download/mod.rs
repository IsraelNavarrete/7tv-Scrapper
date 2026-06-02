use std::path::PathBuf;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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

fn create_emote_save_path() -> PathBuf {
    let mut emote_path = std::env::current_dir().unwrap();

    emote_path.push("/emotes/");

    std::fs::create_dir_all(&emote_path).unwrap();
    emote_path
}