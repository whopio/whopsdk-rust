pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateAudiencesResponse {
    Audience(Audience),

    CreateAudiencesResponseData(CreateAudiencesResponseData),
}

impl CreateAudiencesResponse {
    pub fn is_audience(&self) -> bool {
        matches!(self, Self::Audience(_))
    }

    pub fn is_create_audiences_response_data(&self) -> bool {
        matches!(self, Self::CreateAudiencesResponseData(_))
    }

    pub fn as_audience(&self) -> Option<&Audience> {
        match self {
            Self::Audience(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_audience(self) -> Option<Audience> {
        match self {
            Self::Audience(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_audiences_response_data(&self) -> Option<&CreateAudiencesResponseData> {
        match self {
            Self::CreateAudiencesResponseData(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_audiences_response_data(self) -> Option<CreateAudiencesResponseData> {
        match self {
            Self::CreateAudiencesResponseData(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateAudiencesResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Audience(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreateAudiencesResponseData(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
