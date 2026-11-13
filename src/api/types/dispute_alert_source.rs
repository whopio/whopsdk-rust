pub use crate::prelude::*;

/// The network program that sent the alert, which `type` alone doesn't tell apart. `ethoca` for Mastercard Ethoca, `cdrn` for Visa Verifi CDRN, `rdr` for Visa RDR, `tc40` / `safe` for a Visa / Mastercard fraud report, `paypal` for a PayPal alert. `null` when the network didn't say.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeAlertSource {
    Ethoca,
    Cdrn,
    Rdr,
    Tc40,
    Safe,
    Paypal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeAlertSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ethoca => serializer.serialize_str("ethoca"),
            Self::Cdrn => serializer.serialize_str("cdrn"),
            Self::Rdr => serializer.serialize_str("rdr"),
            Self::Tc40 => serializer.serialize_str("tc40"),
            Self::Safe => serializer.serialize_str("safe"),
            Self::Paypal => serializer.serialize_str("paypal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeAlertSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ethoca" => Ok(Self::Ethoca),
            "cdrn" => Ok(Self::Cdrn),
            "rdr" => Ok(Self::Rdr),
            "tc40" => Ok(Self::Tc40),
            "safe" => Ok(Self::Safe),
            "paypal" => Ok(Self::Paypal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeAlertSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ethoca => write!(f, "ethoca"),
            Self::Cdrn => write!(f, "cdrn"),
            Self::Rdr => write!(f, "rdr"),
            Self::Tc40 => write!(f, "tc40"),
            Self::Safe => write!(f, "safe"),
            Self::Paypal => write!(f, "paypal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
