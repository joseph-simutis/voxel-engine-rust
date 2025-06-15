use std::fmt;
use std::error::Error;
use crate::common::*;

#[derive(Debug)]
pub enum DualError<A: fmt::Debug + fmt::Display + Clone, B: fmt::Debug + fmt::Display + Clone> {
    Left(A),
    Right(B),
}
#[derive(Debug, Clone)]
pub struct InvalidLevelError {
    pub level: GlobalIdentifier,
}
#[derive(Debug, Clone)]
pub struct InvalidLevelCoordinatesError {
    pub coords: LevelCoordinates,
    pub level: GlobalIdentifier,
}
#[derive(Debug, Clone)]
pub struct InvalidChunkCoordinatesError {
    pub coords: ChunkCoordinates
}
#[derive(Debug, Clone)]
pub struct InvalidRelativeCoordinatesError {
    pub coords: RelativeCoordinates
}

impl<A: fmt::Debug + fmt::Display + Clone, B: fmt::Debug + fmt::Display + Clone> fmt::Display for DualError<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DualError::Left(err) => fmt::Display::fmt(err, f),
            DualError::Right(err) => fmt::Display::fmt(err, f),
        }
    }
}
impl fmt::Display for InvalidLevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid level {}", self.level)
    }
}
impl fmt::Display for InvalidLevelCoordinatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid level coordinates: {} in level {}", self.coords, self.level)
    }
}
impl fmt::Display for InvalidChunkCoordinatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid chunk coordinates: {}", self.coords)
    }
}
impl fmt::Display for InvalidRelativeCoordinatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid relative coordinates: {}", self.coords)
    }
}