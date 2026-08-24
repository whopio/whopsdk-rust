pub use crate::prelude::*;

/// High-level business category for the account. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAccountsRequestBusinessType {
    EducationProgram,
    Coaching,
    Software,
    PaidGroup,
    Newsletter,
    Agency,
    PhysicalProducts,
    BrickAndMortar,
    Events,
    CoachingAndCourses,
    Other,
    Services,
    GigEconomy,
    Marketplace,
    Telehealth,
    ClassActionSettlement,
    PhysicalProduct,
    Saas,
    Course,
    Community,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAccountsRequestBusinessType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EducationProgram => serializer.serialize_str("education_program"),
            Self::Coaching => serializer.serialize_str("coaching"),
            Self::Software => serializer.serialize_str("software"),
            Self::PaidGroup => serializer.serialize_str("paid_group"),
            Self::Newsletter => serializer.serialize_str("newsletter"),
            Self::Agency => serializer.serialize_str("agency"),
            Self::PhysicalProducts => serializer.serialize_str("physical_products"),
            Self::BrickAndMortar => serializer.serialize_str("brick_and_mortar"),
            Self::Events => serializer.serialize_str("events"),
            Self::CoachingAndCourses => serializer.serialize_str("coaching_and_courses"),
            Self::Other => serializer.serialize_str("other"),
            Self::Services => serializer.serialize_str("services"),
            Self::GigEconomy => serializer.serialize_str("gig_economy"),
            Self::Marketplace => serializer.serialize_str("marketplace"),
            Self::Telehealth => serializer.serialize_str("telehealth"),
            Self::ClassActionSettlement => serializer.serialize_str("class_action_settlement"),
            Self::PhysicalProduct => serializer.serialize_str("physical_product"),
            Self::Saas => serializer.serialize_str("saas"),
            Self::Course => serializer.serialize_str("course"),
            Self::Community => serializer.serialize_str("community"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAccountsRequestBusinessType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "education_program" => Ok(Self::EducationProgram),
            "coaching" => Ok(Self::Coaching),
            "software" => Ok(Self::Software),
            "paid_group" => Ok(Self::PaidGroup),
            "newsletter" => Ok(Self::Newsletter),
            "agency" => Ok(Self::Agency),
            "physical_products" => Ok(Self::PhysicalProducts),
            "brick_and_mortar" => Ok(Self::BrickAndMortar),
            "events" => Ok(Self::Events),
            "coaching_and_courses" => Ok(Self::CoachingAndCourses),
            "other" => Ok(Self::Other),
            "services" => Ok(Self::Services),
            "gig_economy" => Ok(Self::GigEconomy),
            "marketplace" => Ok(Self::Marketplace),
            "telehealth" => Ok(Self::Telehealth),
            "class_action_settlement" => Ok(Self::ClassActionSettlement),
            "physical_product" => Ok(Self::PhysicalProduct),
            "saas" => Ok(Self::Saas),
            "course" => Ok(Self::Course),
            "community" => Ok(Self::Community),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAccountsRequestBusinessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EducationProgram => write!(f, "education_program"),
            Self::Coaching => write!(f, "coaching"),
            Self::Software => write!(f, "software"),
            Self::PaidGroup => write!(f, "paid_group"),
            Self::Newsletter => write!(f, "newsletter"),
            Self::Agency => write!(f, "agency"),
            Self::PhysicalProducts => write!(f, "physical_products"),
            Self::BrickAndMortar => write!(f, "brick_and_mortar"),
            Self::Events => write!(f, "events"),
            Self::CoachingAndCourses => write!(f, "coaching_and_courses"),
            Self::Other => write!(f, "other"),
            Self::Services => write!(f, "services"),
            Self::GigEconomy => write!(f, "gig_economy"),
            Self::Marketplace => write!(f, "marketplace"),
            Self::Telehealth => write!(f, "telehealth"),
            Self::ClassActionSettlement => write!(f, "class_action_settlement"),
            Self::PhysicalProduct => write!(f, "physical_product"),
            Self::Saas => write!(f, "saas"),
            Self::Course => write!(f, "course"),
            Self::Community => write!(f, "community"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
