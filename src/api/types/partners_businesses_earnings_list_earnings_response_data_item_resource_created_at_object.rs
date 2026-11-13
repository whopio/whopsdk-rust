pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemResourceCreatedAtObject {
    #[serde(rename = "commission")]
    Commission,
}
impl fmt::Display for ListEarningsResponseDataItemResourceCreatedAtObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Commission => "commission",
        };
        write!(f, "{}", s)
    }
}
