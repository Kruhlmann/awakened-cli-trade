use std::fmt;

pub struct Item {
    string_representation: String,
}

impl Item {
    pub fn new(string_representation: String) -> Item {
        Item {
            string_representation: base64::encode(string_representation),
        }
    }

    pub fn as_base64(&self) -> String {
        self.string_representation.clone()
    }
}

impl fmt::Display for Item {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Item<{}>", self.string_representation)
    }
}
