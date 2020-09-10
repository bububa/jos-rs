use crate::client::Error;
use serde::de::DeserializeOwned;

pub trait Response {
    type ResultType: DeserializeOwned;
    fn result(&mut self) -> Result<Self::ResultType, Error>;
}
