use {
    anyhow::Result,
    lazy_regex::*,
    std::{
        num,
        str::FromStr,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub width: u32,
    pub height: u32,
}

impl FromStr for Rect {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, w, h) = regex_captures!(r#"^(\d+)(?:(?:x|,|-)(\d+))?$"#, s)
            .ok_or_else(|| "Invalid format".to_string())?;
        let width = w.parse().map_err(|e: num::ParseIntError| e.to_string())?;
        let height = if h.is_empty() {
            width
        } else {
            h.parse().map_err(|e: num::ParseIntError| e.to_string())?
        };
        if width == 0 || height == 0 {
            return Err("Empty rect".to_string());
        }
        Ok(Self { width, height })
    }
}
