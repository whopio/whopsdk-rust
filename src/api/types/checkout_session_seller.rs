pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutSessionSeller {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub id: String,
    /// The seller's logo image URL, or `null` when they have not uploaded one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Where this seller sends buyers after any purchase, or `null`. The plan's and the checkout configuration's own redirects take precedence over it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_purchase_url: Option<String>,
    /// The seller's store-page slug — the `:route` in `whop.com/joined/:route`, which is where a purchase lands when nothing more specific is configured.
    #[serde(default)]
    pub route: String,
    /// The seller's checkout legal surface: whether explicit acceptance is required, and the policy documents it covers.
    #[serde(default)]
    pub terms: CheckoutSessionSellerTerms,
    /// The seller's public name, as the buyer knows them.
    #[serde(default)]
    pub title: String,
}

impl CheckoutSessionSeller {
    pub fn builder() -> CheckoutSessionSellerBuilder {
        <CheckoutSessionSellerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionSellerBuilder {
    id: Option<String>,
    logo_url: Option<String>,
    redirect_purchase_url: Option<String>,
    route: Option<String>,
    terms: Option<CheckoutSessionSellerTerms>,
    title: Option<String>,
}

impl CheckoutSessionSellerBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn redirect_purchase_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_purchase_url = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn terms(mut self, value: CheckoutSessionSellerTerms) -> Self {
        self.terms = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionSeller`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CheckoutSessionSellerBuilder::id)
    /// - [`route`](CheckoutSessionSellerBuilder::route)
    /// - [`terms`](CheckoutSessionSellerBuilder::terms)
    /// - [`title`](CheckoutSessionSellerBuilder::title)
    pub fn build(self) -> Result<CheckoutSessionSeller, BuildError> {
        Ok(CheckoutSessionSeller {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_url: self.logo_url,
            redirect_purchase_url: self.redirect_purchase_url,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            terms: self
                .terms
                .ok_or_else(|| BuildError::missing_field("terms"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
