
pub type Result<T> = core::result::Result<T, BikeError>;

#[derive(Debug)]
pub enum BikeError {
    InvalidArgument(&'static str),
    VecFull,
    VecEmpty,
    StackFull,
    StackEmpty,
    CannotDrawToTarget,
    Unknown,
}