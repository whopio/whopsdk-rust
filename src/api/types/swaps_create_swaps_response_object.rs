pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateSwapsResponseObject {
    #[serde(rename = "swap")]
    Swap,
}
impl fmt::Display for CreateSwapsResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Swap => "swap",
        };
        write!(f, "{}", s)
    }
}
