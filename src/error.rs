
#[derive(Debug)]
pub enum StapleError {
    CanNotOperateDotRenderFolder,
    IoError(std::io::Error),
    ConfigError(toml::de::Error),
    RenderError(tera::Error)
}