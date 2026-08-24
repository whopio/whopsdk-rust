pub use crate::prelude::*;

/// The channel where an event originated
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateEventsRequestActionSource {
    Email,
    Website,
    App,
    PhoneCall,
    Chat,
    PhysicalStore,
    SystemGenerated,
    BusinessMessaging,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateEventsRequestActionSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Email => serializer.serialize_str("email"),
            Self::Website => serializer.serialize_str("website"),
            Self::App => serializer.serialize_str("app"),
            Self::PhoneCall => serializer.serialize_str("phone_call"),
            Self::Chat => serializer.serialize_str("chat"),
            Self::PhysicalStore => serializer.serialize_str("physical_store"),
            Self::SystemGenerated => serializer.serialize_str("system_generated"),
            Self::BusinessMessaging => serializer.serialize_str("business_messaging"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateEventsRequestActionSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email" => Ok(Self::Email),
            "website" => Ok(Self::Website),
            "app" => Ok(Self::App),
            "phone_call" => Ok(Self::PhoneCall),
            "chat" => Ok(Self::Chat),
            "physical_store" => Ok(Self::PhysicalStore),
            "system_generated" => Ok(Self::SystemGenerated),
            "business_messaging" => Ok(Self::BusinessMessaging),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateEventsRequestActionSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Website => write!(f, "website"),
            Self::App => write!(f, "app"),
            Self::PhoneCall => write!(f, "phone_call"),
            Self::Chat => write!(f, "chat"),
            Self::PhysicalStore => write!(f, "physical_store"),
            Self::SystemGenerated => write!(f, "system_generated"),
            Self::BusinessMessaging => write!(f, "business_messaging"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
