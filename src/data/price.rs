use crate::data::currency::Currency;
use std::fmt;

pub struct Price {
    pub currency: Currency,
    pub min: f64,
    pub max: f64,
    pub confidence: f64,
}

impl fmt::Display for Price {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{:.2} - {:.2} {}\nConfidence: {:.1}%",
            self.min, self.max, self.currency, self.confidence
        )
    }
}
