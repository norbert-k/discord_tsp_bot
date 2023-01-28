use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error(transparent)]
    SerenityError(#[from] serenity::Error),
    #[error(transparent)]
    CliError(#[from] clap::Error),
    #[error(transparent)]
    Eyre(#[from] color_eyre::eyre::ErrReport),
    #[error("unknown error")]
    Unknown,
}

pub type Result<T> = color_eyre::eyre::Result<T, ApplicationError>;