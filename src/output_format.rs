use {
    image::{
        ImageOutputFormat,
    },
};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Png,
    Jpeg(u8),
}

impl OutputFormat {
    pub fn ext(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg(_) => "jpeg",
        }
    }
    pub fn image_format(self) -> ImageOutputFormat {
        match self {
            Self::Png => ImageOutputFormat::Png,
            Self::Jpeg(q) => ImageOutputFormat::Jpeg(q),
        }
    }
}

