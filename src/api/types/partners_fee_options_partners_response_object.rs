pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FeeOptionsPartnersResponseObject {
    #[serde(rename = "partner_fee_options")]
    PartnerFeeOptions,
}
impl fmt::Display for FeeOptionsPartnersResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PartnerFeeOptions => "partner_fee_options",
        };
        write!(f, "{}", s)
    }
}
