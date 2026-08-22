pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAdsRequestLeadFormQuestionsItemType {
    Email,
    Phone,
    FullName,
    FirstName,
    LastName,
    City,
    State,
    Zip,
    Country,
    StreetAddress,
    JobTitle,
    CompanyName,
    WorkEmail,
    WorkPhoneNumber,
    Dob,
    Gender,
    MaritalStatus,
    RelationshipStatus,
    MilitaryStatus,
    DateTime,
    Custom,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAdsRequestLeadFormQuestionsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Email => serializer.serialize_str("email"),
            Self::Phone => serializer.serialize_str("phone"),
            Self::FullName => serializer.serialize_str("full_name"),
            Self::FirstName => serializer.serialize_str("first_name"),
            Self::LastName => serializer.serialize_str("last_name"),
            Self::City => serializer.serialize_str("city"),
            Self::State => serializer.serialize_str("state"),
            Self::Zip => serializer.serialize_str("zip"),
            Self::Country => serializer.serialize_str("country"),
            Self::StreetAddress => serializer.serialize_str("street_address"),
            Self::JobTitle => serializer.serialize_str("job_title"),
            Self::CompanyName => serializer.serialize_str("company_name"),
            Self::WorkEmail => serializer.serialize_str("work_email"),
            Self::WorkPhoneNumber => serializer.serialize_str("work_phone_number"),
            Self::Dob => serializer.serialize_str("dob"),
            Self::Gender => serializer.serialize_str("gender"),
            Self::MaritalStatus => serializer.serialize_str("marital_status"),
            Self::RelationshipStatus => serializer.serialize_str("relationship_status"),
            Self::MilitaryStatus => serializer.serialize_str("military_status"),
            Self::DateTime => serializer.serialize_str("date_time"),
            Self::Custom => serializer.serialize_str("custom"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAdsRequestLeadFormQuestionsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email" => Ok(Self::Email),
            "phone" => Ok(Self::Phone),
            "full_name" => Ok(Self::FullName),
            "first_name" => Ok(Self::FirstName),
            "last_name" => Ok(Self::LastName),
            "city" => Ok(Self::City),
            "state" => Ok(Self::State),
            "zip" => Ok(Self::Zip),
            "country" => Ok(Self::Country),
            "street_address" => Ok(Self::StreetAddress),
            "job_title" => Ok(Self::JobTitle),
            "company_name" => Ok(Self::CompanyName),
            "work_email" => Ok(Self::WorkEmail),
            "work_phone_number" => Ok(Self::WorkPhoneNumber),
            "dob" => Ok(Self::Dob),
            "gender" => Ok(Self::Gender),
            "marital_status" => Ok(Self::MaritalStatus),
            "relationship_status" => Ok(Self::RelationshipStatus),
            "military_status" => Ok(Self::MilitaryStatus),
            "date_time" => Ok(Self::DateTime),
            "custom" => Ok(Self::Custom),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAdsRequestLeadFormQuestionsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Phone => write!(f, "phone"),
            Self::FullName => write!(f, "full_name"),
            Self::FirstName => write!(f, "first_name"),
            Self::LastName => write!(f, "last_name"),
            Self::City => write!(f, "city"),
            Self::State => write!(f, "state"),
            Self::Zip => write!(f, "zip"),
            Self::Country => write!(f, "country"),
            Self::StreetAddress => write!(f, "street_address"),
            Self::JobTitle => write!(f, "job_title"),
            Self::CompanyName => write!(f, "company_name"),
            Self::WorkEmail => write!(f, "work_email"),
            Self::WorkPhoneNumber => write!(f, "work_phone_number"),
            Self::Dob => write!(f, "dob"),
            Self::Gender => write!(f, "gender"),
            Self::MaritalStatus => write!(f, "marital_status"),
            Self::RelationshipStatus => write!(f, "relationship_status"),
            Self::MilitaryStatus => write!(f, "military_status"),
            Self::DateTime => write!(f, "date_time"),
            Self::Custom => write!(f, "custom"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
