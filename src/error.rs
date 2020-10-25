#[derive(Debug, thiserror::Error)]
pub enum SlippiError {
    #[error("The character with id {0} is unknown")]
    UnknownCharacter(u8),
    #[error("The move with id {0} is unknown")]
    UnknownMove(u8),
    #[error("The stage with id {0} is unknown")]
    UnknownStage(u8),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type SlippiResult<T> = Result<T, SlippiError>;
