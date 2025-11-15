use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressWithTag {
    address: String,
    address_tag: String,
}

impl AddressWithTag {
    pub fn new(address: impl Into<String>, address_tag: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            address_tag: address_tag.into(),
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn address_tag(&self) -> &str {
        &self.address_tag
    }
}

impl fmt::Display for AddressWithTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AddressWithTag {{ address: {}, address_tag: {} }}",
            self.address, self.address_tag
        )
    }
}

#[derive(Default)]
pub struct AddressWithTagBuilder {
    address: Option<String>,
    address_tag: Option<String>,
}

impl AddressWithTagBuilder {
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn address_tag(mut self, address_tag: impl Into<String>) -> Self {
        self.address_tag = Some(address_tag.into());
        self
    }

    pub fn build(self) -> AddressWithTag {
        AddressWithTag {
            address: self.address.expect("address must be set"),
            address_tag: self.address_tag.unwrap_or_default(),
        }
    }
}
