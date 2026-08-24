pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdateAdGroupsRequestPlacementsZero {
    #[serde(rename = "automatic")]
    Automatic,
}
impl fmt::Display for UpdateAdGroupsRequestPlacementsZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Automatic => "automatic",
        };
        write!(f, "{}", s)
    }
}
