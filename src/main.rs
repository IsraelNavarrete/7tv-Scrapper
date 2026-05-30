mod emote_values;
mod scrapper;

use emote_values::Format;
use emote_values::Size;
use std::cmp::PartialEq;
use std::io;
use std::str::FromStr;

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

            download_emote_command(command_parts, command).await
        }
    }
}

async fn download_emote_command(command_parts: Vec<&str>, command: Command) {
    
    if command == Command::EMOTE {

        let (url, size, emote_format) = set_command_parameters(command_parts);

        if url.is_empty() {
            println!("Necesito como mínimo la URL del emote");
            return;
        }

        let download_emote_result = scrapper::download_emote(
            url,
            Some(get_valid_size(size)),
            Some(get_valid_emote_format(emote_format)),
        )
        .await;

        let result = match download_emote_result {
            Ok(_result) => "La descarga ha terminado correctamente",

            Err(error) => {
                &(String::from("Se ha producido un error: ") + error.to_string().as_str())
            }
        };

        println!("{}", result);
    }
}

fn set_command_parameters(command_parts: Vec<&str>) -> (String, String, String) {
    let (url, size, emote_format) = match command_parts.len() {
        2 => (
            String::from(command_parts[1]),
            Size::SIZE4.to_string(),
            Format::WEBP.to_string(),
        ),
        3 => (
            String::from(command_parts[1]),
            String::from(command_parts[2]),
            Format::WEBP.to_string(),
        ),
        4 => (
            String::from(command_parts[1]),
            String::from(command_parts[2]),
            String::from(command_parts[3]),
        ),
        _ => (
            String::new(),
            Size::SIZE4.to_string(),
            Format::WEBP.to_string(),
        ),
    };
    (url, size, emote_format)
}

fn get_valid_size(size: String) -> String {
    if size.is_empty() {
        return String::from("4x");
    }

    size
}

fn get_valid_emote_format(emote_format: String) -> String {
    if emote_format.is_empty() {
        return String::from(Format::WEBP.to_string());
    }

    if !emote_format.starts_with('.') {
        return format!(".{}", emote_format);
    }

    String::from(emote_format)
}

fn print_info() {
    println!("Ala monstrou, esto es pa descargar emotes de 7tv.");
    println!("los comando son:");
    println!(
        "\"EMOTE\" URL del emote en 7tv (obligatorio) tamaño (1x,2x,3x,4x) y formato (webp [por defecto], png, avif y gif [usa este si sabes que es animado])."
    );
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
