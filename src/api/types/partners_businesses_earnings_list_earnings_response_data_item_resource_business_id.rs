pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemResourceBusinessId {
    /// The referred business that qualified.
    #[serde(default)]
    pub business_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The onboarding reward the referred business qualified for, prefixed `onbr_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub object: ListEarningsResponseDataItemResourceBusinessIdObject,
    /// The reward link slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl ListEarningsResponseDataItemResourceBusinessId {
    pub fn builder() -> ListEarningsResponseDataItemResourceBusinessIdBuilder {
        <ListEarningsResponseDataItemResourceBusinessIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemResourceBusinessIdBuilder {
    business_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    object: Option<ListEarningsResponseDataItemResourceBusinessIdObject>,
    slug: Option<String>,
}

impl ListEarningsResponseDataItemResourceBusinessIdBuilder {
    pub fn business_id(mut self, value: impl Into<String>) -> Self {
        self.business_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: ListEarningsResponseDataItemResourceBusinessIdObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemResourceBusinessId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_id`](ListEarningsResponseDataItemResourceBusinessIdBuilder::business_id)
    /// - [`created_at`](ListEarningsResponseDataItemResourceBusinessIdBuilder::created_at)
    /// - [`object`](ListEarningsResponseDataItemResourceBusinessIdBuilder::object)
    pub fn build(self) -> Result<ListEarningsResponseDataItemResourceBusinessId, BuildError> {
        Ok(ListEarningsResponseDataItemResourceBusinessId {
            business_id: self
                .business_id
                .ok_or_else(|| BuildError::missing_field("business_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            slug: self.slug,
        })
    }
}
