pub use crate::prelude::*;

/// Which side of the income statement the category falls on, or `null` when it is not a P&L category.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetFinancialReportResponseRowsItemProfitAndLossSection {
    Revenue,
    CostOfRevenue,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetFinancialReportResponseRowsItemProfitAndLossSection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Revenue => serializer.serialize_str("revenue"),
            Self::CostOfRevenue => serializer.serialize_str("cost_of_revenue"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetFinancialReportResponseRowsItemProfitAndLossSection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "revenue" => Ok(Self::Revenue),
            "cost_of_revenue" => Ok(Self::CostOfRevenue),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetFinancialReportResponseRowsItemProfitAndLossSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Revenue => write!(f, "revenue"),
            Self::CostOfRevenue => write!(f, "cost_of_revenue"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
