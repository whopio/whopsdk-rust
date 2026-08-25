pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCheckoutSessionsRequest {
    /// The affiliate this checkout is attributed to. Write-once — set it here or never.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_code: Option<String>,
    /// String-to-string acquisition context. Recognized keys: `utm_source`, `utm_medium`, `utm_campaign`, `utm_term`, `utm_content`, `tracking_link_id`, `funnel_id`, `source`, `country`; anything else is dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<HashMap<String, serde_json::Value>>,
    /// A seller's checkout configuration (`ch_…`) to open this checkout from. Its plan, mode, affiliate code, metadata, redirect URL, 3DS level and payment method configuration seed the session; anything you also send explicitly wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_configuration: Option<String>,
    /// What the buyer is purchasing. Exactly one entry today — more are refused until multi-item checkout ships; the array shape is the forward contract. Alongside a `checkout_configuration` or `link` it may only name that mount's own plan, where it sets quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreateCheckoutSessionsRequestItemsItem>>,
    /// Any checkout link the seller has shared, resolved for you: a plan ID, a checkout configuration ID, a vanity short link (send `page_route` with it), a membership transfer code, or a checkout link the seller handed out earlier. A link that is not a checkout link is refused with a coded message rather than a bare not-found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Free-form string-to-string map, at most 40 keys. Whop never interprets it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// Defaults to the checkout configuration's mode, then `payment`. `setup` sessions are not yet available and are refused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CreateCheckoutSessionsRequestMode>,
    /// Where this checkout is being opened from — the scheme and host of your page, with no path (`https://shop.example.com`). Ignored when the request carries a browser `Origin` header, which is used instead. Recorded against the session as acquisition context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// The product route a vanity `link` belongs to — the `pageRoute` in the seller's shared URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_route: Option<String>,
    /// The password for a password-protected plan. Right, and the gate is cleared for the session's whole life; wrong or omitted, and the session still opens — it publishes a `custom_password` requirement, the answer arrives through update, and confirm refuses until it is right.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// A promo code to apply to the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code: Option<String>,
    /// Where the buyer lands after an off-site payment step. Absolute https URL without credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// An existing membership (`mem_…`) this checkout pays against instead of creating a new one — the buyer pays the plan's price again onto something they already own. Ownership is checked at confirm, against the buyer who confirms: a membership they do not own is refused as not found. Cannot accompany a membership transfer link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_up_membership: Option<String>,
    /// First-party tracking-link candidates keyed by account ID. Ignored outside Whop's hosted checkout; an explicit `attribution.tracking_link_id` wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_link_ids_by_account: Option<HashMap<String, Option<String>>>,
}

impl CreateCheckoutSessionsRequest {
    pub fn builder() -> CreateCheckoutSessionsRequestBuilder {
        <CreateCheckoutSessionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCheckoutSessionsRequestBuilder {
    affiliate_code: Option<String>,
    attribution: Option<HashMap<String, serde_json::Value>>,
    checkout_configuration: Option<String>,
    items: Option<Vec<CreateCheckoutSessionsRequestItemsItem>>,
    link: Option<String>,
    metadata: Option<HashMap<String, Option<String>>>,
    mode: Option<CreateCheckoutSessionsRequestMode>,
    origin: Option<String>,
    page_route: Option<String>,
    password: Option<String>,
    promo_code: Option<String>,
    return_url: Option<String>,
    top_up_membership: Option<String>,
    tracking_link_ids_by_account: Option<HashMap<String, Option<String>>>,
}

impl CreateCheckoutSessionsRequestBuilder {
    pub fn affiliate_code(mut self, value: impl Into<String>) -> Self {
        self.affiliate_code = Some(value.into());
        self
    }

    pub fn attribution(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attribution = Some(value);
        self
    }

    pub fn checkout_configuration(mut self, value: impl Into<String>) -> Self {
        self.checkout_configuration = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<CreateCheckoutSessionsRequestItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn mode(mut self, value: CreateCheckoutSessionsRequestMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn origin(mut self, value: impl Into<String>) -> Self {
        self.origin = Some(value.into());
        self
    }

    pub fn page_route(mut self, value: impl Into<String>) -> Self {
        self.page_route = Some(value.into());
        self
    }

    pub fn password(mut self, value: impl Into<String>) -> Self {
        self.password = Some(value.into());
        self
    }

    pub fn promo_code(mut self, value: impl Into<String>) -> Self {
        self.promo_code = Some(value.into());
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn top_up_membership(mut self, value: impl Into<String>) -> Self {
        self.top_up_membership = Some(value.into());
        self
    }

    pub fn tracking_link_ids_by_account(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.tracking_link_ids_by_account = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCheckoutSessionsRequest`].
    pub fn build(self) -> Result<CreateCheckoutSessionsRequest, BuildError> {
        Ok(CreateCheckoutSessionsRequest {
            affiliate_code: self.affiliate_code,
            attribution: self.attribution,
            checkout_configuration: self.checkout_configuration,
            items: self.items,
            link: self.link,
            metadata: self.metadata,
            mode: self.mode,
            origin: self.origin,
            page_route: self.page_route,
            password: self.password,
            promo_code: self.promo_code,
            return_url: self.return_url,
            top_up_membership: self.top_up_membership,
            tracking_link_ids_by_account: self.tracking_link_ids_by_account,
        })
    }
}
