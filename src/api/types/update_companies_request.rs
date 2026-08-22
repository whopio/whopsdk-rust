pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCompaniesRequest {
    /// Whether prospective affiliates must submit an application before they can promote this company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_application_required: Option<bool>,
    /// Guidelines and instructions shown to affiliates explaining how to promote this company's products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_instructions: Option<String>,
    /// The company's banner image. Accepts PNG or JPEG format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<UpdateCompaniesRequestBannerImage>,
    /// A promotional pitch displayed to potential customers on the company's store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the product to feature on this company's affiliate page. Pass null to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_affiliate_product_id: Option<String>,
    /// The company's logo image. Accepts PNG, JPEG, or GIF format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<UpdateCompaniesRequestLogo>,
    /// The unique URL slug for the company's store page. Must be lowercase and can include hyphens (e.g., 'my-company'). If not provided, the route will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Whether Whop sends transactional emails (receipts, renewals, cancelations) to customers on behalf of this company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_customer_emails: Option<bool>,
    /// The social media links to display on the company's store page. Pass the full list of desired social links — any existing links not included will be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_links: Option<Vec<UpdateCompaniesRequestSocialLinksItem>>,
    /// The target audience for this company (e.g., 'beginner day traders aged 18-25 looking to learn options').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<String>,
    /// The display name of the company shown to customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UpdateCompaniesRequest {
    pub fn builder() -> UpdateCompaniesRequestBuilder {
        <UpdateCompaniesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCompaniesRequestBuilder {
    affiliate_application_required: Option<bool>,
    affiliate_instructions: Option<String>,
    banner_image: Option<UpdateCompaniesRequestBannerImage>,
    description: Option<String>,
    featured_affiliate_product_id: Option<String>,
    logo: Option<UpdateCompaniesRequestLogo>,
    route: Option<String>,
    send_customer_emails: Option<bool>,
    social_links: Option<Vec<UpdateCompaniesRequestSocialLinksItem>>,
    target_audience: Option<String>,
    title: Option<String>,
}

impl UpdateCompaniesRequestBuilder {
    pub fn affiliate_application_required(mut self, value: bool) -> Self {
        self.affiliate_application_required = Some(value);
        self
    }

    pub fn affiliate_instructions(mut self, value: impl Into<String>) -> Self {
        self.affiliate_instructions = Some(value.into());
        self
    }

    pub fn banner_image(mut self, value: UpdateCompaniesRequestBannerImage) -> Self {
        self.banner_image = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn featured_affiliate_product_id(mut self, value: impl Into<String>) -> Self {
        self.featured_affiliate_product_id = Some(value.into());
        self
    }

    pub fn logo(mut self, value: UpdateCompaniesRequestLogo) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn send_customer_emails(mut self, value: bool) -> Self {
        self.send_customer_emails = Some(value);
        self
    }

    pub fn social_links(mut self, value: Vec<UpdateCompaniesRequestSocialLinksItem>) -> Self {
        self.social_links = Some(value);
        self
    }

    pub fn target_audience(mut self, value: impl Into<String>) -> Self {
        self.target_audience = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCompaniesRequest`].
    pub fn build(self) -> Result<UpdateCompaniesRequest, BuildError> {
        Ok(UpdateCompaniesRequest {
            affiliate_application_required: self.affiliate_application_required,
            affiliate_instructions: self.affiliate_instructions,
            banner_image: self.banner_image,
            description: self.description,
            featured_affiliate_product_id: self.featured_affiliate_product_id,
            logo: self.logo,
            route: self.route,
            send_customer_emails: self.send_customer_emails,
            social_links: self.social_links,
            target_audience: self.target_audience,
            title: self.title,
        })
    }
}
