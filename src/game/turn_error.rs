use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
pub enum TurnError {
    #[error("game is already finished")]
    GameFinished,
    #[error("cell is already occupied")]
    CellOccupied,
}
