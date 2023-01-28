///
/// supplied schema
///
use serde::Serialize;

///
/// payload v1 with schema definition
///
#[derive(Clone, Serialize, Debug)]
pub struct Payload {
    pub id: String,
    pub timestamp: String,
    pub data: String,
}
