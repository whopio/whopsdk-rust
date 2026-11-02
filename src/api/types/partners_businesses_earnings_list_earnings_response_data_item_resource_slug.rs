pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemResourceSlug {
    /// The referred business that qualified.
    #[serde(default)]
    pub business_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The onboarding reward the referred business qualified for, prefixed `onbr_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub object: ListEarningsResponseDataItemResourceSlugObject,
    /// The reward link slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl ListEarningsResponseDataItemResourceSlug {
    pub fn builder() -> ListEarningsResponseDataItemResourceSlugBuilder {
        <ListEarningsResponseDataItemResourceSlugBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemResourceSlugBuilder {
    business_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    object: Option<ListEarningsResponseDataItemResourceSlugObject>,
    slug: Option<String>,
}

impl ListEarningsResponseDataItemResourceSlugBuilder {
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

    pub fn object(mut self, value: ListEarningsResponseDataItemResourceSlugObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemResourceSlug`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_id`](ListEarningsResponseDataItemResourceSlugBuilder::business_id)
    /// - [`created_at`](ListEarningsResponseDataItemResourceSlugBuilder::created_at)
    /// - [`object`](ListEarningsResponseDataItemResourceSlugBuilder::object)
    pub fn build(self) -> Result<ListEarningsResponseDataItemResourceSlug, BuildError> {
        Ok(ListEarningsResponseDataItemResourceSlug {
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
