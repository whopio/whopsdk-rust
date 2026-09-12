pub use crate::prelude::*;

/// The certification this entry describes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdatePreferencesResponseAdsCertificationsItemCertificationType {
    #[serde(rename = "prescription_drug_ads")]
    PrescriptionDrugAds,
}
impl fmt::Display for UpdatePreferencesResponseAdsCertificationsItemCertificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PrescriptionDrugAds => "prescription_drug_ads",
        };
        write!(f, "{}", s)
    }
}
