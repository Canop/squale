use {
    crate::{
        args::*,
        output_format::*,
    },
    anyhow::Result,
    image::{
        io::Reader as ImageReader,
        imageops::FilterType,
    },
    lazy_regex::*,
    std::{
        fs::File,
        path::PathBuf,
    },
};

#[derive(Debug)]
pub struct Scaler {
    width: u32,
    height: u32,
    verb: Verbosity,
    jpeg_quality: u8,
}

impl Scaler {
    pub fn new(args: &Args) -> Result<Self> {
        Ok(Self {
            width: args.fit.width,
            height: args.fit.height,
            verb: args.verb,
            jpeg_quality: args.jpeg_quality,
        })
    }
    /// Return None if we can't handle this kind of file
    fn output_format(&self, ext: &str) -> Option<OutputFormat> {
        if regex_is_match!("^(jpe?g|tiff?)$"i, ext) {
            return Some(OutputFormat::Jpeg(self.jpeg_quality));
        }
        if regex_is_match!("^(png|ico|bmp)$"i, ext) {
            return Some(OutputFormat::Png);
        }
        None
    }
    pub fn handle(&self, sources: &[PathBuf]) -> Result<()> {
        for src_path in sources {
            if self.verb>=Verbosity::VeryVerbose {
                eprintln!("Looking at {src_path:?} ...");
            }
            if !src_path.is_file() {
                if self.verb>=Verbosity::VeryVerbose {
                    eprintln!("Excluding not-file {src_path:?}");
                }
                continue;
            }
            let Some(dir) = src_path.parent() else { continue };
            let Some(stem) = src_path
                .file_stem()
                .and_then(|oss| oss.to_str())
                else { continue };
            let Some(output_format) = src_path
                .extension()
                .and_then(|oss| oss.to_str())
                .and_then(|ext| self.output_format(ext))
                else {
                    if self.verb>=Verbosity::Verbose {
                        eprintln!("Excluding non image file {src_path:?}");
                    }
                    continue;
                };
            if regex_is_match!(r#"-\d+x\d+$"#, stem) {
                if self.verb>=Verbosity::Verbose {
                    eprintln!("Excluding output looking file {src_path:?}");
                }
                continue;
            }
            let src = ImageReader::open(src_path)?.decode()?;
            if self.verb>=Verbosity::Verbose {
                eprintln!("Input image is {}x{}", src.width(), src.height());
            }
            if src.width() == self.width {
                if self.verb>=Verbosity::Verbose {
                    eprintln!("Excluding image already at target width {src_path:?}");
                }
                continue;
            }
            let mut dst_path = dir.to_path_buf();
            dst_path.push(format!(
                "{}-{}x{}.{}",
                stem,
                self.width,
                self.height,
                output_format.ext(),
            ));
            if dst_path.exists() {
                if self.verb>=Verbosity::Verbose {
                    eprintln!("Excluding image already scaled {src_path:?}");
                }
                continue;
            }
            let dst = src.resize(self.width, self.height, FilterType::Lanczos3);
            let mut dst_file = File::create(&dst_path)?;
            dst.write_to(&mut dst_file, output_format.image_format())?;
            if self.verb>=Verbosity::Normal {
                eprintln!("Wrote file {dst_path:?}");
            }
        }
        Ok(())
    }
}
