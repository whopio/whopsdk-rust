pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Account {
    #[serde(default)]
    pub balances: Vec<AccountBalanceToken>,
    /// Account banner image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image_url: Option<String>,
    /// Account business address used to calculate tax, with `line1`, `line2`, `city`, `state`, `postal_code`, and `country`. `null` when no address is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_address: Option<HashMap<String, serde_json::Value>>,
    /// The account's legal business name used with its tax address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// High-level business category for the account. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<AccountBusinessType>,
    /// Whether pending funds may be transferred from this platform account to its connected accounts.
    #[serde(default)]
    pub can_transfer_pending_balance_to_children: bool,
    /// Payment rails enabled for this account, each `active`, `inactive`, or `pending` (onboarding or review in progress). Computed only on `retrieve` and `me` for callers with `company:balance:read` scope; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AccountCapabilities>,
    /// Whop Cards application details for the account. Computed only on `retrieve` and `me` for callers with `company:balance:read` scope; `null` otherwise, or when the account has no card application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards: Option<AccountCards>,
    /// Whether checkout shows a VAT/tax ID field for buyers to optionally enter. Does not require a VAT ID to purchase.
    #[serde(default)]
    pub collect_vat_id: bool,
    /// Company formation state for the account, managed through [Form Company](/api-reference/beta/accounts/form-company). A `draft` `status` until the formation checkout is paid, then filing progress with downloadable documents and signatures awaiting action. Empty when the formation state is temporarily unavailable.
    #[serde(default)]
    pub company_formation: AccountCompanyFormation,
    /// Country where the account is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// When the account was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Account promotional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Account owner email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The account's end-user license agreement document, or `null` if they have not published one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eula: Option<File>,
    #[serde(default)]
    pub home_preferences: Vec<AccountHomePreferencesItem>,
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub id: String,
    /// Account industry group. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_group: Option<AccountIndustryGroup>,
    /// Specific industry vertical for the account. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_type: Option<String>,
    /// Prefix used for account invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,
    /// Account logo image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Arbitrary key/value metadata supplied at account creation.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// Type of onboarding the account has completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_type: Option<AccountOnboardingType>,
    /// Account Open Graph image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opengraph_image_url: Option<String>,
    /// Account Open Graph image variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opengraph_image_variant: Option<AccountOpengraphImageVariant>,
    /// Business type details when business_type is `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_business_description: Option<String>,
    /// Industry details when industry_type is `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_industry_description: Option<String>,
    /// The single user who owns the account, whose email is the `email` above. Distinct from the `owner` role on team members, which any number of them can hold.
    #[serde(default)]
    pub owner: UserSummary,
    /// Parent account for connected accounts, or `null` for standalone accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_account: Option<AccountParent>,
    /// Payment health controls currently applied to the account. Computed only on `retrieve` and `me` for callers with `company:balance:read` scope; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_controls: Option<AccountPaymentControls>,
    /// The account's privacy policy document, or `null` if they have not published one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<File>,
    /// Tax classification code applied by default to the account's products, with `id`, `name`, and `product_type`. `null` when no default is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_tax_code: Option<HashMap<String, serde_json::Value>>,
    /// DEPRECATED: Use the `GET /recommended_actions?account_id={account_id}` endpoint instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_actions: Option<Vec<AccountRecommendedAction>>,
    /// Whether authorized users must enable two-factor authentication.
    #[serde(rename = "require_2fa")]
    #[serde(default)]
    pub require2fa: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_actions: Option<Vec<AccountRequiredAction>>,
    /// The account's return policy document, or `null` if they have not published one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_policy: Option<File>,
    /// Account public route identifier.
    #[serde(default)]
    pub route: String,
    /// Whether Whop sends transactional emails to customers on behalf of this account.
    #[serde(default)]
    pub send_customer_emails: bool,
    /// Whether the account appears in joined whops on other accounts.
    #[serde(default)]
    pub show_joined_whops: bool,
    /// Whether reviews are displayed on direct-to-consumer product pages.
    #[serde(default)]
    pub show_reviews_dtc: bool,
    /// Whether the account shows users in the user directory.
    #[serde(default)]
    pub show_user_directory: bool,
    #[serde(default)]
    pub social_links: Vec<AccountSocialLink>,
    /// Whether the account settles on stablecoin rails — its balance is held on-chain as USDT and paid out over crypto, rather than as fiat cash.
    #[serde(default)]
    pub stablecoin_rails: bool,
    /// Whether the account can operate on Whop: `active` or `suspended`. Computed on `list`, `retrieve`, and `me`; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Why the account was suspended, in language safe to show the account owner. Computed only on `retrieve` and `me`; `null` otherwise, when `status` is not `suspended`, and when the suspension was recorded without a reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// Account store page display configuration.
    #[serde(default)]
    pub store_page_config: AccountStorePageConfig,
    /// Target audience for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_audience: Option<String>,
    #[serde(default)]
    pub tax_collection_enabled_states: Vec<String>,
    #[serde(default)]
    pub tax_identifiers: Vec<AccountTaxIdentifier>,
    /// Who calculates and remits tax for the account: `whop` (Whop calculates and remits), `self` (Whop calculates; the account collects and remits), or `none` (neither; the account is responsible). `null` until the account enrolls in the Whop tax service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_remitted_by: Option<AccountTaxRemittedBy>,
    /// How tax is applied to the account's prices: `inclusive` (tax included in the listed price) or `exclusive` (tax added on top). Defaults to `exclusive` when unset; `null` only when the account has no payment connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<AccountTaxType>,
    /// The account's terms of service document, or `null` if they have not published one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<File>,
    /// Account-level 3D Secure behavior. `mandate_challenge` requires cardholder verification on supported card payments; `null` uses the standard checkout flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<AccountThreeDsLevel>,
    /// Account display name.
    #[serde(default)]
    pub title: String,
    /// Account lifetime sales, normalized to USD. Computed only on `retrieve` and `me` for callers with `stats:read` scope; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_earned_usd: Option<f64>,
    /// Total USD value across balances with known exchange rates. Computed only on single-account reads (`retrieve` and `me`); `null` on list responses, writes, missing balance-read permission, or unavailable balance source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_usd: Option<String>,
    /// Whether the account uses its logo as the fallback Open Graph image.
    #[serde(default)]
    pub use_logo_as_opengraph_image_fallback: bool,
    /// Account identity verification status for the `individual` (KYC) and `business` (KYB) profiles. Each is `null` until created, otherwise a `status` of `not_started`, `pending`, `manual_review`, `approved`, or `rejected`.
    #[serde(default)]
    pub verification: HashMap<String, serde_json::Value>,
    /// Lifetime volume through the account — sales plus transfers received — normalized to USD. Computed only on `list` for callers with `stats:read` on the account; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub volume_usd: Option<f64>,
    /// Account primary crypto wallet, or `null` if none has been provisioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<AccountWallet>,
}

impl Account {
    pub fn builder() -> AccountBuilder {
        <AccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountBuilder {
    balances: Option<Vec<AccountBalanceToken>>,
    banner_image_url: Option<String>,
    business_address: Option<HashMap<String, serde_json::Value>>,
    business_name: Option<String>,
    business_type: Option<AccountBusinessType>,
    can_transfer_pending_balance_to_children: Option<bool>,
    capabilities: Option<AccountCapabilities>,
    cards: Option<AccountCards>,
    collect_vat_id: Option<bool>,
    company_formation: Option<AccountCompanyFormation>,
    country: Option<String>,
    created_at: Option<String>,
    description: Option<String>,
    email: Option<String>,
    eula: Option<File>,
    home_preferences: Option<Vec<AccountHomePreferencesItem>>,
    id: Option<String>,
    industry_group: Option<AccountIndustryGroup>,
    industry_type: Option<String>,
    invoice_prefix: Option<String>,
    logo_url: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    onboarding_type: Option<AccountOnboardingType>,
    opengraph_image_url: Option<String>,
    opengraph_image_variant: Option<AccountOpengraphImageVariant>,
    other_business_description: Option<String>,
    other_industry_description: Option<String>,
    owner: Option<UserSummary>,
    parent_account: Option<AccountParent>,
    payment_controls: Option<AccountPaymentControls>,
    privacy_policy: Option<File>,
    product_tax_code: Option<HashMap<String, serde_json::Value>>,
    recommended_actions: Option<Vec<AccountRecommendedAction>>,
    require2fa: Option<bool>,
    required_actions: Option<Vec<AccountRequiredAction>>,
    return_policy: Option<File>,
    route: Option<String>,
    send_customer_emails: Option<bool>,
    show_joined_whops: Option<bool>,
    show_reviews_dtc: Option<bool>,
    show_user_directory: Option<bool>,
    social_links: Option<Vec<AccountSocialLink>>,
    stablecoin_rails: Option<bool>,
    status: Option<String>,
    status_reason: Option<String>,
    store_page_config: Option<AccountStorePageConfig>,
    target_audience: Option<String>,
    tax_collection_enabled_states: Option<Vec<String>>,
    tax_identifiers: Option<Vec<AccountTaxIdentifier>>,
    tax_remitted_by: Option<AccountTaxRemittedBy>,
    tax_type: Option<AccountTaxType>,
    terms_of_service: Option<File>,
    three_ds_level: Option<AccountThreeDsLevel>,
    title: Option<String>,
    total_earned_usd: Option<f64>,
    total_usd: Option<String>,
    use_logo_as_opengraph_image_fallback: Option<bool>,
    verification: Option<HashMap<String, serde_json::Value>>,
    volume_usd: Option<f64>,
    wallet: Option<AccountWallet>,
}

impl AccountBuilder {
    pub fn balances(mut self, value: Vec<AccountBalanceToken>) -> Self {
        self.balances = Some(value);
        self
    }

    pub fn banner_image_url(mut self, value: impl Into<String>) -> Self {
        self.banner_image_url = Some(value.into());
        self
    }

    pub fn business_address(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.business_address = Some(value);
        self
    }

    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn business_type(mut self, value: AccountBusinessType) -> Self {
        self.business_type = Some(value);
        self
    }

    pub fn can_transfer_pending_balance_to_children(mut self, value: bool) -> Self {
        self.can_transfer_pending_balance_to_children = Some(value);
        self
    }

    pub fn capabilities(mut self, value: AccountCapabilities) -> Self {
        self.capabilities = Some(value);
        self
    }

    pub fn cards(mut self, value: AccountCards) -> Self {
        self.cards = Some(value);
        self
    }

    pub fn collect_vat_id(mut self, value: bool) -> Self {
        self.collect_vat_id = Some(value);
        self
    }

    pub fn company_formation(mut self, value: AccountCompanyFormation) -> Self {
        self.company_formation = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn eula(mut self, value: File) -> Self {
        self.eula = Some(value);
        self
    }

    pub fn home_preferences(mut self, value: Vec<AccountHomePreferencesItem>) -> Self {
        self.home_preferences = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn industry_group(mut self, value: AccountIndustryGroup) -> Self {
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

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn onboarding_type(mut self, value: AccountOnboardingType) -> Self {
        self.onboarding_type = Some(value);
        self
    }

    pub fn opengraph_image_url(mut self, value: impl Into<String>) -> Self {
        self.opengraph_image_url = Some(value.into());
        self
    }

    pub fn opengraph_image_variant(mut self, value: AccountOpengraphImageVariant) -> Self {
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

    pub fn owner(mut self, value: UserSummary) -> Self {
        self.owner = Some(value);
        self
    }

    pub fn parent_account(mut self, value: AccountParent) -> Self {
        self.parent_account = Some(value);
        self
    }

    pub fn payment_controls(mut self, value: AccountPaymentControls) -> Self {
        self.payment_controls = Some(value);
        self
    }

    pub fn privacy_policy(mut self, value: File) -> Self {
        self.privacy_policy = Some(value);
        self
    }

    pub fn product_tax_code(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.product_tax_code = Some(value);
        self
    }

    pub fn recommended_actions(mut self, value: Vec<AccountRecommendedAction>) -> Self {
        self.recommended_actions = Some(value);
        self
    }

    pub fn require2fa(mut self, value: bool) -> Self {
        self.require2fa = Some(value);
        self
    }

    pub fn required_actions(mut self, value: Vec<AccountRequiredAction>) -> Self {
        self.required_actions = Some(value);
        self
    }

    pub fn return_policy(mut self, value: File) -> Self {
        self.return_policy = Some(value);
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

    pub fn social_links(mut self, value: Vec<AccountSocialLink>) -> Self {
        self.social_links = Some(value);
        self
    }

    pub fn stablecoin_rails(mut self, value: bool) -> Self {
        self.stablecoin_rails = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn status_reason(mut self, value: impl Into<String>) -> Self {
        self.status_reason = Some(value.into());
        self
    }

    pub fn store_page_config(mut self, value: AccountStorePageConfig) -> Self {
        self.store_page_config = Some(value);
        self
    }

    pub fn target_audience(mut self, value: impl Into<String>) -> Self {
        self.target_audience = Some(value.into());
        self
    }

    pub fn tax_collection_enabled_states(mut self, value: Vec<String>) -> Self {
        self.tax_collection_enabled_states = Some(value);
        self
    }

    pub fn tax_identifiers(mut self, value: Vec<AccountTaxIdentifier>) -> Self {
        self.tax_identifiers = Some(value);
        self
    }

    pub fn tax_remitted_by(mut self, value: AccountTaxRemittedBy) -> Self {
        self.tax_remitted_by = Some(value);
        self
    }

    pub fn tax_type(mut self, value: AccountTaxType) -> Self {
        self.tax_type = Some(value);
        self
    }

    pub fn terms_of_service(mut self, value: File) -> Self {
        self.terms_of_service = Some(value);
        self
    }

    pub fn three_ds_level(mut self, value: AccountThreeDsLevel) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn total_earned_usd(mut self, value: f64) -> Self {
        self.total_earned_usd = Some(value);
        self
    }

    pub fn total_usd(mut self, value: impl Into<String>) -> Self {
        self.total_usd = Some(value.into());
        self
    }

    pub fn use_logo_as_opengraph_image_fallback(mut self, value: bool) -> Self {
        self.use_logo_as_opengraph_image_fallback = Some(value);
        self
    }

    pub fn verification(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.verification = Some(value);
        self
    }

    pub fn volume_usd(mut self, value: f64) -> Self {
        self.volume_usd = Some(value);
        self
    }

    pub fn wallet(mut self, value: AccountWallet) -> Self {
        self.wallet = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Account`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balances`](AccountBuilder::balances)
    /// - [`can_transfer_pending_balance_to_children`](AccountBuilder::can_transfer_pending_balance_to_children)
    /// - [`collect_vat_id`](AccountBuilder::collect_vat_id)
    /// - [`company_formation`](AccountBuilder::company_formation)
    /// - [`created_at`](AccountBuilder::created_at)
    /// - [`home_preferences`](AccountBuilder::home_preferences)
    /// - [`id`](AccountBuilder::id)
    /// - [`metadata`](AccountBuilder::metadata)
    /// - [`owner`](AccountBuilder::owner)
    /// - [`require2fa`](AccountBuilder::require2fa)
    /// - [`route`](AccountBuilder::route)
    /// - [`send_customer_emails`](AccountBuilder::send_customer_emails)
    /// - [`show_joined_whops`](AccountBuilder::show_joined_whops)
    /// - [`show_reviews_dtc`](AccountBuilder::show_reviews_dtc)
    /// - [`show_user_directory`](AccountBuilder::show_user_directory)
    /// - [`social_links`](AccountBuilder::social_links)
    /// - [`stablecoin_rails`](AccountBuilder::stablecoin_rails)
    /// - [`store_page_config`](AccountBuilder::store_page_config)
    /// - [`tax_collection_enabled_states`](AccountBuilder::tax_collection_enabled_states)
    /// - [`tax_identifiers`](AccountBuilder::tax_identifiers)
    /// - [`title`](AccountBuilder::title)
    /// - [`use_logo_as_opengraph_image_fallback`](AccountBuilder::use_logo_as_opengraph_image_fallback)
    /// - [`verification`](AccountBuilder::verification)
    pub fn build(self) -> Result<Account, BuildError> {
        Ok(Account {
            balances: self
                .balances
                .ok_or_else(|| BuildError::missing_field("balances"))?,
            banner_image_url: self.banner_image_url,
            business_address: self.business_address,
            business_name: self.business_name,
            business_type: self.business_type,
            can_transfer_pending_balance_to_children: self
                .can_transfer_pending_balance_to_children
                .ok_or_else(|| {
                    BuildError::missing_field("can_transfer_pending_balance_to_children")
                })?,
            capabilities: self.capabilities,
            cards: self.cards,
            collect_vat_id: self
                .collect_vat_id
                .ok_or_else(|| BuildError::missing_field("collect_vat_id"))?,
            company_formation: self
                .company_formation
                .ok_or_else(|| BuildError::missing_field("company_formation"))?,
            country: self.country,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            email: self.email,
            eula: self.eula,
            home_preferences: self
                .home_preferences
                .ok_or_else(|| BuildError::missing_field("home_preferences"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            industry_group: self.industry_group,
            industry_type: self.industry_type,
            invoice_prefix: self.invoice_prefix,
            logo_url: self.logo_url,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            onboarding_type: self.onboarding_type,
            opengraph_image_url: self.opengraph_image_url,
            opengraph_image_variant: self.opengraph_image_variant,
            other_business_description: self.other_business_description,
            other_industry_description: self.other_industry_description,
            owner: self
                .owner
                .ok_or_else(|| BuildError::missing_field("owner"))?,
            parent_account: self.parent_account,
            payment_controls: self.payment_controls,
            privacy_policy: self.privacy_policy,
            product_tax_code: self.product_tax_code,
            recommended_actions: self.recommended_actions,
            require2fa: self
                .require2fa
                .ok_or_else(|| BuildError::missing_field("require2fa"))?,
            required_actions: self.required_actions,
            return_policy: self.return_policy,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            send_customer_emails: self
                .send_customer_emails
                .ok_or_else(|| BuildError::missing_field("send_customer_emails"))?,
            show_joined_whops: self
                .show_joined_whops
                .ok_or_else(|| BuildError::missing_field("show_joined_whops"))?,
            show_reviews_dtc: self
                .show_reviews_dtc
                .ok_or_else(|| BuildError::missing_field("show_reviews_dtc"))?,
            show_user_directory: self
                .show_user_directory
                .ok_or_else(|| BuildError::missing_field("show_user_directory"))?,
            social_links: self
                .social_links
                .ok_or_else(|| BuildError::missing_field("social_links"))?,
            stablecoin_rails: self
                .stablecoin_rails
                .ok_or_else(|| BuildError::missing_field("stablecoin_rails"))?,
            status: self.status,
            status_reason: self.status_reason,
            store_page_config: self
                .store_page_config
                .ok_or_else(|| BuildError::missing_field("store_page_config"))?,
            target_audience: self.target_audience,
            tax_collection_enabled_states: self
                .tax_collection_enabled_states
                .ok_or_else(|| BuildError::missing_field("tax_collection_enabled_states"))?,
            tax_identifiers: self
                .tax_identifiers
                .ok_or_else(|| BuildError::missing_field("tax_identifiers"))?,
            tax_remitted_by: self.tax_remitted_by,
            tax_type: self.tax_type,
            terms_of_service: self.terms_of_service,
            three_ds_level: self.three_ds_level,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            total_earned_usd: self.total_earned_usd,
            total_usd: self.total_usd,
            use_logo_as_opengraph_image_fallback: self
                .use_logo_as_opengraph_image_fallback
                .ok_or_else(|| BuildError::missing_field("use_logo_as_opengraph_image_fallback"))?,
            verification: self
                .verification
                .ok_or_else(|| BuildError::missing_field("verification"))?,
            volume_usd: self.volume_usd,
            wallet: self.wallet,
        })
    }
}
