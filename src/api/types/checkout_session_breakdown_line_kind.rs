pub use crate::prelude::*;

/// What this line is — `plan` today. New kinds arrive as checkout learns to sell more than one thing at once.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBreakdownLineKind {
    #[serde(rename = "plan")]
    Plan,
}
impl fmt::Display for CheckoutSessionBreakdownLineKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Plan => "plan",
        };
        write!(f, "{}", s)
    }
}
