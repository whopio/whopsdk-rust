pub use crate::prelude::*;

/// Kind of demographic the category belongs to.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdGroupDetailedTargetingBodyDemographicsItemType {
    LifeEvents,
    Industries,
    Income,
    FamilyStatuses,
    WorkEmployers,
    WorkPositions,
    EducationSchools,
    EducationMajors,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdGroupDetailedTargetingBodyDemographicsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::LifeEvents => serializer.serialize_str("life_events"),
            Self::Industries => serializer.serialize_str("industries"),
            Self::Income => serializer.serialize_str("income"),
            Self::FamilyStatuses => serializer.serialize_str("family_statuses"),
            Self::WorkEmployers => serializer.serialize_str("work_employers"),
            Self::WorkPositions => serializer.serialize_str("work_positions"),
            Self::EducationSchools => serializer.serialize_str("education_schools"),
            Self::EducationMajors => serializer.serialize_str("education_majors"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdGroupDetailedTargetingBodyDemographicsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "life_events" => Ok(Self::LifeEvents),
            "industries" => Ok(Self::Industries),
            "income" => Ok(Self::Income),
            "family_statuses" => Ok(Self::FamilyStatuses),
            "work_employers" => Ok(Self::WorkEmployers),
            "work_positions" => Ok(Self::WorkPositions),
            "education_schools" => Ok(Self::EducationSchools),
            "education_majors" => Ok(Self::EducationMajors),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdGroupDetailedTargetingBodyDemographicsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LifeEvents => write!(f, "life_events"),
            Self::Industries => write!(f, "industries"),
            Self::Income => write!(f, "income"),
            Self::FamilyStatuses => write!(f, "family_statuses"),
            Self::WorkEmployers => write!(f, "work_employers"),
            Self::WorkPositions => write!(f, "work_positions"),
            Self::EducationSchools => write!(f, "education_schools"),
            Self::EducationMajors => write!(f, "education_majors"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
