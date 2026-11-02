pub use crate::prelude::*;

/// The certification this entry describes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RetrievePreferencesResponseAdsCertificationsItemCertificationType {
    #[serde(rename = "prescription_drug_ads")]
    PrescriptionDrugAds,
}
impl fmt::Display for RetrievePreferencesResponseAdsCertificationsItemCertificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PrescriptionDrugAds => "prescription_drug_ads",
        };
        write!(f, "{}", s)
    }
}
