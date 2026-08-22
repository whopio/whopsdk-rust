pub use crate::prelude::*;

/// The type of refund reference that was made available by the payment provider.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefundReferenceTypes {
    AcquirerReferenceNumber,
    RetrievalReferenceNumber,
    SystemTraceAuditNumber,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RefundReferenceTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AcquirerReferenceNumber => serializer.serialize_str("acquirer_reference_number"),
            Self::RetrievalReferenceNumber => {
                serializer.serialize_str("retrieval_reference_number")
            }
            Self::SystemTraceAuditNumber => serializer.serialize_str("system_trace_audit_number"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RefundReferenceTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "acquirer_reference_number" => Ok(Self::AcquirerReferenceNumber),
            "retrieval_reference_number" => Ok(Self::RetrievalReferenceNumber),
            "system_trace_audit_number" => Ok(Self::SystemTraceAuditNumber),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RefundReferenceTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AcquirerReferenceNumber => write!(f, "acquirer_reference_number"),
            Self::RetrievalReferenceNumber => write!(f, "retrieval_reference_number"),
            Self::SystemTraceAuditNumber => write!(f, "system_trace_audit_number"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
