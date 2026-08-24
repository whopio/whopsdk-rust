pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListBusinessesResponseDataItemObject {
    #[serde(rename = "partner_business")]
    PartnerBusiness,
}
impl fmt::Display for ListBusinessesResponseDataItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PartnerBusiness => "partner_business",
        };
        write!(f, "{}", s)
    }
}
