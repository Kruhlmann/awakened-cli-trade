use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct PoePricesPayload {
    pub min: f64,
    pub max: f64,
    pub currency: String,
    pub warning_msg: String,
    pub error: i64,
    pub pred_explanation: Vec<(String, f64)>,
    pub pred_confidence_score: f64,
    pub error_msg: String,
}

impl fmt::Display for PoePricesPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
