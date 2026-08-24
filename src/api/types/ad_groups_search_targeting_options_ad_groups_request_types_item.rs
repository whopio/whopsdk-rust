pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchTargetingOptionsAdGroupsRequestTypesItem {
    Interests,
    Behaviors,
    LifeEvents,
    Industries,
    Income,
    FamilyStatuses,
    WorkEmployers,
    WorkPositions,
    EducationSchools,
    EducationMajors,
    Languages,
    Locations,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchTargetingOptionsAdGroupsRequestTypesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Interests => serializer.serialize_str("interests"),
            Self::Behaviors => serializer.serialize_str("behaviors"),
            Self::LifeEvents => serializer.serialize_str("life_events"),
            Self::Industries => serializer.serialize_str("industries"),
            Self::Income => serializer.serialize_str("income"),
            Self::FamilyStatuses => serializer.serialize_str("family_statuses"),
            Self::WorkEmployers => serializer.serialize_str("work_employers"),
            Self::WorkPositions => serializer.serialize_str("work_positions"),
            Self::EducationSchools => serializer.serialize_str("education_schools"),
            Self::EducationMajors => serializer.serialize_str("education_majors"),
            Self::Languages => serializer.serialize_str("languages"),
            Self::Locations => serializer.serialize_str("locations"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchTargetingOptionsAdGroupsRequestTypesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "interests" => Ok(Self::Interests),
            "behaviors" => Ok(Self::Behaviors),
            "life_events" => Ok(Self::LifeEvents),
            "industries" => Ok(Self::Industries),
            "income" => Ok(Self::Income),
            "family_statuses" => Ok(Self::FamilyStatuses),
            "work_employers" => Ok(Self::WorkEmployers),
            "work_positions" => Ok(Self::WorkPositions),
            "education_schools" => Ok(Self::EducationSchools),
            "education_majors" => Ok(Self::EducationMajors),
            "languages" => Ok(Self::Languages),
            "locations" => Ok(Self::Locations),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchTargetingOptionsAdGroupsRequestTypesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interests => write!(f, "interests"),
            Self::Behaviors => write!(f, "behaviors"),
            Self::LifeEvents => write!(f, "life_events"),
            Self::Industries => write!(f, "industries"),
            Self::Income => write!(f, "income"),
            Self::FamilyStatuses => write!(f, "family_statuses"),
            Self::WorkEmployers => write!(f, "work_employers"),
            Self::WorkPositions => write!(f, "work_positions"),
            Self::EducationSchools => write!(f, "education_schools"),
            Self::EducationMajors => write!(f, "education_majors"),
            Self::Languages => write!(f, "languages"),
            Self::Locations => write!(f, "locations"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
