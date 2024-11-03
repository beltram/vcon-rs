pub type VconResult<T> = Result<T, VconError>;

#[derive(Debug, thiserror::Error)]
pub enum VconError {}
