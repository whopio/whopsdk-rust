pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckoutSession {
    /// The affiliate this checkout is attributed to, or `null`. Set at create only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_code: Option<String>,
    #[serde(default)]
    pub available_currencies: Vec<String>,
    /// The seller's checkout appearance, resolved configuration first, then plan, then account. `null` when nobody set any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<CheckoutSessionBranding>,
    /// The buyer's email, once one has been resolved for this checkout — `null` until then. Read-only, and read it together with `buyer_identity`: an address alone does not mean the person holds it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_email: Option<String>,
    /// The buyer's user account (`user_…`), once one has been resolved for this checkout — `null` until then. Read-only, and read it together with `buyer_identity`: `attributed` names the account the purchase is for and proves nothing about who is at the keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    /// How well this checkout knows its buyer, or `null` before one is resolved. `attributed` means an account was matched from what the buyer typed — it says who the purchase is for and nothing about who is at the keyboard. `authenticated` means the person proved they hold that account during this checkout. Only `authenticated` may be handed anything that acts as the buyer, and the value only strengthens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_identity: Option<CheckoutSessionBuyerIdentity>,
    /// When the buyer's identity was last established or strengthened, as an ISO 8601 timestamp. `null` before a buyer is resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_identity_at: Option<String>,
    /// How the buyer's identity was established — the mechanism behind `buyer_identity`, so a checkout stays explicable long after it completed. `null` before a buyer is resolved. New mechanisms are added over time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_identity_method: Option<CheckoutSessionBuyerIdentityMethod>,
    /// The seller's checkout configuration this session was mounted from (`ch_…`), or `null` when it was opened directly from a plan. Its presets seeded this session at create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_configuration: Option<String>,
    /// The session's one credential — returned by create, and echoed on a session read authenticated with it (or with the checkout's own resume cookie, which holds the same value). Every other call authenticates with it; treat it like a password for this checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// Whether this seller offers tax ID entry on checkout (their VAT ID setting). When `false`, hide the input — a stored `tax_id` still applies either way.
    #[serde(default)]
    pub collect_tax_id: bool,
    /// When the session was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The verb for the button that confirms this checkout, so every surface names the act the same way: `pay`, `subscribe`, `start_trial`, `join_waitlist`, or `continue` when nothing is charged today (a free checkout, `setup` mode saving a payment method, or a transfer that charges nothing). Render your own wording for each value — this is a key, never display text — and fall back to a generic label on a value you do not recognize.
    pub cta_label: CheckoutSessionCtaLabel,
    #[serde(default)]
    pub custom_field_responses: Vec<CheckoutSessionCustomFieldResponse>,
    /// The currency this checkout is priced AND charged in, lowercase. Seeded at create from where the buyer is; update it to one of `available_currencies` to price and charge in that currency instead.
    #[serde(default)]
    pub display_currency: String,
    /// The waitlist entry the confirm created, or `null`. Only a waitlist plan produces one: joining charges nothing — the payment method is saved and the seller charges it if they accept the entry — so a completed waitlist session carries an `entry` and a `null` payment. Read it with its `status`: `succeeded` means the join stands; `failed` (the card save died — the buyer is not on the waitlist) is transient, reopening the session on the next read with the failure on `last_confirm_error`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<CheckoutSessionEntry>,
    /// When the session expires, as an ISO 8601 timestamp. An expired session cannot be updated or confirmed — start a new one.
    #[serde(default)]
    pub expires_at: String,
    /// Checkout session ID, prefixed `chs_`.
    #[serde(default)]
    pub id: String,
    /// When this checkout's invoice is due, as an ISO 8601 timestamp — `null` for everything that is not an invoice checkout, and for an invoice without a due date. Present when the plan collects a seller-issued invoice; a surface should state the date, and a date in the past reads as overdue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_due_at: Option<String>,
    #[serde(default)]
    pub items: Vec<CheckoutSessionItem>,
    /// Why the most recent confirm attempt did not complete, or `null`. Only terminal refusals land here — anything the buyer can resolve is a `next_action` instead. The session stays open; fix what the code names and confirm again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_confirm_error: Option<CheckoutSessionConfirmError>,
    /// Free-form string-to-string map set at create. Whop never interprets it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// `payment` charges the items at confirm; `setup` saves a payment method for later use.
    pub mode: CheckoutSessionMode,
    /// The one thing to do now, or `null` when there is nothing. Do it, re-read the session, and repeat until it is `null`. A `completed` session with no `next_action` has nothing left to do — whether the checkout WENT THROUGH is the result's own `status` (`payment.status`, or `entry.status` for a waitlist join): a charge that decisively dies REOPENS the session on the next read, with the failure on `last_confirm_error` — so a `failed` result is a transient reading, never a resting state, and nothing about it may be presented as a success. `type` picks the shape, and each type carries only its own fields: switch on `type` and the payload it needs is there; a type you do not recognize is safe to skip when it is not blocking. `blocking` says whether the flow may move past it — read it rather than inferring it from the type. Before the charge, blocking means the buyer cannot pay until it is done. `complete` is the attempt's own unfinished ceremony — a 3D Secure challenge, a bank redirect — and one executor serves it whatever its `kind`: hand its `client_secret` to the payments element's `handleNextAction`. After it, `wait_for_payment` is blocking because the money is still moving and everything past that point would tell the buyer their order landed — hold, wait `poll_after_seconds`, and read again. The post-completion actions (`upgrade_authentication`, `redirect`, `await_claim`) are advisory: the purchase stands either way. After completion the action can also depend on WHO is reading: a signed-in buyer may be handed the `redirect` where an anonymous read of the same session gets `await_claim` — told to check their email to claim the purchase, or that the checkout finished on another device — and a read from outside whop.com's own pages (an embedded element) may be handed a `redirect` whose destination is whop.com's checkout-finish page instead of the seller's: execute it like any other redirect, and treat the URL as the credential it carries. This is an instruction, never the gate: confirm re-checks everything for itself, so a client that skips a blocking action is refused all the same. `upgrade_authentication` carries its own recipe (`email`, `sign_in_intent`); when its `session_intent_id` is non-null, the reader's own mid-checkout sign-in already proved the first factor and the recipe collapses to its last step — call that session intent's upgrade endpoint directly (the credential the sign-in installed authorizes it) and collect only what the login still owes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<CheckoutSessionNextAction>,
    /// Always `checkout_session`.
    #[serde(default)]
    pub object: String,
    /// The payment the confirm created, or `null` while none exists. Read it with its `status`: a completed session's payment can still be `requires_action` (a step remains) or `processing` (accepted, settling). `failed` is a transient reading — a charge that decisively dies reopens the session on the next read, unbinding the dead payment and recording the failure on `last_confirm_error`. Anything the charge still needs comes through `next_action`, which is also where the payment's own credential lives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<CheckoutSessionPayment>,
    /// The configuration governing this session, resolved through every layer — the checkout configuration it was mounted from, the plan's, and the account's. Apply it over the payment method types catalogue for the offerable set. `null` means nothing is configured at any layer: platform defaults apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<CheckoutSessionPaymentMethodConfiguration>,
    /// Whether confirm needs a `confirmation_token`. `false` when no charge will ever take money — a free plan, a transfer of a non-renewing plan, or an applied promo code that makes every charge free, now and forever: collect no payment method there and confirm without a token. Live like every session fact: applying or removing a code can flip it, so mount your payment surface off this field, never off a zero total — a checkout that charges nothing today but something later (a trial, a first-charge-only code) stays `true`.
    #[serde(default)]
    pub payment_method_required: bool,
    /// The buyer's phone number, or `null`. Collected when the session publishes a `phone_number` requirement (the seller collects numbers), set through update, and recorded against the order. Buyer-typed and unverified — sellers who VERIFY numbers get the `verify_phone` next action instead, which writes to the buyer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// What the applied code takes off and which charges it covers, or `null` when no code applies. `promo_code` is the writable field; this is its resolved semantics — the duration says which future charges the breakdown's `upcoming` amounts have the discount in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo: Option<CheckoutSessionPromo>,
    /// The promo code applied to the quote, or `null`. Set it via update; the discount shows up in the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code: Option<String>,
    /// The current price of the items, computed by the server on every read. Confirm charges this number while it is fresh.
    pub quote: CheckoutSessionQuote,
    /// Where the SELLER sends the buyer after a completed purchase, when the checkout configuration this session was mounted from sets one. Always an absolute HTTP or HTTPS URL — anything else serializes as `null`. Distinct from `return_url`, which is the payment provider's return leg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(default)]
    pub requirements: Vec<CheckoutSessionRequirement>,
    /// Where the buyer lands after an off-site payment step. Must be an absolute https URL without credentials (http is allowed for localhost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// The seller this checkout purchases from.
    #[serde(default)]
    pub seller: CheckoutSessionSeller,
    /// Where physical goods ship, or `null`. Only the keys the buyer supplied are present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CheckoutSessionShippingAddress>,
    /// Whether to offer promo code entry on this checkout: `false` on a membership transfer, a checkout with nothing due, or a one-off purchase — no code can apply — and when the seller disabled codes for it, or none exists to enter. A code already on the session still discounts either way.
    #[serde(default)]
    pub show_promo_code_input: bool,
    /// `open` until a confirm succeeds (`completed`) or the session ages out (`expired`). Only an `open` session can be updated or confirmed. A `completed` session whose charge later decisively dies returns to `open` with the failure on `last_confirm_error` — the same session takes the retry.
    pub status: CheckoutSessionStatus,
    /// Whether this checkout collects tax, and how its price reads when it does. `null` means no tax is collected here — skip `calculate_tax` entirely and ask for no more address than the payment method itself needs. `exclusive` means tax is ADDED to the quote, `inclusive` that the quote already contains it. Present before any address is known, so a surface can decide what to collect up front; the value is what this checkout expects to price with, and `calculate_tax` answers with the authoritative one once a location is known (tax behaviour varies by country).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CheckoutSessionTaxBehavior>,
    /// The buyer's tax registration for a business purchase, or `null`. Set it via update (`null` clears it); a stored registration re-prices the quote's tax row — a valid EU B2B id reverse-charges EU VAT — and confirm charges tax under the same registration, so the shown total and the taken one agree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<CheckoutSessionTaxId>,
    /// The 3D Secure behavior the checkout configuration asks for, or `null` to use the plan's or the account's default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<CheckoutSessionThreeDsLevel>,
    /// The membership this checkout tops up (`mem_…`), or `null` for an ordinary purchase. Set at create only. Confirming charges the plan's price against that existing membership instead of creating a new one — no stock is taken, and for an expiring plan the paid time stacks onto what is left. The buyer must own it: confirm resolves it against the resolved buyer's own memberships and refuses anything else as not found.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_up_membership: Option<String>,
    /// The membership transfer this checkout performs, or `null` for an ordinary purchase. Present when the session was mounted from a transfer link: confirming moves the linked membership to this buyer instead of creating a fresh purchase, the quote prices zero due today, and for a renewing plan the collected payment method takes over the origin's future renewals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<CheckoutSessionTransfer>,
}

impl CheckoutSession {
    pub fn builder() -> CheckoutSessionBuilder {
        <CheckoutSessionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionBuilder {
    affiliate_code: Option<String>,
    available_currencies: Option<Vec<String>>,
    branding: Option<CheckoutSessionBranding>,
    buyer_email: Option<String>,
    buyer_id: Option<String>,
    buyer_identity: Option<CheckoutSessionBuyerIdentity>,
    buyer_identity_at: Option<String>,
    buyer_identity_method: Option<CheckoutSessionBuyerIdentityMethod>,
    checkout_configuration: Option<String>,
    client_secret: Option<String>,
    collect_tax_id: Option<bool>,
    created_at: Option<String>,
    cta_label: Option<CheckoutSessionCtaLabel>,
    custom_field_responses: Option<Vec<CheckoutSessionCustomFieldResponse>>,
    display_currency: Option<String>,
    entry: Option<CheckoutSessionEntry>,
    expires_at: Option<String>,
    id: Option<String>,
    invoice_due_at: Option<String>,
    items: Option<Vec<CheckoutSessionItem>>,
    last_confirm_error: Option<CheckoutSessionConfirmError>,
    metadata: Option<HashMap<String, Option<String>>>,
    mode: Option<CheckoutSessionMode>,
    next_action: Option<CheckoutSessionNextAction>,
    object: Option<String>,
    payment: Option<CheckoutSessionPayment>,
    payment_method_configuration: Option<CheckoutSessionPaymentMethodConfiguration>,
    payment_method_required: Option<bool>,
    phone_number: Option<String>,
    promo: Option<CheckoutSessionPromo>,
    promo_code: Option<String>,
    quote: Option<CheckoutSessionQuote>,
    redirect_url: Option<String>,
    requirements: Option<Vec<CheckoutSessionRequirement>>,
    return_url: Option<String>,
    seller: Option<CheckoutSessionSeller>,
    shipping_address: Option<CheckoutSessionShippingAddress>,
    show_promo_code_input: Option<bool>,
    status: Option<CheckoutSessionStatus>,
    tax_behavior: Option<CheckoutSessionTaxBehavior>,
    tax_id: Option<CheckoutSessionTaxId>,
    three_ds_level: Option<CheckoutSessionThreeDsLevel>,
    top_up_membership: Option<String>,
    transfer: Option<CheckoutSessionTransfer>,
}

impl CheckoutSessionBuilder {
    pub fn affiliate_code(mut self, value: impl Into<String>) -> Self {
        self.affiliate_code = Some(value.into());
        self
    }

    pub fn available_currencies(mut self, value: Vec<String>) -> Self {
        self.available_currencies = Some(value);
        self
    }

    pub fn branding(mut self, value: CheckoutSessionBranding) -> Self {
        self.branding = Some(value);
        self
    }

    pub fn buyer_email(mut self, value: impl Into<String>) -> Self {
        self.buyer_email = Some(value.into());
        self
    }

    pub fn buyer_id(mut self, value: impl Into<String>) -> Self {
        self.buyer_id = Some(value.into());
        self
    }

    pub fn buyer_identity(mut self, value: CheckoutSessionBuyerIdentity) -> Self {
        self.buyer_identity = Some(value);
        self
    }

    pub fn buyer_identity_at(mut self, value: impl Into<String>) -> Self {
        self.buyer_identity_at = Some(value.into());
        self
    }

    pub fn buyer_identity_method(mut self, value: CheckoutSessionBuyerIdentityMethod) -> Self {
        self.buyer_identity_method = Some(value);
        self
    }

    pub fn checkout_configuration(mut self, value: impl Into<String>) -> Self {
        self.checkout_configuration = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn collect_tax_id(mut self, value: bool) -> Self {
        self.collect_tax_id = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn cta_label(mut self, value: CheckoutSessionCtaLabel) -> Self {
        self.cta_label = Some(value);
        self
    }

    pub fn custom_field_responses(
        mut self,
        value: Vec<CheckoutSessionCustomFieldResponse>,
    ) -> Self {
        self.custom_field_responses = Some(value);
        self
    }

    pub fn display_currency(mut self, value: impl Into<String>) -> Self {
        self.display_currency = Some(value.into());
        self
    }

    pub fn entry(mut self, value: CheckoutSessionEntry) -> Self {
        self.entry = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn invoice_due_at(mut self, value: impl Into<String>) -> Self {
        self.invoice_due_at = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<CheckoutSessionItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn last_confirm_error(mut self, value: CheckoutSessionConfirmError) -> Self {
        self.last_confirm_error = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn mode(mut self, value: CheckoutSessionMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn next_action(mut self, value: CheckoutSessionNextAction) -> Self {
        self.next_action = Some(value);
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn payment(mut self, value: CheckoutSessionPayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn payment_method_configuration(
        mut self,
        value: CheckoutSessionPaymentMethodConfiguration,
    ) -> Self {
        self.payment_method_configuration = Some(value);
        self
    }

    pub fn payment_method_required(mut self, value: bool) -> Self {
        self.payment_method_required = Some(value);
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn promo(mut self, value: CheckoutSessionPromo) -> Self {
        self.promo = Some(value);
        self
    }

    pub fn promo_code(mut self, value: impl Into<String>) -> Self {
        self.promo_code = Some(value.into());
        self
    }

    pub fn quote(mut self, value: CheckoutSessionQuote) -> Self {
        self.quote = Some(value);
        self
    }

    pub fn redirect_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_url = Some(value.into());
        self
    }

    pub fn requirements(mut self, value: Vec<CheckoutSessionRequirement>) -> Self {
        self.requirements = Some(value);
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn seller(mut self, value: CheckoutSessionSeller) -> Self {
        self.seller = Some(value);
        self
    }

    pub fn shipping_address(mut self, value: CheckoutSessionShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn show_promo_code_input(mut self, value: bool) -> Self {
        self.show_promo_code_input = Some(value);
        self
    }

    pub fn status(mut self, value: CheckoutSessionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tax_behavior(mut self, value: CheckoutSessionTaxBehavior) -> Self {
        self.tax_behavior = Some(value);
        self
    }

    pub fn tax_id(mut self, value: CheckoutSessionTaxId) -> Self {
        self.tax_id = Some(value);
        self
    }

    pub fn three_ds_level(mut self, value: CheckoutSessionThreeDsLevel) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    pub fn top_up_membership(mut self, value: impl Into<String>) -> Self {
        self.top_up_membership = Some(value.into());
        self
    }

    pub fn transfer(mut self, value: CheckoutSessionTransfer) -> Self {
        self.transfer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSession`].
    /// This method will fail if any of the following fields are not set:
    /// - [`available_currencies`](CheckoutSessionBuilder::available_currencies)
    /// - [`collect_tax_id`](CheckoutSessionBuilder::collect_tax_id)
    /// - [`created_at`](CheckoutSessionBuilder::created_at)
    /// - [`cta_label`](CheckoutSessionBuilder::cta_label)
    /// - [`custom_field_responses`](CheckoutSessionBuilder::custom_field_responses)
    /// - [`display_currency`](CheckoutSessionBuilder::display_currency)
    /// - [`expires_at`](CheckoutSessionBuilder::expires_at)
    /// - [`id`](CheckoutSessionBuilder::id)
    /// - [`items`](CheckoutSessionBuilder::items)
    /// - [`mode`](CheckoutSessionBuilder::mode)
    /// - [`object`](CheckoutSessionBuilder::object)
    /// - [`payment_method_required`](CheckoutSessionBuilder::payment_method_required)
    /// - [`quote`](CheckoutSessionBuilder::quote)
    /// - [`requirements`](CheckoutSessionBuilder::requirements)
    /// - [`seller`](CheckoutSessionBuilder::seller)
    /// - [`show_promo_code_input`](CheckoutSessionBuilder::show_promo_code_input)
    /// - [`status`](CheckoutSessionBuilder::status)
    pub fn build(self) -> Result<CheckoutSession, BuildError> {
        Ok(CheckoutSession {
            affiliate_code: self.affiliate_code,
            available_currencies: self
                .available_currencies
                .ok_or_else(|| BuildError::missing_field("available_currencies"))?,
            branding: self.branding,
            buyer_email: self.buyer_email,
            buyer_id: self.buyer_id,
            buyer_identity: self.buyer_identity,
            buyer_identity_at: self.buyer_identity_at,
            buyer_identity_method: self.buyer_identity_method,
            checkout_configuration: self.checkout_configuration,
            client_secret: self.client_secret,
            collect_tax_id: self
                .collect_tax_id
                .ok_or_else(|| BuildError::missing_field("collect_tax_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            cta_label: self
                .cta_label
                .ok_or_else(|| BuildError::missing_field("cta_label"))?,
            custom_field_responses: self
                .custom_field_responses
                .ok_or_else(|| BuildError::missing_field("custom_field_responses"))?,
            display_currency: self
                .display_currency
                .ok_or_else(|| BuildError::missing_field("display_currency"))?,
            entry: self.entry,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            invoice_due_at: self.invoice_due_at,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            last_confirm_error: self.last_confirm_error,
            metadata: self.metadata,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            next_action: self.next_action,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            payment: self.payment,
            payment_method_configuration: self.payment_method_configuration,
            payment_method_required: self
                .payment_method_required
                .ok_or_else(|| BuildError::missing_field("payment_method_required"))?,
            phone_number: self.phone_number,
            promo: self.promo,
            promo_code: self.promo_code,
            quote: self
                .quote
                .ok_or_else(|| BuildError::missing_field("quote"))?,
            redirect_url: self.redirect_url,
            requirements: self
                .requirements
                .ok_or_else(|| BuildError::missing_field("requirements"))?,
            return_url: self.return_url,
            seller: self
                .seller
                .ok_or_else(|| BuildError::missing_field("seller"))?,
            shipping_address: self.shipping_address,
            show_promo_code_input: self
                .show_promo_code_input
                .ok_or_else(|| BuildError::missing_field("show_promo_code_input"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            tax_behavior: self.tax_behavior,
            tax_id: self.tax_id,
            three_ds_level: self.three_ds_level,
            top_up_membership: self.top_up_membership,
            transfer: self.transfer,
        })
    }
}
