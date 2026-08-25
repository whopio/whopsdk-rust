pub use crate::prelude::*;

/// Dated API version used when requests authenticated with this key omit the `Api-Version-Date` header.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApiKeyApiVersionDate {
    TwoThousandTwentyFive0101,
    TwoThousandTwentySix0608,
    TwoThousandTwentySix0609,
    TwoThousandTwentySix0620,
    TwoThousandTwentySix0701,
    TwoThousandTwentySix0708,
    TwoThousandTwentySix07081,
    TwoThousandTwentySix0718,
    TwoThousandTwentySix0720,
    TwoThousandTwentySix0722,
    TwoThousandTwentySix0723,
    TwoThousandTwentySix0725,
    TwoThousandTwentySix0726,
    TwoThousandTwentySix0727,
    TwoThousandTwentySix0729,
    TwoThousandTwentySix07291,
    TwoThousandTwentySix0731,
    TwoThousandTwentySix0803,
    TwoThousandTwentySix0805,
    TwoThousandTwentySix08051,
    TwoThousandTwentySix0810,
    TwoThousandTwentySix0812,
    TwoThousandTwentySix0813,
    TwoThousandTwentySix0814,
    TwoThousandTwentySix0821,
    TwoThousandTwentySix08211,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApiKeyApiVersionDate {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TwoThousandTwentyFive0101 => serializer.serialize_str("2025-01-01"),
            Self::TwoThousandTwentySix0608 => serializer.serialize_str("2026-06-08"),
            Self::TwoThousandTwentySix0609 => serializer.serialize_str("2026-06-09"),
            Self::TwoThousandTwentySix0620 => serializer.serialize_str("2026-06-20"),
            Self::TwoThousandTwentySix0701 => serializer.serialize_str("2026-07-01"),
            Self::TwoThousandTwentySix0708 => serializer.serialize_str("2026-07-08"),
            Self::TwoThousandTwentySix07081 => serializer.serialize_str("2026-07-08-1"),
            Self::TwoThousandTwentySix0718 => serializer.serialize_str("2026-07-18"),
            Self::TwoThousandTwentySix0720 => serializer.serialize_str("2026-07-20"),
            Self::TwoThousandTwentySix0722 => serializer.serialize_str("2026-07-22"),
            Self::TwoThousandTwentySix0723 => serializer.serialize_str("2026-07-23"),
            Self::TwoThousandTwentySix0725 => serializer.serialize_str("2026-07-25"),
            Self::TwoThousandTwentySix0726 => serializer.serialize_str("2026-07-26"),
            Self::TwoThousandTwentySix0727 => serializer.serialize_str("2026-07-27"),
            Self::TwoThousandTwentySix0729 => serializer.serialize_str("2026-07-29"),
            Self::TwoThousandTwentySix07291 => serializer.serialize_str("2026-07-29-1"),
            Self::TwoThousandTwentySix0731 => serializer.serialize_str("2026-07-31"),
            Self::TwoThousandTwentySix0803 => serializer.serialize_str("2026-08-03"),
            Self::TwoThousandTwentySix0805 => serializer.serialize_str("2026-08-05"),
            Self::TwoThousandTwentySix08051 => serializer.serialize_str("2026-08-05-1"),
            Self::TwoThousandTwentySix0810 => serializer.serialize_str("2026-08-10"),
            Self::TwoThousandTwentySix0812 => serializer.serialize_str("2026-08-12"),
            Self::TwoThousandTwentySix0813 => serializer.serialize_str("2026-08-13"),
            Self::TwoThousandTwentySix0814 => serializer.serialize_str("2026-08-14"),
            Self::TwoThousandTwentySix0821 => serializer.serialize_str("2026-08-21"),
            Self::TwoThousandTwentySix08211 => serializer.serialize_str("2026-08-21-1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApiKeyApiVersionDate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "2025-01-01" => Ok(Self::TwoThousandTwentyFive0101),
            "2026-06-08" => Ok(Self::TwoThousandTwentySix0608),
            "2026-06-09" => Ok(Self::TwoThousandTwentySix0609),
            "2026-06-20" => Ok(Self::TwoThousandTwentySix0620),
            "2026-07-01" => Ok(Self::TwoThousandTwentySix0701),
            "2026-07-08" => Ok(Self::TwoThousandTwentySix0708),
            "2026-07-08-1" => Ok(Self::TwoThousandTwentySix07081),
            "2026-07-18" => Ok(Self::TwoThousandTwentySix0718),
            "2026-07-20" => Ok(Self::TwoThousandTwentySix0720),
            "2026-07-22" => Ok(Self::TwoThousandTwentySix0722),
            "2026-07-23" => Ok(Self::TwoThousandTwentySix0723),
            "2026-07-25" => Ok(Self::TwoThousandTwentySix0725),
            "2026-07-26" => Ok(Self::TwoThousandTwentySix0726),
            "2026-07-27" => Ok(Self::TwoThousandTwentySix0727),
            "2026-07-29" => Ok(Self::TwoThousandTwentySix0729),
            "2026-07-29-1" => Ok(Self::TwoThousandTwentySix07291),
            "2026-07-31" => Ok(Self::TwoThousandTwentySix0731),
            "2026-08-03" => Ok(Self::TwoThousandTwentySix0803),
            "2026-08-05" => Ok(Self::TwoThousandTwentySix0805),
            "2026-08-05-1" => Ok(Self::TwoThousandTwentySix08051),
            "2026-08-10" => Ok(Self::TwoThousandTwentySix0810),
            "2026-08-12" => Ok(Self::TwoThousandTwentySix0812),
            "2026-08-13" => Ok(Self::TwoThousandTwentySix0813),
            "2026-08-14" => Ok(Self::TwoThousandTwentySix0814),
            "2026-08-21" => Ok(Self::TwoThousandTwentySix0821),
            "2026-08-21-1" => Ok(Self::TwoThousandTwentySix08211),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApiKeyApiVersionDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TwoThousandTwentyFive0101 => write!(f, "2025-01-01"),
            Self::TwoThousandTwentySix0608 => write!(f, "2026-06-08"),
            Self::TwoThousandTwentySix0609 => write!(f, "2026-06-09"),
            Self::TwoThousandTwentySix0620 => write!(f, "2026-06-20"),
            Self::TwoThousandTwentySix0701 => write!(f, "2026-07-01"),
            Self::TwoThousandTwentySix0708 => write!(f, "2026-07-08"),
            Self::TwoThousandTwentySix07081 => write!(f, "2026-07-08-1"),
            Self::TwoThousandTwentySix0718 => write!(f, "2026-07-18"),
            Self::TwoThousandTwentySix0720 => write!(f, "2026-07-20"),
            Self::TwoThousandTwentySix0722 => write!(f, "2026-07-22"),
            Self::TwoThousandTwentySix0723 => write!(f, "2026-07-23"),
            Self::TwoThousandTwentySix0725 => write!(f, "2026-07-25"),
            Self::TwoThousandTwentySix0726 => write!(f, "2026-07-26"),
            Self::TwoThousandTwentySix0727 => write!(f, "2026-07-27"),
            Self::TwoThousandTwentySix0729 => write!(f, "2026-07-29"),
            Self::TwoThousandTwentySix07291 => write!(f, "2026-07-29-1"),
            Self::TwoThousandTwentySix0731 => write!(f, "2026-07-31"),
            Self::TwoThousandTwentySix0803 => write!(f, "2026-08-03"),
            Self::TwoThousandTwentySix0805 => write!(f, "2026-08-05"),
            Self::TwoThousandTwentySix08051 => write!(f, "2026-08-05-1"),
            Self::TwoThousandTwentySix0810 => write!(f, "2026-08-10"),
            Self::TwoThousandTwentySix0812 => write!(f, "2026-08-12"),
            Self::TwoThousandTwentySix0813 => write!(f, "2026-08-13"),
            Self::TwoThousandTwentySix0814 => write!(f, "2026-08-14"),
            Self::TwoThousandTwentySix0821 => write!(f, "2026-08-21"),
            Self::TwoThousandTwentySix08211 => write!(f, "2026-08-21-1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
