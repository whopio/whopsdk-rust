pub use crate::prelude::*;

/// The type of this object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FinancingAccessChangeObject {
    #[serde(rename = "financing_access_change")]
    FinancingAccessChange,
}
impl fmt::Display for FinancingAccessChangeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FinancingAccessChange => "financing_access_change",
        };
        write!(f, "{}", s)
    }
}
