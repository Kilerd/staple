use crate::{data::PageInfo, error::StapleError};
use std::path::Path;

pub(crate) mod json;
pub(crate) mod markdown;

pub struct CreationOptions {
    pub title: String,
    pub url: String,
    pub template: String,
    pub draw: bool,
}

pub trait FileType {
    type Output;
    fn load(file: impl AsRef<Path>) -> Result<Self::Output, StapleError>;
    fn create(file: impl AsRef<Path>, options: &CreationOptions) -> Result<(), StapleError>;
    fn into_page_info(self) -> PageInfo;
}
