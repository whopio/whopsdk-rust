pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RetrieveBusinessesResponseObject {
    #[serde(rename = "partner_business")]
    PartnerBusiness,
}
impl fmt::Display for RetrieveBusinessesResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PartnerBusiness => "partner_business",
        };
        write!(f, "{}", s)
    }
}
