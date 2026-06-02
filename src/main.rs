mod command_values;
mod scrapper;
mod download;
mod seven_tv;

use crate::scrapper::Scrapper;
use command_values::Size;
use std::cmp::PartialEq;
use std::io;
use std::str::FromStr;
use crate::seven_tv::emote_search_request_parameters::Sort;

#[derive(PartialEq, Clone, Copy)]
enum Command {
    EMOTE,
    EMOTES,
    SALIR,
    OTRO,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EMOTE" => Ok(Command::EMOTE),
            "EMOTES" => Ok(Command::EMOTES),
            "SALIR" => Ok(Command::SALIR),
            _ => Ok(Command::OTRO),
        }
    }
}

#[tokio::main]
async fn main() {
    print_info();

    let scrapper = Scrapper::new(String::from("https://api.7tv.app/v4/gql")).unwrap();

    loop {
        let mut line = String::new();

        io::stdin().read_line(&mut line).unwrap();

        if line.is_empty() || line == "\r\n" {
            println!("Añade un comando");
        } else {

            let command_parts: Vec<&str> = line.split_whitespace().collect();

            let command = Command::from_str(&command_parts[0]).unwrap();

            if is_valid_command_or_finish(command) {
                return;
            }

            download_emote_command(command_parts.clone(), command, scrapper.clone()).await;
            //download_emotes_command(command_parts, command).await;
        }
    }
}

async fn download_emote_command(command_parts: Vec<&str>, command: Command, scrapper: Scrapper) {
    if command == Command::EMOTE {
        let (url, size) = set_emote_command_parameters(command_parts);

        if url.is_empty() {
            println!("Necesito como mínimo la URL del emote");
            return;
        }

        let download_emote_result =
            Scrapper::download_single_emote(&scrapper, url, get_valid_size(size)).await;

        let result = match download_emote_result {
            Ok(_result) => "La descarga ha terminado correctamente",

            Err(error) => {
                &(String::from("Se ha producido un error: ") + error.to_string().as_str())
            }
        };

        println!("{}", result);
    }
}

fn set_emotes_command_parameters(command_parts: Vec<&str>) -> (String, u32) {
    let (filtro, numero_pagina) = match command_parts.len() {
        2 => (String::from(command_parts[1]), 1),
        3 => (
            String::from(command_parts[1]),
            String::from(command_parts[2]).parse::<u32>().unwrap(),
        ),
        _ => (Sort::TOPALLTIME.to_string(), 1),
    };
    (filtro, numero_pagina)
}

fn set_emote_command_parameters(command_parts: Vec<&str>) -> (String, String) {
    let (url, size) = match command_parts.len() {
        2 => (String::from(command_parts[1]), Size::SIZE4.to_string()),
        3 => (
            String::from(command_parts[1]),
            String::from(command_parts[2]),
        ),
        4 => (
            String::from(command_parts[1]),
            String::from(command_parts[2]),
        ),
        _ => (String::new(), Size::SIZE4.to_string()),
    };
    (url, size)
}

fn get_valid_size(size: String) -> String {
    if size.is_empty() {
        return String::from("4x");
    }

    size
}

fn print_info() {
    println!("Ala monstrou, esto es pa descargar emotes de seven_tv.");
    println!("los comando son:");
    println!("\"EMOTE\" URL del emote en seven_tv (obligatorio) y tamaño (1x,2x,3x,4x)).");
    println!(
        "\"EMOTES\" Filtro (Populares,Tendencias,Nuevo [Por defecto populares]) numero de la pagina."
    );
    println!("\"SALIR\"");
}

fn is_valid_command_or_finish(command: Command) -> bool {
    if command == Command::OTRO {
        println!("Ese comando no es válido, solo EMOTE, EMOTES y SALIR");
    }

    if command == Command::SALIR {
        return true;
    }

    false
}
