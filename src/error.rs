

#[derive(Debug)]
pub enum StapleError {
    CanNotOperateDotRenderFolder,
    IoError(std::io::Error),
}