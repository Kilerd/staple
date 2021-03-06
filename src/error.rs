use thiserror::Error;

#[derive(Error, Debug)]
pub enum StapleError {
    #[error("`Staple.toml` does not exist, try to run `staple init` before.")]
    ConfigNotFound,

    #[error("io error {:?} {}", .0.kind(), .0.to_string())]
    IoError(#[from] std::io::Error),

    #[error("config error {}", .0.to_string())]
    ConfigError(#[from] toml::de::Error),

    #[error("render error {}", .0.to_string())]
    RenderError(#[from] tera::Error),

    #[error("error on loading article {filename} : {reason}")]
    ArticleError { filename: String, reason: String },

    #[error("error on parse url: {}", .0.to_string())]
    UrlParseError(#[from] url::ParseError),

    #[error("cannot serde json file: {0}")]
    JsonFileParseError(#[from] serde_json::Error),

    #[error("execute hook `{}` get non-zero exit code: {}", .0, .1.unwrap_or(-1))]
    HookError(String, Option<i32>),
}
