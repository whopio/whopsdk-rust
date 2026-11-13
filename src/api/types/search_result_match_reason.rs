pub use crate::prelude::*;

/// Why the row matched, or `null` for a ranked row whose whole ID or whole title isn't the query. `id`: its ID, or a related one such as its user's, its dispute's, or a plan or payout ID for an account. `link`: a whop.com store or product link. `name`: its whole title. `email`: the email of the user it belongs to. `owner_email`, `team_member_email` or `customer_email`: the email of a subaccount's owner, a member of its team, or one of its customers. `card`: its card's first 6 or 8 and last 4 digits, or the last 4 alone. `phone`: the phone number of the user it belongs to. `payment_number`: the number of the invoice a payment collects, so an invoice from `GET /invoices` comes back as its `payment`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchResultMatchReason {
    Id,
    OwnerEmail,
    TeamMemberEmail,
    Email,
    CustomerEmail,
    Card,
    Phone,
    PaymentNumber,
    Link,
    Name,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchResultMatchReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Id => serializer.serialize_str("id"),
            Self::OwnerEmail => serializer.serialize_str("owner_email"),
            Self::TeamMemberEmail => serializer.serialize_str("team_member_email"),
            Self::Email => serializer.serialize_str("email"),
            Self::CustomerEmail => serializer.serialize_str("customer_email"),
            Self::Card => serializer.serialize_str("card"),
            Self::Phone => serializer.serialize_str("phone"),
            Self::PaymentNumber => serializer.serialize_str("payment_number"),
            Self::Link => serializer.serialize_str("link"),
            Self::Name => serializer.serialize_str("name"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchResultMatchReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "id" => Ok(Self::Id),
            "owner_email" => Ok(Self::OwnerEmail),
            "team_member_email" => Ok(Self::TeamMemberEmail),
            "email" => Ok(Self::Email),
            "customer_email" => Ok(Self::CustomerEmail),
            "card" => Ok(Self::Card),
            "phone" => Ok(Self::Phone),
            "payment_number" => Ok(Self::PaymentNumber),
            "link" => Ok(Self::Link),
            "name" => Ok(Self::Name),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchResultMatchReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id => write!(f, "id"),
            Self::OwnerEmail => write!(f, "owner_email"),
            Self::TeamMemberEmail => write!(f, "team_member_email"),
            Self::Email => write!(f, "email"),
            Self::CustomerEmail => write!(f, "customer_email"),
            Self::Card => write!(f, "card"),
            Self::Phone => write!(f, "phone"),
            Self::PaymentNumber => write!(f, "payment_number"),
            Self::Link => write!(f, "link"),
            Self::Name => write!(f, "name"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
