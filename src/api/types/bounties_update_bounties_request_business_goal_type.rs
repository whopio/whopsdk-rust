pub use crate::prelude::*;

/// What the poster wants the work to achieve, declared once here.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateBountiesRequestBusinessGoalType {
    Clipping,
    PostEngagement,
    OwnedAccountGrowth,
    UgcContent,
    LocalActivation,
    DataCapture,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateBountiesRequestBusinessGoalType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Clipping => serializer.serialize_str("clipping"),
            Self::PostEngagement => serializer.serialize_str("post_engagement"),
            Self::OwnedAccountGrowth => serializer.serialize_str("owned_account_growth"),
            Self::UgcContent => serializer.serialize_str("ugc_content"),
            Self::LocalActivation => serializer.serialize_str("local_activation"),
            Self::DataCapture => serializer.serialize_str("data_capture"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateBountiesRequestBusinessGoalType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "clipping" => Ok(Self::Clipping),
            "post_engagement" => Ok(Self::PostEngagement),
            "owned_account_growth" => Ok(Self::OwnedAccountGrowth),
            "ugc_content" => Ok(Self::UgcContent),
            "local_activation" => Ok(Self::LocalActivation),
            "data_capture" => Ok(Self::DataCapture),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateBountiesRequestBusinessGoalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clipping => write!(f, "clipping"),
            Self::PostEngagement => write!(f, "post_engagement"),
            Self::OwnedAccountGrowth => write!(f, "owned_account_growth"),
            Self::UgcContent => write!(f, "ugc_content"),
            Self::LocalActivation => write!(f, "local_activation"),
            Self::DataCapture => write!(f, "data_capture"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
