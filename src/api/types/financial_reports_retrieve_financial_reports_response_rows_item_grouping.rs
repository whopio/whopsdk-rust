pub use crate::prelude::*;

/// The family the row's `line_category` rolls up into. Balance summary rows are always `balance`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveFinancialReportsResponseRowsItemGrouping {
    Advertising,
    AffiliatesAndRevshare,
    Airdrops,
    BadDebt,
    Balance,
    Capital,
    Clawbacks,
    Crypto,
    Disputes,
    Fees,
    Fx,
    Legacy,
    LevelsPoints,
    Misc,
    OperationalCosts,
    Other,
    Payments,
    PlatformCredits,
    Refunds,
    Reserves,
    Subaccounts,
    Transfers,
    WalletsAndBalance,
    Withdrawals,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveFinancialReportsResponseRowsItemGrouping {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Advertising => serializer.serialize_str("advertising"),
            Self::AffiliatesAndRevshare => serializer.serialize_str("affiliates_and_revshare"),
            Self::Airdrops => serializer.serialize_str("airdrops"),
            Self::BadDebt => serializer.serialize_str("bad_debt"),
            Self::Balance => serializer.serialize_str("balance"),
            Self::Capital => serializer.serialize_str("capital"),
            Self::Clawbacks => serializer.serialize_str("clawbacks"),
            Self::Crypto => serializer.serialize_str("crypto"),
            Self::Disputes => serializer.serialize_str("disputes"),
            Self::Fees => serializer.serialize_str("fees"),
            Self::Fx => serializer.serialize_str("fx"),
            Self::Legacy => serializer.serialize_str("legacy"),
            Self::LevelsPoints => serializer.serialize_str("levels_points"),
            Self::Misc => serializer.serialize_str("misc"),
            Self::OperationalCosts => serializer.serialize_str("operational_costs"),
            Self::Other => serializer.serialize_str("other"),
            Self::Payments => serializer.serialize_str("payments"),
            Self::PlatformCredits => serializer.serialize_str("platform_credits"),
            Self::Refunds => serializer.serialize_str("refunds"),
            Self::Reserves => serializer.serialize_str("reserves"),
            Self::Subaccounts => serializer.serialize_str("subaccounts"),
            Self::Transfers => serializer.serialize_str("transfers"),
            Self::WalletsAndBalance => serializer.serialize_str("wallets_and_balance"),
            Self::Withdrawals => serializer.serialize_str("withdrawals"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveFinancialReportsResponseRowsItemGrouping {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "advertising" => Ok(Self::Advertising),
            "affiliates_and_revshare" => Ok(Self::AffiliatesAndRevshare),
            "airdrops" => Ok(Self::Airdrops),
            "bad_debt" => Ok(Self::BadDebt),
            "balance" => Ok(Self::Balance),
            "capital" => Ok(Self::Capital),
            "clawbacks" => Ok(Self::Clawbacks),
            "crypto" => Ok(Self::Crypto),
            "disputes" => Ok(Self::Disputes),
            "fees" => Ok(Self::Fees),
            "fx" => Ok(Self::Fx),
            "legacy" => Ok(Self::Legacy),
            "levels_points" => Ok(Self::LevelsPoints),
            "misc" => Ok(Self::Misc),
            "operational_costs" => Ok(Self::OperationalCosts),
            "other" => Ok(Self::Other),
            "payments" => Ok(Self::Payments),
            "platform_credits" => Ok(Self::PlatformCredits),
            "refunds" => Ok(Self::Refunds),
            "reserves" => Ok(Self::Reserves),
            "subaccounts" => Ok(Self::Subaccounts),
            "transfers" => Ok(Self::Transfers),
            "wallets_and_balance" => Ok(Self::WalletsAndBalance),
            "withdrawals" => Ok(Self::Withdrawals),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveFinancialReportsResponseRowsItemGrouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Advertising => write!(f, "advertising"),
            Self::AffiliatesAndRevshare => write!(f, "affiliates_and_revshare"),
            Self::Airdrops => write!(f, "airdrops"),
            Self::BadDebt => write!(f, "bad_debt"),
            Self::Balance => write!(f, "balance"),
            Self::Capital => write!(f, "capital"),
            Self::Clawbacks => write!(f, "clawbacks"),
            Self::Crypto => write!(f, "crypto"),
            Self::Disputes => write!(f, "disputes"),
            Self::Fees => write!(f, "fees"),
            Self::Fx => write!(f, "fx"),
            Self::Legacy => write!(f, "legacy"),
            Self::LevelsPoints => write!(f, "levels_points"),
            Self::Misc => write!(f, "misc"),
            Self::OperationalCosts => write!(f, "operational_costs"),
            Self::Other => write!(f, "other"),
            Self::Payments => write!(f, "payments"),
            Self::PlatformCredits => write!(f, "platform_credits"),
            Self::Refunds => write!(f, "refunds"),
            Self::Reserves => write!(f, "reserves"),
            Self::Subaccounts => write!(f, "subaccounts"),
            Self::Transfers => write!(f, "transfers"),
            Self::WalletsAndBalance => write!(f, "wallets_and_balance"),
            Self::Withdrawals => write!(f, "withdrawals"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
