pub use crate::prelude::*;

/// The financing provider.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinancingProviderProvider {
    AfterpayClearpay,
    Zip,
    Sezzle,
    Tabby,
    Tamara,
    Kueski,
    Addi,
    Klarna,
    Oney,
    Splitit,
    Claritypay,
    FlexPay,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinancingProviderProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AfterpayClearpay => serializer.serialize_str("afterpay_clearpay"),
            Self::Zip => serializer.serialize_str("zip"),
            Self::Sezzle => serializer.serialize_str("sezzle"),
            Self::Tabby => serializer.serialize_str("tabby"),
            Self::Tamara => serializer.serialize_str("tamara"),
            Self::Kueski => serializer.serialize_str("kueski"),
            Self::Addi => serializer.serialize_str("addi"),
            Self::Klarna => serializer.serialize_str("klarna"),
            Self::Oney => serializer.serialize_str("oney"),
            Self::Splitit => serializer.serialize_str("splitit"),
            Self::Claritypay => serializer.serialize_str("claritypay"),
            Self::FlexPay => serializer.serialize_str("flex_pay"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinancingProviderProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "zip" => Ok(Self::Zip),
            "sezzle" => Ok(Self::Sezzle),
            "tabby" => Ok(Self::Tabby),
            "tamara" => Ok(Self::Tamara),
            "kueski" => Ok(Self::Kueski),
            "addi" => Ok(Self::Addi),
            "klarna" => Ok(Self::Klarna),
            "oney" => Ok(Self::Oney),
            "splitit" => Ok(Self::Splitit),
            "claritypay" => Ok(Self::Claritypay),
            "flex_pay" => Ok(Self::FlexPay),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinancingProviderProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AfterpayClearpay => write!(f, "afterpay_clearpay"),
            Self::Zip => write!(f, "zip"),
            Self::Sezzle => write!(f, "sezzle"),
            Self::Tabby => write!(f, "tabby"),
            Self::Tamara => write!(f, "tamara"),
            Self::Kueski => write!(f, "kueski"),
            Self::Addi => write!(f, "addi"),
            Self::Klarna => write!(f, "klarna"),
            Self::Oney => write!(f, "oney"),
            Self::Splitit => write!(f, "splitit"),
            Self::Claritypay => write!(f, "claritypay"),
            Self::FlexPay => write!(f, "flex_pay"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
