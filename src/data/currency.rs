use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum Currency {
    Chaos,
    Divine,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Currency {
    type Err = ();

    fn from_str(input: &str) -> Result<Currency, Self::Err> {
        match input {
            "chaos" => Ok(Currency::Chaos),
            "divine" => Ok(Currency::Divine),
            _ => Err(()),
        }
    }
}
