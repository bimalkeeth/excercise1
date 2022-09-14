mod tests;
mod main2;

use log::info;
use thiserror::Error;

#[derive(Debug,Error)]
#[non_exhaustive]
pub enum PuzzleError {
    #[error("Piece {0} doesn't fit")]
    WontFit(u16),
    #[error("Missing piece")]
    MissingPiece,
}

pub fn snuggle(bunnies:u64) -> u64{
    bunnies * 8
}

fn main() {
    env_logger::init();
    info!("Hello, world!");
}

