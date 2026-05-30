use std::fmt::Display;

#[derive(PartialEq, Clone)]
#[warn(dead_code)]
pub enum Size {
    SIZE1,
    SIZE2,
    SIZE3,
    SIZE4,
}

impl Display for Size{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Size::SIZE1 => "1x".to_string(),
            Size::SIZE2 => "2x".to_string(),
            Size::SIZE3 => "3x".to_string(),
            Size::SIZE4 => "4x".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(PartialEq, Clone)]
#[warn(dead_code)]
pub enum Format {
    WEBP,
    PNG,
    GIF,
    AVIF,
}


impl Display for Format{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Format::WEBP => ".webp".to_string(),
            Format::PNG => ".png".to_string(),
            Format::GIF => ".gif".to_string(),
            Format::AVIF => ".avif".to_string(),
        };
        write!(f, "{}", str)
    }
}