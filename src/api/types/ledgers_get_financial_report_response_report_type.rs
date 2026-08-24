pub use crate::prelude::*;

/// The report that was generated, echoing the requested `report_type`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetFinancialReportResponseReportType {
    BalanceSummary,
    IncomeStatement,
    BalanceActivity,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetFinancialReportResponseReportType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BalanceSummary => serializer.serialize_str("balance_summary"),
            Self::IncomeStatement => serializer.serialize_str("income_statement"),
            Self::BalanceActivity => serializer.serialize_str("balance_activity"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetFinancialReportResponseReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "balance_summary" => Ok(Self::BalanceSummary),
            "income_statement" => Ok(Self::IncomeStatement),
            "balance_activity" => Ok(Self::BalanceActivity),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetFinancialReportResponseReportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BalanceSummary => write!(f, "balance_summary"),
            Self::IncomeStatement => write!(f, "income_statement"),
            Self::BalanceActivity => write!(f, "balance_activity"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
