use thiserror::Error;

#[derive(Error, Debug)]
pub enum StapleError {
    #[error("cannot operate folder .render")]
    CanNotOperateDotRenderFolder,

    #[error("io error {:?} {}", .0.kind(), .0.to_string())]
    IoError(#[from] std::io::Error),

    #[error("config error {}", .0.to_string())]
    ConfigError(#[from] toml::de::Error),

    #[error("render error {}", .0.to_string())]
    RenderError(#[from] tera::Error),

    #[error("theme '{0}' does not exist")]
    ThemeNotFound(String),

    #[error("error on loading article {filename} : {reason}")]
    ArticleError { filename: String, reason: String },
}
