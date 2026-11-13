pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipProduct {
    /// Product ID, prefixed `prod_`.
    #[serde(default)]
    pub id: String,
    /// Product logo image URL. `null` when the product has no logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Product display name.
    #[serde(default)]
    pub title: String,
}

impl MembershipProduct {
    pub fn builder() -> MembershipProductBuilder {
        <MembershipProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipProductBuilder {
    id: Option<String>,
    logo_url: Option<String>,
    title: Option<String>,
}

impl MembershipProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MembershipProductBuilder::id)
    /// - [`title`](MembershipProductBuilder::title)
    pub fn build(self) -> Result<MembershipProduct, BuildError> {
        Ok(MembershipProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_url: self.logo_url,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
