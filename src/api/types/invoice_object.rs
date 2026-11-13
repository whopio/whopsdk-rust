pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InvoiceObject {
    #[serde(rename = "invoice")]
    Invoice,
}
impl fmt::Display for InvoiceObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Invoice => "invoice",
        };
        write!(f, "{}", s)
    }
}
