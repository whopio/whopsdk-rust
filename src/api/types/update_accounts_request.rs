pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAccountsRequest {
    /// Whether prospective affiliates must submit an application before promoting this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_application_required: Option<bool>,
    /// Guidelines shown to affiliates promoting this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_instructions: Option<String>,
    /// Account banner image, used as the cover photo when creating a Whop-managed Facebook page. Image files up to 10 MB, except `image/gif`. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<UpdateAccountsRequestBannerImage>,
    /// Account business address used to calculate tax. A complete address in a supported country is required when `tax_remitted_by` is `self`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_address: Option<UpdateAccountsRequestBusinessAddress>,
    /// The legal business name used with the account's tax address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// High-level business category for the account. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<UpdateAccountsRequestBusinessType>,
    /// Whether checkout shows a VAT/tax ID field for buyers to optionally enter. Does not require a VAT ID to purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect_vat_id: Option<bool>,
    /// Country where the account is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Account promotional description. When creating a Whop-managed Facebook page, it is truncated to 155 characters and used as the About text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the product to feature for affiliates. Pass `null` to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured_affiliate_product_id: Option<String>,
    /// Public account home page preferences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_preferences: Option<Vec<UpdateAccountsRequestHomePreferencesItem>>,
    /// Account industry group. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_group: Option<UpdateAccountsRequestIndustryGroup>,
    /// Specific industry vertical for the account. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_type: Option<String>,
    /// Prefix used for account invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,
    /// Account logo, used as the profile picture when creating a Whop-managed Facebook page. Image files up to 5 MB. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<UpdateAccountsRequestLogo>,
    /// Arbitrary key/value metadata to store on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The type of onboarding the account has completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_type: Option<UpdateAccountsRequestOnboardingType>,
    /// Open Graph preview media used when the account is shared. Image and video files up to 5 MB. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opengraph_image: Option<UpdateAccountsRequestOpengraphImage>,
    /// The account Open Graph image variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opengraph_image_variant: Option<UpdateAccountsRequestOpengraphImageVariant>,
    /// The description of the business type when business_type is other.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_business_description: Option<String>,
    /// The description of the industry type when industry_type is other.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_industry_description: Option<String>,
    /// ID of the tax classification code applied by default to the account's products. See the available [product categories](https://docs.numeral.com/essentials/product-categories).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_tax_code_id: Option<String>,
    /// Whether the account requires authorized users to have two-factor authentication enabled.
    #[serde(rename = "require_2fa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require2fa: Option<bool>,
    /// The unique URL slug for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Whether Whop sends transactional emails to customers on behalf of this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_customer_emails: Option<bool>,
    /// Whether the account appears in joined whops on other accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_joined_whops: Option<bool>,
    /// Whether reviews are displayed on direct-to-consumer product pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_reviews_dtc: Option<bool>,
    /// Whether the account shows users in the user directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_user_directory: Option<bool>,
    /// The full list of social links to display for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_links: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// Account store page display configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_page_config: Option<UpdateAccountsRequestStorePageConfig>,
    /// The target audience for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<String>,
    /// US state codes (50 states plus `DC`) where the account collects tax. Replaces the full set on update. Only settable when `tax_remitted_by` is `self`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_collection_enabled_states:
        Option<Vec<UpdateAccountsRequestTaxCollectionEnabledStatesItem>>,
    /// Account tax/VAT registrations to add or update. When `tax_remitted_by` is `self`, tax is calculated and collected only in the countries where the account holds a registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_identifiers: Option<Vec<UpdateAccountsRequestTaxIdentifiersItem>>,
    /// Determines whether Whop or the account calculates and remits tax. The account must provide a supported-country business address when it self-remits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_remitted_by: Option<UpdateAccountsRequestTaxRemittedBy>,
    /// Determines whether tax is included in the listed price or added at checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<UpdateAccountsRequestTaxType>,
    /// Account-level 3D Secure behavior. Set `mandate_challenge` to require cardholder verification on supported card payments, or `null` to use the standard checkout flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<UpdateAccountsRequestThreeDsLevel>,
    /// The display name of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether the account uses its logo as the fallback Open Graph image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_logo_as_opengraph_image_fallback: Option<bool>,
}

impl UpdateAccountsRequest {
    pub fn builder() -> UpdateAccountsRequestBuilder {
        <UpdateAccountsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestBuilder {
    affiliate_application_required: Option<bool>,
    affiliate_instructions: Option<String>,
    banner_image: Option<UpdateAccountsRequestBannerImage>,
    business_address: Option<UpdateAccountsRequestBusinessAddress>,
    business_name: Option<String>,
    business_type: Option<UpdateAccountsRequestBusinessType>,
    collect_vat_id: Option<bool>,
    country: Option<String>,
    description: Option<String>,
    featured_affiliate_product_id: Option<String>,
    home_preferences: Option<Vec<UpdateAccountsRequestHomePreferencesItem>>,
    industry_group: Option<UpdateAccountsRequestIndustryGroup>,
    industry_type: Option<String>,
    invoice_prefix: Option<String>,
    logo: Option<UpdateAccountsRequestLogo>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    onboarding_type: Option<UpdateAccountsRequestOnboardingType>,
    opengraph_image: Option<UpdateAccountsRequestOpengraphImage>,
    opengraph_image_variant: Option<UpdateAccountsRequestOpengraphImageVariant>,
    other_business_description: Option<String>,
    other_industry_description: Option<String>,
    product_tax_code_id: Option<String>,
    require2fa: Option<bool>,
    route: Option<String>,
    send_customer_emails: Option<bool>,
    show_joined_whops: Option<bool>,
    show_reviews_dtc: Option<bool>,
    show_user_directory: Option<bool>,
    social_links: Option<Vec<HashMap<String, serde_json::Value>>>,
    store_page_config: Option<UpdateAccountsRequestStorePageConfig>,
    target_audience: Option<String>,
    tax_collection_enabled_states: Option<Vec<UpdateAccountsRequestTaxCollectionEnabledStatesItem>>,
    tax_identifiers: Option<Vec<UpdateAccountsRequestTaxIdentifiersItem>>,
    tax_remitted_by: Option<UpdateAccountsRequestTaxRemittedBy>,
    tax_type: Option<UpdateAccountsRequestTaxType>,
    three_ds_level: Option<UpdateAccountsRequestThreeDsLevel>,
    title: Option<String>,
    use_logo_as_opengraph_image_fallback: Option<bool>,
}

impl UpdateAccountsRequestBuilder {
    pub fn affiliate_application_required(mut self, value: bool) -> Self {
        self.affiliate_application_required = Some(value);
        self
    }

    pub fn affiliate_instructions(mut self, value: impl Into<String>) -> Self {
        self.affiliate_instructions = Some(value.into());
        self
    }

    pub fn banner_image(mut self, value: UpdateAccountsRequestBannerImage) -> Self {
        self.banner_image = Some(value);
        self
    }

    pub fn business_address(mut self, value: UpdateAccountsRequestBusinessAddress) -> Self {
        self.business_address = Some(value);
        self
    }

    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn business_type(mut self, value: UpdateAccountsRequestBusinessType) -> Self {
        self.business_type = Some(value);
        self
    }

    pub fn collect_vat_id(mut self, value: bool) -> Self {
        self.collect_vat_id = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    pub fn home_preferences(
        mut self,
        value: Vec<UpdateAccountsRequestHomePreferencesItem>,
    ) -> Self {
        self.home_preferences = Some(value);
        self
    }

    pub fn industry_group(mut self, value: UpdateAccountsRequestIndustryGroup) -> Self {
        self.industry_group = Some(value);
        self
    }

    pub fn industry_type(mut self, value: impl Into<String>) -> Self {
        self.industry_type = Some(value.into());
        self
    }

    pub fn invoice_prefix(mut self, value: impl Into<String>) -> Self {
        self.invoice_prefix = Some(value.into());
        self
    }

    pub fn logo(mut self, value: UpdateAccountsRequestLogo) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn onboarding_type(mut self, value: UpdateAccountsRequestOnboardingType) -> Self {
        self.onboarding_type = Some(value);
        self
    }

    pub fn opengraph_image(mut self, value: UpdateAccountsRequestOpengraphImage) -> Self {
        self.opengraph_image = Some(value);
        self
    }

    pub fn opengraph_image_variant(
        mut self,
        value: UpdateAccountsRequestOpengraphImageVariant,
    ) -> Self {
        self.opengraph_image_variant = Some(value);
        self
    }

    pub fn other_business_description(mut self, value: impl Into<String>) -> Self {
        self.other_business_description = Some(value.into());
        self
    }

    pub fn other_industry_description(mut self, value: impl Into<String>) -> Self {
        self.other_industry_description = Some(value.into());
        self
    }

    pub fn product_tax_code_id(mut self, value: impl Into<String>) -> Self {
        self.product_tax_code_id = Some(value.into());
        self
    }

    pub fn require2fa(mut self, value: bool) -> Self {
        self.require2fa = Some(value);
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

    pub fn show_joined_whops(mut self, value: bool) -> Self {
        self.show_joined_whops = Some(value);
        self
    }

    pub fn show_reviews_dtc(mut self, value: bool) -> Self {
        self.show_reviews_dtc = Some(value);
        self
    }

    pub fn show_user_directory(mut self, value: bool) -> Self {
        self.show_user_directory = Some(value);
        self
    }

    pub fn social_links(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.social_links = Some(value);
        self
    }

    pub fn store_page_config(mut self, value: UpdateAccountsRequestStorePageConfig) -> Self {
        self.store_page_config = Some(value);
        self
    }

    pub fn target_audience(mut self, value: impl Into<String>) -> Self {
        self.target_audience = Some(value.into());
        self
    }

    pub fn tax_collection_enabled_states(
        mut self,
        value: Vec<UpdateAccountsRequestTaxCollectionEnabledStatesItem>,
    ) -> Self {
        self.tax_collection_enabled_states = Some(value);
        self
    }

    pub fn tax_identifiers(mut self, value: Vec<UpdateAccountsRequestTaxIdentifiersItem>) -> Self {
        self.tax_identifiers = Some(value);
        self
    }

    pub fn tax_remitted_by(mut self, value: UpdateAccountsRequestTaxRemittedBy) -> Self {
        self.tax_remitted_by = Some(value);
        self
    }

    pub fn tax_type(mut self, value: UpdateAccountsRequestTaxType) -> Self {
        self.tax_type = Some(value);
        self
    }

    pub fn three_ds_level(mut self, value: UpdateAccountsRequestThreeDsLevel) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn use_logo_as_opengraph_image_fallback(mut self, value: bool) -> Self {
        self.use_logo_as_opengraph_image_fallback = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequest`].
    pub fn build(self) -> Result<UpdateAccountsRequest, BuildError> {
        Ok(UpdateAccountsRequest {
            affiliate_application_required: self.affiliate_application_required,
            affiliate_instructions: self.affiliate_instructions,
            banner_image: self.banner_image,
            business_address: self.business_address,
            business_name: self.business_name,
            business_type: self.business_type,
            collect_vat_id: self.collect_vat_id,
            country: self.country,
            description: self.description,
            featured_affiliate_product_id: self.featured_affiliate_product_id,
            home_preferences: self.home_preferences,
            industry_group: self.industry_group,
            industry_type: self.industry_type,
            invoice_prefix: self.invoice_prefix,
            logo: self.logo,
            metadata: self.metadata,
            onboarding_type: self.onboarding_type,
            opengraph_image: self.opengraph_image,
            opengraph_image_variant: self.opengraph_image_variant,
            other_business_description: self.other_business_description,
            other_industry_description: self.other_industry_description,
            product_tax_code_id: self.product_tax_code_id,
            require2fa: self.require2fa,
            route: self.route,
            send_customer_emails: self.send_customer_emails,
            show_joined_whops: self.show_joined_whops,
            show_reviews_dtc: self.show_reviews_dtc,
            show_user_directory: self.show_user_directory,
            social_links: self.social_links,
            store_page_config: self.store_page_config,
            target_audience: self.target_audience,
            tax_collection_enabled_states: self.tax_collection_enabled_states,
            tax_identifiers: self.tax_identifiers,
            tax_remitted_by: self.tax_remitted_by,
            tax_type: self.tax_type,
            three_ds_level: self.three_ds_level,
            title: self.title,
            use_logo_as_opengraph_image_fallback: self.use_logo_as_opengraph_image_fallback,
        })
    }
}
