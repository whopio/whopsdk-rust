pub use crate::prelude::*;

/// The underlying event recorded. Every movement on this feed is a ledger line, so switch on `type` rather than this.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PulseEventsResponseDataItemEventName {
    #[serde(rename = "ledger_line.created")]
    LedgerLineCreated,
}
impl fmt::Display for PulseEventsResponseDataItemEventName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LedgerLineCreated => "ledger_line.created",
        };
        write!(f, "{}", s)
    }
}
