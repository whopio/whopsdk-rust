pub use crate::prelude::*;

/// Legal entity ending appended to `business_name`. LLC formations accept `LLC`, `L.L.C`, `L.L.C.` or `Limited Liability Company` and default to `LLC`; C-Corp formations accept `Inc`, `Inc.`, `Incorporated`, `Corp.`, `Corporation`, `C Corp`, `C Corporation`, `CCorp` or `Company` and default to `Inc.`. Unrecognized values fall back to the default for the entity type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FormCompanyAccountsRequestEntitySuffix {
    Llc,
    LlcDotted,
    LlcDottedTrailingPeriod,
    LimitedLiabilityCompany,
    Inc,
    IncTrailingPeriod,
    Incorporated,
    Corp,
    Corporation,
    CCorp,
    CCorporation,
    CCorpOneWord,
    Company,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FormCompanyAccountsRequestEntitySuffix {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Llc => serializer.serialize_str("LLC"),
            Self::LlcDotted => serializer.serialize_str("L.L.C"),
            Self::LlcDottedTrailingPeriod => serializer.serialize_str("L.L.C."),
            Self::LimitedLiabilityCompany => serializer.serialize_str("Limited Liability Company"),
            Self::Inc => serializer.serialize_str("Inc"),
            Self::IncTrailingPeriod => serializer.serialize_str("Inc."),
            Self::Incorporated => serializer.serialize_str("Incorporated"),
            Self::Corp => serializer.serialize_str("Corp."),
            Self::Corporation => serializer.serialize_str("Corporation"),
            Self::CCorp => serializer.serialize_str("C Corp"),
            Self::CCorporation => serializer.serialize_str("C Corporation"),
            Self::CCorpOneWord => serializer.serialize_str("CCorp"),
            Self::Company => serializer.serialize_str("Company"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FormCompanyAccountsRequestEntitySuffix {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "LLC" => Ok(Self::Llc),
            "L.L.C" => Ok(Self::LlcDotted),
            "L.L.C." => Ok(Self::LlcDottedTrailingPeriod),
            "Limited Liability Company" => Ok(Self::LimitedLiabilityCompany),
            "Inc" => Ok(Self::Inc),
            "Inc." => Ok(Self::IncTrailingPeriod),
            "Incorporated" => Ok(Self::Incorporated),
            "Corp." => Ok(Self::Corp),
            "Corporation" => Ok(Self::Corporation),
            "C Corp" => Ok(Self::CCorp),
            "C Corporation" => Ok(Self::CCorporation),
            "CCorp" => Ok(Self::CCorpOneWord),
            "Company" => Ok(Self::Company),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FormCompanyAccountsRequestEntitySuffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Llc => write!(f, "LLC"),
            Self::LlcDotted => write!(f, "L.L.C"),
            Self::LlcDottedTrailingPeriod => write!(f, "L.L.C."),
            Self::LimitedLiabilityCompany => write!(f, "Limited Liability Company"),
            Self::Inc => write!(f, "Inc"),
            Self::IncTrailingPeriod => write!(f, "Inc."),
            Self::Incorporated => write!(f, "Incorporated"),
            Self::Corp => write!(f, "Corp."),
            Self::Corporation => write!(f, "Corporation"),
            Self::CCorp => write!(f, "C Corp"),
            Self::CCorporation => write!(f, "C Corporation"),
            Self::CCorpOneWord => write!(f, "CCorp"),
            Self::Company => write!(f, "Company"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
