pub use crate::prelude::*;

/// An experience is a feature or content module within a product, such as a chat, course, or custom app.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Experience {
    /// The app that powers this experience, defining its interface and behavior.
    #[serde(default)]
    pub app: ExperienceApp,
    /// The company that owns this experience.
    #[serde(default)]
    pub company: ExperienceCompany,
    /// The datetime the experience was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the experience.
    #[serde(default)]
    pub id: String,
    /// The custom logo image for this experience. Null if no custom logo has been uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ExperienceImage>,
    /// Whether this experience is publicly visible to all users, including those without a membership.
    #[serde(default)]
    pub is_public: bool,
    /// The display name of this experience shown to users in the product navigation. Maximum 255 characters.
    #[serde(default)]
    pub name: String,
    /// The sort position of this experience within its section. Lower values appear first. Null if no position has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// The list of products this experience is attached to, which determines which customers have access. Empty if the experience is only visible to authorized company team members.
    #[serde(default)]
    pub products: Vec<ExperienceProductsItem>,
}

impl Experience {
    pub fn builder() -> ExperienceBuilder {
        <ExperienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperienceBuilder {
    app: Option<ExperienceApp>,
    company: Option<ExperienceCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    image: Option<ExperienceImage>,
    is_public: Option<bool>,
    name: Option<String>,
    order: Option<String>,
    products: Option<Vec<ExperienceProductsItem>>,
}

impl ExperienceBuilder {
    pub fn app(mut self, value: ExperienceApp) -> Self {
        self.app = Some(value);
        self
    }

    pub fn company(mut self, value: ExperienceCompany) -> Self {
        self.company = Some(value);
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

    pub fn image(mut self, value: ExperienceImage) -> Self {
        self.image = Some(value);
        self
    }

    pub fn is_public(mut self, value: bool) -> Self {
        self.is_public = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.order = Some(value.into());
        self
    }

    pub fn products(mut self, value: Vec<ExperienceProductsItem>) -> Self {
        self.products = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Experience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`app`](ExperienceBuilder::app)
    /// - [`company`](ExperienceBuilder::company)
    /// - [`created_at`](ExperienceBuilder::created_at)
    /// - [`id`](ExperienceBuilder::id)
    /// - [`is_public`](ExperienceBuilder::is_public)
    /// - [`name`](ExperienceBuilder::name)
    /// - [`products`](ExperienceBuilder::products)
    pub fn build(self) -> Result<Experience, BuildError> {
        Ok(Experience {
            app: self.app.ok_or_else(|| BuildError::missing_field("app"))?,
            company: self
                .company
                .ok_or_else(|| BuildError::missing_field("company"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image: self.image,
            is_public: self
                .is_public
                .ok_or_else(|| BuildError::missing_field("is_public"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            order: self.order,
            products: self
                .products
                .ok_or_else(|| BuildError::missing_field("products"))?,
        })
    }
}
