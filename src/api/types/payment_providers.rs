pub use crate::prelude::*;

/// The different payment providers.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentProviders {
    Stripe,
    Coinbase,
    Paypal,
    Apple,
    Sezzle,
    Splitit,
    PlatformBalance,
    MultiPsp,
    Adyen,
    Claritypay,
    FlexPay,
    CheckoutDotCom,
    Airwallex,
    Coinflow,
    Sequra,
    Dlocal,
    Masspay,
    Braintree,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentProviders {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Stripe => serializer.serialize_str("stripe"),
            Self::Coinbase => serializer.serialize_str("coinbase"),
            Self::Paypal => serializer.serialize_str("paypal"),
            Self::Apple => serializer.serialize_str("apple"),
            Self::Sezzle => serializer.serialize_str("sezzle"),
            Self::Splitit => serializer.serialize_str("splitit"),
            Self::PlatformBalance => serializer.serialize_str("platform_balance"),
            Self::MultiPsp => serializer.serialize_str("multi_psp"),
            Self::Adyen => serializer.serialize_str("adyen"),
            Self::Claritypay => serializer.serialize_str("claritypay"),
            Self::FlexPay => serializer.serialize_str("flex_pay"),
            Self::CheckoutDotCom => serializer.serialize_str("checkout_dot_com"),
            Self::Airwallex => serializer.serialize_str("airwallex"),
            Self::Coinflow => serializer.serialize_str("coinflow"),
            Self::Sequra => serializer.serialize_str("sequra"),
            Self::Dlocal => serializer.serialize_str("dlocal"),
            Self::Masspay => serializer.serialize_str("masspay"),
            Self::Braintree => serializer.serialize_str("braintree"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentProviders {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "stripe" => Ok(Self::Stripe),
            "coinbase" => Ok(Self::Coinbase),
            "paypal" => Ok(Self::Paypal),
            "apple" => Ok(Self::Apple),
            "sezzle" => Ok(Self::Sezzle),
            "splitit" => Ok(Self::Splitit),
            "platform_balance" => Ok(Self::PlatformBalance),
            "multi_psp" => Ok(Self::MultiPsp),
            "adyen" => Ok(Self::Adyen),
            "claritypay" => Ok(Self::Claritypay),
            "flex_pay" => Ok(Self::FlexPay),
            "checkout_dot_com" => Ok(Self::CheckoutDotCom),
            "airwallex" => Ok(Self::Airwallex),
            "coinflow" => Ok(Self::Coinflow),
            "sequra" => Ok(Self::Sequra),
            "dlocal" => Ok(Self::Dlocal),
            "masspay" => Ok(Self::Masspay),
            "braintree" => Ok(Self::Braintree),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentProviders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stripe => write!(f, "stripe"),
            Self::Coinbase => write!(f, "coinbase"),
            Self::Paypal => write!(f, "paypal"),
            Self::Apple => write!(f, "apple"),
            Self::Sezzle => write!(f, "sezzle"),
            Self::Splitit => write!(f, "splitit"),
            Self::PlatformBalance => write!(f, "platform_balance"),
            Self::MultiPsp => write!(f, "multi_psp"),
            Self::Adyen => write!(f, "adyen"),
            Self::Claritypay => write!(f, "claritypay"),
            Self::FlexPay => write!(f, "flex_pay"),
            Self::CheckoutDotCom => write!(f, "checkout_dot_com"),
            Self::Airwallex => write!(f, "airwallex"),
            Self::Coinflow => write!(f, "coinflow"),
            Self::Sequra => write!(f, "sequra"),
            Self::Dlocal => write!(f, "dlocal"),
            Self::Masspay => write!(f, "masspay"),
            Self::Braintree => write!(f, "braintree"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
