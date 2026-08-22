pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TargetingOption {
    #[serde(rename = "behaviors")]
    #[non_exhaustive]
    Behaviors {
        #[serde(flatten)]
        data: DetailedTargetingOption,
    },

    #[serde(rename = "education_majors")]
    #[non_exhaustive]
    EducationMajors {
        #[serde(flatten)]
        data: DetailedTargetingOption,
    },

    #[serde(rename = "education_schools")]
    #[non_exhaustive]
    EducationSchools {
        #[serde(flatten)]
        data: DetailedTargetingOption,
    },

    #[serde(rename = "family_statuses")]
    #[non_exhaustive]
    FamilyStatuses {
        #[serde(flatten)]
        data: DetailedTargetingOption,
    },

    #[serde(rename = "income")]
    #[non_exhaustive]
    Income {
        #[serde(flatten)]
        data: DetailedTargetingOption,
    },

    #[serde(rename = "industries")]
    #[non_exhaustive]
    Industries {
        #[serde(flatten)]
        data: DetailedTargetingOption,
    },

    #[serde(rename = "interests")]
    #[non_exhaustive]
    Interests {
        #[serde(flatten)]
        data: DetailedTargetingOption,
    },

    #[serde(rename = "languages")]
    #[non_exhaustive]
    Languages {
        #[serde(default)]
        code: String,
        #[serde(default)]
        name: String,
    },

    #[serde(rename = "life_events")]
    #[non_exhaustive]
    LifeEvents {
        #[serde(flatten)]
        data: DetailedTargetingOption,
    },

    #[serde(rename = "locations")]
    #[non_exhaustive]
    Locations {
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        country_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        country_name: Option<String>,
        #[serde(default)]
        key: String,
        location_type: LocationTargetingOptionLocationType,
        #[serde(default)]
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        region: Option<String>,
    },

    #[serde(rename = "work_employers")]
    #[non_exhaustive]
    WorkEmployers {
        #[serde(flatten)]
        data: DetailedTargetingOption,
    },

    #[serde(rename = "work_positions")]
    #[non_exhaustive]
    WorkPositions {
        #[serde(flatten)]
        data: DetailedTargetingOption,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TargetingOption {
    pub fn behaviors(data: DetailedTargetingOption) -> Self {
        Self::Behaviors { data }
    }

    pub fn education_majors(data: DetailedTargetingOption) -> Self {
        Self::EducationMajors { data }
    }

    pub fn education_schools(data: DetailedTargetingOption) -> Self {
        Self::EducationSchools { data }
    }

    pub fn family_statuses(data: DetailedTargetingOption) -> Self {
        Self::FamilyStatuses { data }
    }

    pub fn income(data: DetailedTargetingOption) -> Self {
        Self::Income { data }
    }

    pub fn industries(data: DetailedTargetingOption) -> Self {
        Self::Industries { data }
    }

    pub fn interests(data: DetailedTargetingOption) -> Self {
        Self::Interests { data }
    }

    pub fn languages(code: String, name: String) -> Self {
        Self::Languages { code, name }
    }

    pub fn life_events(data: DetailedTargetingOption) -> Self {
        Self::LifeEvents { data }
    }

    pub fn locations(
        key: String,
        location_type: LocationTargetingOptionLocationType,
        name: String,
    ) -> Self {
        Self::Locations {
            code: None,
            country_code: None,
            country_name: None,
            key,
            location_type,
            name,
            region: None,
        }
    }

    pub fn work_employers(data: DetailedTargetingOption) -> Self {
        Self::WorkEmployers { data }
    }

    pub fn work_positions(data: DetailedTargetingOption) -> Self {
        Self::WorkPositions { data }
    }

    pub fn locations_with_code(
        code: String,
        country_code: Option<String>,
        country_name: Option<String>,
        key: String,
        location_type: LocationTargetingOptionLocationType,
        name: String,
        region: Option<String>,
    ) -> Self {
        Self::Locations {
            code: Some(code),
            country_code,
            country_name,
            key,
            location_type,
            name,
            region,
        }
    }

    pub fn locations_with_country_code(
        code: Option<String>,
        country_code: String,
        country_name: Option<String>,
        key: String,
        location_type: LocationTargetingOptionLocationType,
        name: String,
        region: Option<String>,
    ) -> Self {
        Self::Locations {
            code,
            country_code: Some(country_code),
            country_name,
            key,
            location_type,
            name,
            region,
        }
    }

    pub fn locations_with_country_name(
        code: Option<String>,
        country_code: Option<String>,
        country_name: String,
        key: String,
        location_type: LocationTargetingOptionLocationType,
        name: String,
        region: Option<String>,
    ) -> Self {
        Self::Locations {
            code,
            country_code,
            country_name: Some(country_name),
            key,
            location_type,
            name,
            region,
        }
    }

    pub fn locations_with_region(
        code: Option<String>,
        country_code: Option<String>,
        country_name: Option<String>,
        key: String,
        location_type: LocationTargetingOptionLocationType,
        name: String,
        region: String,
    ) -> Self {
        Self::Locations {
            code,
            country_code,
            country_name,
            key,
            location_type,
            name,
            region: Some(region),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
