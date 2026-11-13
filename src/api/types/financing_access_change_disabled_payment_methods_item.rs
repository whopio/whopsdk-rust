pub use crate::prelude::*;

/// BNPL payment methods disabled on this account.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinancingAccessChangeDisabledPaymentMethodsItem {
    Addi,
    Affirm,
    AfterpayClearpay,
    Alma,
    Billie,
    CapchasePay,
    Claritypay,
    FlexPay,
    Klarna,
    Kriya,
    Kueski,
    Mondu,
    Oney,
    Oney3X,
    Oney4X,
    Scalapay,
    Sequra,
    Sezzle,
    Splitit,
    Sunbit,
    Tabby,
    Tamara,
    Zip,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinancingAccessChangeDisabledPaymentMethodsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Addi => serializer.serialize_str("addi"),
            Self::Affirm => serializer.serialize_str("affirm"),
            Self::AfterpayClearpay => serializer.serialize_str("afterpay_clearpay"),
            Self::Alma => serializer.serialize_str("alma"),
            Self::Billie => serializer.serialize_str("billie"),
            Self::CapchasePay => serializer.serialize_str("capchase_pay"),
            Self::Claritypay => serializer.serialize_str("claritypay"),
            Self::FlexPay => serializer.serialize_str("flex_pay"),
            Self::Klarna => serializer.serialize_str("klarna"),
            Self::Kriya => serializer.serialize_str("kriya"),
            Self::Kueski => serializer.serialize_str("kueski"),
            Self::Mondu => serializer.serialize_str("mondu"),
            Self::Oney => serializer.serialize_str("oney"),
            Self::Oney3X => serializer.serialize_str("oney_3x"),
            Self::Oney4X => serializer.serialize_str("oney_4x"),
            Self::Scalapay => serializer.serialize_str("scalapay"),
            Self::Sequra => serializer.serialize_str("sequra"),
            Self::Sezzle => serializer.serialize_str("sezzle"),
            Self::Splitit => serializer.serialize_str("splitit"),
            Self::Sunbit => serializer.serialize_str("sunbit"),
            Self::Tabby => serializer.serialize_str("tabby"),
            Self::Tamara => serializer.serialize_str("tamara"),
            Self::Zip => serializer.serialize_str("zip"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinancingAccessChangeDisabledPaymentMethodsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "addi" => Ok(Self::Addi),
            "affirm" => Ok(Self::Affirm),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "alma" => Ok(Self::Alma),
            "billie" => Ok(Self::Billie),
            "capchase_pay" => Ok(Self::CapchasePay),
            "claritypay" => Ok(Self::Claritypay),
            "flex_pay" => Ok(Self::FlexPay),
            "klarna" => Ok(Self::Klarna),
            "kriya" => Ok(Self::Kriya),
            "kueski" => Ok(Self::Kueski),
            "mondu" => Ok(Self::Mondu),
            "oney" => Ok(Self::Oney),
            "oney_3x" => Ok(Self::Oney3X),
            "oney_4x" => Ok(Self::Oney4X),
            "scalapay" => Ok(Self::Scalapay),
            "sequra" => Ok(Self::Sequra),
            "sezzle" => Ok(Self::Sezzle),
            "splitit" => Ok(Self::Splitit),
            "sunbit" => Ok(Self::Sunbit),
            "tabby" => Ok(Self::Tabby),
            "tamara" => Ok(Self::Tamara),
            "zip" => Ok(Self::Zip),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinancingAccessChangeDisabledPaymentMethodsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Addi => write!(f, "addi"),
            Self::Affirm => write!(f, "affirm"),
            Self::AfterpayClearpay => write!(f, "afterpay_clearpay"),
            Self::Alma => write!(f, "alma"),
            Self::Billie => write!(f, "billie"),
            Self::CapchasePay => write!(f, "capchase_pay"),
            Self::Claritypay => write!(f, "claritypay"),
            Self::FlexPay => write!(f, "flex_pay"),
            Self::Klarna => write!(f, "klarna"),
            Self::Kriya => write!(f, "kriya"),
            Self::Kueski => write!(f, "kueski"),
            Self::Mondu => write!(f, "mondu"),
            Self::Oney => write!(f, "oney"),
            Self::Oney3X => write!(f, "oney_3x"),
            Self::Oney4X => write!(f, "oney_4x"),
            Self::Scalapay => write!(f, "scalapay"),
            Self::Sequra => write!(f, "sequra"),
            Self::Sezzle => write!(f, "sezzle"),
            Self::Splitit => write!(f, "splitit"),
            Self::Sunbit => write!(f, "sunbit"),
            Self::Tabby => write!(f, "tabby"),
            Self::Tamara => write!(f, "tamara"),
            Self::Zip => write!(f, "zip"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
