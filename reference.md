# Reference
## AccessTokens
<details><summary><code>client.access_tokens.<a href="/src/api/resources/access_tokens/client.rs">create</a>(request: CreateAccessTokensRequest) -> Result&lt;AccessToken, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a short-lived access token for authenticating API requests. When using API key authentication, provide company_id or user_id. When using OAuth, the user is derived from the token. Use this token with Whop's web and mobile embedded components.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .access_tokens
        .create(
            &CreateAccessTokensRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `Option<Option<String>>` — The unique identifier of the company to generate the token for, starting with 'biz_'. The API key must have permission to access this company.
    
</dd>
</dl>

<dl>
<dd>

**expires_at:** `Option<Option<String>>` — The expiration timestamp for the access token. Defaults to 1 hour from now, with a maximum of 3 hours.
    
</dd>
</dl>

<dl>
<dd>

**scoped_actions:** `Option<Option<Vec<String>>>` — An array of permission scopes to grant to the access token. If empty or omitted, all permissions from the authenticating credential are inherited. Must be a subset of the credential's permissions.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>` — The unique identifier of the user to generate the token for, starting with 'user_'. The API key must have permission to access this user.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AccountLinks
<details><summary><code>client.account_links.<a href="/src/api/resources/account_links/client.rs">create</a>(request: CreateAccountLinksRequest) -> Result&lt;AccountLink, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generate a URL that directs a sub-merchant to their account portal, such as the hosted payouts dashboard or the KYC onboarding flow.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .account_links
        .create(
            &CreateAccountLinksRequest {
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                refresh_url: "refresh_url".to_string(),
                return_url: "return_url".to_string(),
                use_case: AccountLinkUseCases::AccountOnboarding,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to generate the link for, starting with 'biz_'. Must be a sub-merchant of the API key's company.
    
</dd>
</dl>

<dl>
<dd>

**refresh_url:** `String` — The URL to redirect the user to if the session expires and needs to be re-authenticated, such as 'https://example.com/refresh'.
    
</dd>
</dl>

<dl>
<dd>

**return_url:** `String` — The URL to redirect the user to when they want to return to your site, such as 'https://example.com/return'.
    
</dd>
</dl>

<dl>
<dd>

**use_case:** `AccountLinkUseCases` — The purpose of the account link, such as hosted payouts portal or hosted KYC onboarding.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Accounts
<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">list</a>(first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListAccountsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListAccountsRequestDirection&gt;&gt;, status: Option&lt;Option&lt;ListAccountsRequestStatus&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, volume_min: Option&lt;Option&lt;f64&gt;&gt;, volume_max: Option&lt;Option&lt;f64&gt;&gt;, parent_account_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListAccountsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists accounts visible to the credential. User tokens return the user's business accounts; Account API keys return the requesting account and its connected accounts. Pass `parent_account_id` to return only that parent account's connected accounts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .accounts
        .list(
            &AccountsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**first:** `Option<i64>` — The number of accounts to return (default 10, max 50).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns accounts after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of accounts to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns accounts before this position.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListAccountsRequestOrder>` — The field to sort accounts by. `volume` requires `stats:read` on the parent account.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListAccountsRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListAccountsRequestStatus>` — Return only accounts with this status: `active` (includes accounts that have not entered payments review) or `suspended`.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Free-text filter on account title or ID. `%` and `_` match literally.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Return only accounts created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Return only accounts created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**volume_min:** `Option<f64>` — Return only accounts whose lifetime USD volume is at least this value. Requires `stats:read` on the parent account.
    
</dd>
</dl>

<dl>
<dd>

**volume_max:** `Option<f64>` — Return only accounts whose lifetime USD volume is at most this value. Requires `stats:read` on the parent account.
    
</dd>
</dl>

<dl>
<dd>

**parent_account_id:** `Option<String>` — For platforms: the parent account ID whose direct connected accounts to return. Requires `payout:account:read` on the parent account.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">create</a>(request: CreateAccountsRequest) -> Result&lt;Account, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates an account. User tokens create business accounts; Account API keys create connected accounts. Tax fields (`tax_remitted_by`, `tax_type`, `product_tax_code_id`, `business_address`, `tax_identifiers`) are configured with Update Account, not at creation.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .accounts
        .create(
            &CreateAccountsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**affiliate_code:** `Option<Option<String>>` — The username, if any, of the partner who referred this account
    
</dd>
</dl>

<dl>
<dd>

**blueprint_id:** `Option<Option<String>>` — The blueprint App ID, prefixed `app_`. Creates a hosted website for the account and queues its deployment asynchronously; the Account response does not report deployment completion.
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — The ISO 3166-1 alpha-2 country code where the account's business is located (e.g. `US`). Defaults to the parent account's country for connected accounts.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — The email address of the account owner. Required for Account API key requests.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Arbitrary key/value metadata to store on the account.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The display name of the account. Defaults to `metadata.external_id` or the owner's email when omitted.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">me</a>() -> Result&lt;Account, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the account associated with the current Account API key.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.accounts.me(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">retrieve</a>(id: String) -> Result&lt;Account, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single account by ID or public route when it is visible to the credential, including its crypto wallet. The reserved id `me` retrieves the account associated with the current Account API key; user tokens have no single account, so they must address one by ID or route.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.accounts.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Account ID, prefixed `biz_`, its public route, or `me` for the account associated with the current API key.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">update</a>(id: String, request: UpdateAccountsRequest) -> Result&lt;Account, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an account. User tokens can update business accounts; Account API keys can update connected accounts. The reserved id `me` — accepted on Retrieve Account — resolves to the requesting account, which an Account API key cannot edit, so updates must name the connected account by its `biz_` id.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .accounts
        .update(
            &"id".to_string(),
            &UpdateAccountsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Account ID, prefixed `biz_`.
    
</dd>
</dl>

<dl>
<dd>

**affiliate_application_required:** `Option<bool>` — Whether prospective affiliates must submit an application before promoting this account.
    
</dd>
</dl>

<dl>
<dd>

**affiliate_instructions:** `Option<Option<String>>` — Guidelines shown to affiliates promoting this account.
    
</dd>
</dl>

<dl>
<dd>

**banner_image:** `Option<Option<UpdateAccountsRequestBannerImage>>` — Account banner image, used as the cover photo when creating a Whop-managed Facebook page. Image files up to 10 MB, except `image/gif`. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file).
    
</dd>
</dl>

<dl>
<dd>

**business_address:** `Option<UpdateAccountsRequestBusinessAddress>` — Account business address used to calculate tax. A complete address in a supported country is required when `tax_remitted_by` is `self`.
    
</dd>
</dl>

<dl>
<dd>

**business_name:** `Option<Option<String>>` — The legal business name used with the account's tax address.
    
</dd>
</dl>

<dl>
<dd>

**business_type:** `Option<Option<UpdateAccountsRequestBusinessType>>` — High-level business category for the account. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
    
</dd>
</dl>

<dl>
<dd>

**collect_vat_id:** `Option<bool>` — Whether checkout shows a VAT/tax ID field for buyers to optionally enter. Does not require a VAT ID to purchase.
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<Option<String>>` — Country where the account is located.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — Account promotional description. When creating a Whop-managed Facebook page, it is truncated to 155 characters and used as the About text.
    
</dd>
</dl>

<dl>
<dd>

**featured_affiliate_product_id:** `Option<Option<String>>` — The ID of the product to feature for affiliates. Pass `null` to clear.
    
</dd>
</dl>

<dl>
<dd>

**home_preferences:** `Option<Vec<UpdateAccountsRequestHomePreferencesItem>>` — Public account home page preferences.
    
</dd>
</dl>

<dl>
<dd>

**industry_group:** `Option<Option<UpdateAccountsRequestIndustryGroup>>` — Account industry group. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
    
</dd>
</dl>

<dl>
<dd>

**industry_type:** `Option<Option<String>>` — Specific industry vertical for the account. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
    
</dd>
</dl>

<dl>
<dd>

**invoice_prefix:** `Option<Option<String>>` — Prefix used for account invoices.
    
</dd>
</dl>

<dl>
<dd>

**logo:** `Option<Option<UpdateAccountsRequestLogo>>` — Account logo, used as the profile picture when creating a Whop-managed Facebook page. Image files up to 5 MB. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file).
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Arbitrary key/value metadata to store on the account.
    
</dd>
</dl>

<dl>
<dd>

**onboarding_type:** `Option<Option<UpdateAccountsRequestOnboardingType>>` — The type of onboarding the account has completed.
    
</dd>
</dl>

<dl>
<dd>

**opengraph_image:** `Option<Option<UpdateAccountsRequestOpengraphImage>>` — Open Graph preview media used when the account is shared. Image and video files up to 5 MB. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file).
    
</dd>
</dl>

<dl>
<dd>

**opengraph_image_variant:** `Option<Option<UpdateAccountsRequestOpengraphImageVariant>>` — The account Open Graph image variant.
    
</dd>
</dl>

<dl>
<dd>

**other_business_description:** `Option<Option<String>>` — The description of the business type when business_type is other.
    
</dd>
</dl>

<dl>
<dd>

**other_industry_description:** `Option<Option<String>>` — The description of the industry type when industry_type is other.
    
</dd>
</dl>

<dl>
<dd>

**product_tax_code_id:** `Option<Option<String>>` — ID of the tax classification code applied by default to the account's products. See the available [product categories](https://docs.numeral.com/essentials/product-categories).
    
</dd>
</dl>

<dl>
<dd>

**require2fa:** `Option<bool>` — Whether the account requires authorized users to have two-factor authentication enabled.
    
</dd>
</dl>

<dl>
<dd>

**route:** `Option<Option<String>>` — The unique URL slug for the account.
    
</dd>
</dl>

<dl>
<dd>

**send_customer_emails:** `Option<bool>` — Whether Whop sends transactional emails to customers on behalf of this account.
    
</dd>
</dl>

<dl>
<dd>

**show_joined_whops:** `Option<bool>` — Whether the account appears in joined whops on other accounts.
    
</dd>
</dl>

<dl>
<dd>

**show_reviews_dtc:** `Option<bool>` — Whether reviews are displayed on direct-to-consumer product pages.
    
</dd>
</dl>

<dl>
<dd>

**show_user_directory:** `Option<bool>` — Whether the account shows users in the user directory.
    
</dd>
</dl>

<dl>
<dd>

**social_links:** `Option<Vec<std::collections::HashMap<String, serde_json::Value>>>` — The full list of social links to display for the account.
    
</dd>
</dl>

<dl>
<dd>

**store_page_config:** `Option<Option<UpdateAccountsRequestStorePageConfig>>` — Account store page display configuration.
    
</dd>
</dl>

<dl>
<dd>

**target_audience:** `Option<Option<String>>` — The target audience for this account.
    
</dd>
</dl>

<dl>
<dd>

**tax_collection_enabled_states:** `Option<Vec<UpdateAccountsRequestTaxCollectionEnabledStatesItem>>` — US state codes (50 states plus `DC`) where the account collects tax. Replaces the full set on update. Only settable when `tax_remitted_by` is `self`.
    
</dd>
</dl>

<dl>
<dd>

**tax_identifiers:** `Option<Vec<UpdateAccountsRequestTaxIdentifiersItem>>` — Account tax/VAT registrations to add or update. When `tax_remitted_by` is `self`, tax is calculated and collected only in the countries where the account holds a registration.
    
</dd>
</dl>

<dl>
<dd>

**tax_remitted_by:** `Option<UpdateAccountsRequestTaxRemittedBy>` — Determines whether Whop or the account calculates and remits tax. The account must provide a supported-country business address when it self-remits.
    
</dd>
</dl>

<dl>
<dd>

**tax_type:** `Option<UpdateAccountsRequestTaxType>` — Determines whether tax is included in the listed price or added at checkout.
    
</dd>
</dl>

<dl>
<dd>

**three_ds_level:** `Option<Option<UpdateAccountsRequestThreeDsLevel>>` — Account-level 3D Secure behavior. Set `mandate_challenge` to require cardholder verification on supported card payments, or `null` to use the standard checkout flow.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The display name of the account.
    
</dd>
</dl>

<dl>
<dd>

**use_logo_as_opengraph_image_fallback:** `Option<bool>` — Whether the account uses its logo as the fallback Open Graph image.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">form_company</a>(id: String, request: FormCompanyAccountsRequest) -> Result&lt;FormCompanyAccountsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Starts an LLC or C-Corp formation for a business account. Defaults to an LLC; set `entity_type` to `c_corp` to form a C-Corp, which additionally requires `share_structure` and officer `roles` on every founder. On submission, the application is validated and the response returns a hosted checkout URL. Once paid, the filing is submitted. Track progress through the account's [`company_formation`](/api-reference/beta/accounts/retrieve-account) field on Retrieve Account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .accounts
        .form_company(
            &"id".to_string(),
            &FormCompanyAccountsRequest {
                business_address: Some(FormCompanyAccountsRequestBusinessAddress {
                    city: "Austin".to_string(),
                    country: "US".to_string(),
                    line1: "4180 Burnet Rd".to_string(),
                    line2: Some("Suite 2".to_string()),
                    postal_code: "78756".to_string(),
                    state: "TX".to_string(),
                    ..Default::default()
                }),
                business_name: "Shine Time Auto Detailing".to_string(),
                business_phone: Some("+15125550142".to_string()),
                business_type: "brick_and_mortar".to_string(),
                business_website: Some("https://shinetime.example".to_string()),
                entity_suffix: Some(FormCompanyAccountsRequestEntitySuffix::Llc),
                entity_type: Some(FormCompanyAccountsRequestEntityType::Llc),
                expedite_ein: Some(true),
                formation_state: FormCompanyAccountsRequestFormationState::Tx,
                founders: vec![FormCompanyAccountsRequestFoundersItem {
                    address: FormCompanyAccountsRequestFoundersItemAddress {
                        city: "Austin".to_string(),
                        country: "US".to_string(),
                        line1: "907 Ridgemont Dr".to_string(),
                        line2: Some("Apt 4".to_string()),
                        postal_code: "78704".to_string(),
                        state: "TX".to_string(),
                        ..Default::default()
                    },
                    date_of_birth: Some("1988-03-14".to_string()),
                    email: "marcus@shinetime.example".to_string(),
                    first_name: "Marcus".to_string(),
                    is_primary: true,
                    last_name: "Webb".to_string(),
                    ownership_percentage: Some(100.0),
                    phone: "+15125550142".to_string(),
                    roles: Some(vec![
                        FormCompanyAccountsRequestFoundersItemRolesItem::President,
                    ]),
                    ssn: Some("123-45-6789".to_string()),
                    ..Default::default()
                }],
                industry_group: "automotive".to_string(),
                industry_type: "car_wash".to_string(),
                share_structure: Some(FormCompanyAccountsRequestShareStructure {
                    number_of_shares: 123,
                    value: 123.0,
                    ..Default::default()
                }),
                use_registered_agent: Some(true),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Account ID, prefixed `biz_`.
    
</dd>
</dl>

<dl>
<dd>

**business_address:** `Option<FormCompanyAccountsRequestBusinessAddress>` — Company mailing address. Required unless `use_registered_agent` is `true`.
    
</dd>
</dl>

<dl>
<dd>

**business_name:** `String` — Legal name for the new company.
    
</dd>
</dl>

<dl>
<dd>

**business_phone:** `Option<String>` — Business phone number in E.164 format, for example `+12125550100`. Required unless `use_registered_agent` is `true`.
    
</dd>
</dl>

<dl>
<dd>

**business_type:** `String` — High-level business category, from the Whop business taxonomy. Valid values are listed on [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary).
    
</dd>
</dl>

<dl>
<dd>

**business_website:** `Option<String>` — Company website URL.
    
</dd>
</dl>

<dl>
<dd>

**entity_suffix:** `Option<FormCompanyAccountsRequestEntitySuffix>` — Legal entity ending appended to `business_name`. LLC formations accept `LLC`, `L.L.C`, `L.L.C.` or `Limited Liability Company` and default to `LLC`; C-Corp formations accept `Inc`, `Inc.`, `Incorporated`, `Corp.`, `Corporation`, `C Corp`, `C Corporation`, `CCorp` or `Company` and default to `Inc.`. Unrecognized values fall back to the default for the entity type.
    
</dd>
</dl>

<dl>
<dd>

**entity_type:** `Option<FormCompanyAccountsRequestEntityType>` — Legal entity type to form. Defaults to `llc`.
    
</dd>
</dl>

<dl>
<dd>

**expedite_ein:** `Option<bool>` — Request expedited EIN processing for an additional fee. Available only when no founder supplies an SSN.
    
</dd>
</dl>

<dl>
<dd>

**formation_state:** `FormCompanyAccountsRequestFormationState` — Two-letter code of the US state (or `DC`) to form the company in.
    
</dd>
</dl>

<dl>
<dd>

**founders:** `Vec<FormCompanyAccountsRequestFoundersItem>` — The company's founders. Exactly one must be marked `is_primary` — the responsible party for the filing.
    
</dd>
</dl>

<dl>
<dd>

**industry_group:** `String` — Industry group, from the Whop business taxonomy. Valid values are listed on [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary).
    
</dd>
</dl>

<dl>
<dd>

**industry_type:** `String` — Specific industry vertical, from the Whop business taxonomy. Valid values are listed on [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary).
    
</dd>
</dl>

<dl>
<dd>

**share_structure:** `Option<FormCompanyAccountsRequestShareStructure>` — Authorized share structure. Required when `entity_type` is `c_corp`; ignored for LLCs.
    
</dd>
</dl>

<dl>
<dd>

**use_registered_agent:** `Option<bool>` — Use the registered agent's address as the company address instead of `business_address`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.accounts.<a href="/src/api/resources/accounts/client.rs">transfer_ownership</a>(id: String, request: TransferOwnershipAccountsRequest) -> Result&lt;TransferOwnershipAccountsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Transfers ownership of the account to another user, identified by user ID or email address. If the recipient already holds the owner role, ownership moves immediately; otherwise they get an invite and ownership moves when they accept.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .accounts
        .transfer_ownership(
            &"id".to_string(),
            &TransferOwnershipAccountsRequest {
                identifier: "marcus@shinetime.example".to_string(),
                as_partner: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**as_partner:** `Option<bool>` — If true, the current owner is credited as the account's Whop partner, earning partner commission on its sales. Requires the current owner to already be an enrolled Whop partner. Skipped if the account already has an active partner.
    
</dd>
</dl>

<dl>
<dd>

**identifier:** `String` — The user to transfer ownership to: a user ID (`user_*`) or an email address. An email address with no Whop account yet is sent an invite to create one.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Ad Campaigns
<details><summary><code>client.ad_campaigns.<a href="/src/api/resources/ad_campaigns/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListAdCampaignsRequestStatus&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListAdCampaignsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListAdCampaignsRequestDirection&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, stats_from: Option&lt;Option&lt;String&gt;&gt;, stats_to: Option&lt;Option&lt;String&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;, attribution_model: Option&lt;Option&lt;ListAdCampaignsRequestAttributionModel&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListAdCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the ad campaigns for an account, with stats over the requested window.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_campaigns
        .list(
            &AdCampaignsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The account the campaigns belong to. Defaults to the account-scoped key's own account.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListAdCampaignsRequestStatus>` — Only return campaigns with this status.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Filter campaigns by a title or ID substring.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListAdCampaignsRequestOrder>` — The field to sort by. Defaults to created_at. Stat columns (spend, impressions, …) rank over the stats_from/stats_to window across the whole list, not just the current page. results, cost_per_result and return_on_ad_spend rank by the same Whop pixel-attributed values the response reports.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListAdCampaignsRequestDirection>` — The sort direction. Defaults to desc.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return campaigns created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return campaigns created after this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**stats_from:** `Option<String>` — Start of the stats window. Defaults to all-time.
    
</dd>
</dl>

<dl>
<dd>

**stats_to:** `Option<String>` — End of the stats window. Defaults to now.
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — IANA timezone (e.g. America/New_York) the stats window is interpreted in. Bare stats_from/stats_to dates resolve to day boundaries on this clock. Defaults to UTC.
    
</dd>
</dl>

<dl>
<dd>

**attribution_model:** `Option<ListAdCampaignsRequestAttributionModel>` — Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of campaigns to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of campaigns to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to fetch the page before (from page_info.start_cursor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_campaigns.<a href="/src/api/resources/ad_campaigns/client.rs">create</a>(request: CreateAdCampaignsRequest) -> Result&lt;AdCampaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates an ad campaign for an account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_campaigns
        .create(
            &CreateAdCampaignsRequest {
                objective: CreateAdCampaignsRequestObjective::Awareness,
                platform: CreateAdCampaignsRequestPlatform::Meta,
                title: "Now hiring mobile detailers — Austin".to_string(),
                account_id: None,
                bid_type: None,
                budget_amount: None,
                budget_optimization: None,
                budget_type: None,
                desired_cost_per_result: None,
                ends_at: None,
                special_ad_categories: None,
                starts_at: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The account to create the campaign under. Defaults to the account-scoped key's own account.
    
</dd>
</dl>

<dl>
<dd>

**bid_type:** `Option<CreateAdCampaignsRequestBidType>` — How delivery bids in the ad auction: `minimum_cost` gets the most results for the budget, `average_target` holds an average cost per result, `maximum_target` never bids above a cap. Only for campaigns that own the budget.
    
</dd>
</dl>

<dl>
<dd>

**budget_amount:** `Option<f64>` — The campaign's budget, in the ad account's currency. Required when budget_optimization is `ad_campaign`; omit when each ad group sets its own budget.
    
</dd>
</dl>

<dl>
<dd>

**budget_optimization:** `Option<CreateAdCampaignsRequestBudgetOptimization>` — Which level owns the budget: the whole campaign (`ad_campaign`) or each ad group individually (`ad_group`). Defaults to `ad_group`.
    
</dd>
</dl>

<dl>
<dd>

**budget_type:** `Option<CreateAdCampaignsRequestBudgetType>` — Whether the budget is spent per day (`daily`) or over the campaign's full run (`lifetime`). Defaults to `daily`.
    
</dd>
</dl>

<dl>
<dd>

**desired_cost_per_result:** `Option<f64>` — Cost per result to aim for (`average_target`) or never exceed (`maximum_target`). Only for campaigns that own the budget.
    
</dd>
</dl>

<dl>
<dd>

**ends_at:** `Option<String>` — When the campaign stops delivering, as an ISO 8601 timestamp. Only for campaigns that own the budget.
    
</dd>
</dl>

<dl>
<dd>

**objective:** `CreateAdCampaignsRequestObjective` — The goal the campaign optimizes toward.
    
</dd>
</dl>

<dl>
<dd>

**platform:** `CreateAdCampaignsRequestPlatform` — The ad network the campaign runs on.
    
</dd>
</dl>

<dl>
<dd>

**special_ad_categories:** `Option<Vec<CreateAdCampaignsRequestSpecialAdCategoriesItem>>` — Regulated categories the campaign falls under. Ads in these categories are subject to extra targeting restrictions.
    
</dd>
</dl>

<dl>
<dd>

**starts_at:** `Option<String>` — When the campaign starts delivering, as an ISO 8601 timestamp. Only for campaigns that own the budget.
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` — The title of the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_campaigns.<a href="/src/api/resources/ad_campaigns/client.rs">retrieve</a>(id: String, stats_from: Option&lt;Option&lt;String&gt;&gt;, stats_to: Option&lt;Option&lt;String&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;, attribution_model: Option&lt;Option&lt;RetrieveAdCampaignsRequestAttributionModel&gt;&gt;) -> Result&lt;AdCampaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single ad campaign with stats over the requested window.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_campaigns
        .retrieve(
            &"id".to_string(),
            &AdCampaignsRetrieveQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad campaign ID.
    
</dd>
</dl>

<dl>
<dd>

**stats_from:** `Option<String>` — Start of the stats window.
    
</dd>
</dl>

<dl>
<dd>

**stats_to:** `Option<String>` — End of the stats window.
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — IANA timezone the stats window is interpreted in. Defaults to UTC.
    
</dd>
</dl>

<dl>
<dd>

**attribution_model:** `Option<RetrieveAdCampaignsRequestAttributionModel>` — Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_campaigns.<a href="/src/api/resources/ad_campaigns/client.rs">delete</a>(id: String) -> Result&lt;DeleteAdCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an ad campaign and archives it on the ad platform (cascades to ad groups and ads).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.ad_campaigns.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad campaign ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_campaigns.<a href="/src/api/resources/ad_campaigns/client.rs">update</a>(id: String, request: UpdateAdCampaignsRequest) -> Result&lt;AdCampaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an ad campaign's editable fields (title, budget, schedule, bid strategy, special ad categories, and, before launch, budget type and budget optimization), and launches a draft campaign by setting status to active. Objective and desired cost per result are fixed at creation and cannot be changed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_campaigns
        .update(
            &"id".to_string(),
            &UpdateAdCampaignsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad campaign ID.
    
</dd>
</dl>

<dl>
<dd>

**bid_type:** `Option<UpdateAdCampaignsRequestBidType>` — How delivery bids in the ad auction: `minimum_cost` gets the most results for the budget, `average_target` holds an average cost per result, `maximum_target` never bids above a cap. Switching to `minimum_cost` clears the cap amounts stored on the campaign's ad groups. Only for campaigns that own the budget.
    
</dd>
</dl>

<dl>
<dd>

**budget_amount:** `Option<f64>` — The campaign budget, in the account's currency. Interpreted as daily or lifetime per the campaign's budget type, including a budget_type sent in the same request.
    
</dd>
</dl>

<dl>
<dd>

**budget_optimization:** `Option<UpdateAdCampaignsRequestBudgetOptimization>` — Which level owns the budget: the whole campaign (`ad_campaign`) or each ad group individually (`ad_group`). Only changeable before the campaign is live on the ad network; switching to `ad_campaign` requires budget_amount in the same request, and switching to `ad_group` clears the campaign budget.
    
</dd>
</dl>

<dl>
<dd>

**budget_type:** `Option<UpdateAdCampaignsRequestBudgetType>` — Whether `budget_amount` is spent per day (`daily`) or over the campaign's full run (`lifetime`). Only changeable while the campaign is a draft; send budget_amount in the same request so the amount lands on the new type.
    
</dd>
</dl>

<dl>
<dd>

**ends_at:** `Option<String>` — When the campaign stops delivering, as an ISO 8601 timestamp. Only for campaigns that own the budget.
    
</dd>
</dl>

<dl>
<dd>

**special_ad_categories:** `Option<Vec<UpdateAdCampaignsRequestSpecialAdCategoriesItem>>` — Regulated categories the campaign falls under. Editable on any campaign, draft or launched; pass an empty array to clear.
    
</dd>
</dl>

<dl>
<dd>

**starts_at:** `Option<String>` — When the campaign starts delivering, as an ISO 8601 timestamp. Only for campaigns that own the budget.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<UpdateAdCampaignsRequestStatus>` — Set to active to launch a draft campaign (moderates and pushes it live). Live-campaign pause and resume use the pause and unpause actions.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The name of the campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_campaigns.<a href="/src/api/resources/ad_campaigns/client.rs">duplicate</a>(id: String, request: DuplicateAdCampaignsRequest) -> Result&lt;DuplicateAdCampaignsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates copies of the campaign in `duplicating` status and returns them; each copy transitions to `draft` once duplication completes. Poll each returned campaign until it leaves `duplicating` — a copy that could not be completed is deleted and returns 404.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_campaigns
        .duplicate(
            &"id".to_string(),
            &DuplicateAdCampaignsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad campaign ID.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — Number of copies to create (1-10). Defaults to 1.
    
</dd>
</dl>

<dl>
<dd>

**preserve_engagement:** `Option<bool>` — Whether the copied ads keep the original posts' engagement (likes, comments, shares). Defaults to false.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_campaigns.<a href="/src/api/resources/ad_campaigns/client.rs">pause</a>(id: String) -> Result&lt;AdCampaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Pauses an active ad campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.ad_campaigns.pause(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad campaign ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_campaigns.<a href="/src/api/resources/ad_campaigns/client.rs">retry_payment</a>(id: String) -> Result&lt;AdCampaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retries billing for an ad campaign whose payment previously failed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_campaigns
        .retry_payment(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad campaign ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_campaigns.<a href="/src/api/resources/ad_campaigns/client.rs">unpause</a>(id: String) -> Result&lt;AdCampaign, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Resumes a paused ad campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.ad_campaigns.unpause(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad campaign ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Ad Groups
<details><summary><code>client.ad_groups.<a href="/src/api/resources/ad_groups/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, ad_campaign_id: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListAdGroupsRequestStatus&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListAdGroupsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListAdGroupsRequestDirection&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, stats_from: Option&lt;Option&lt;String&gt;&gt;, stats_to: Option&lt;Option&lt;String&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;, attribution_model: Option&lt;Option&lt;ListAdGroupsRequestAttributionModel&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListAdGroupsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists ad groups for the account, newest first.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_groups
        .list(
            &AdGroupsListQueryRequest {
                ad_campaign_ids: vec![Some("adcamp_xxxxxxxxxxxxxx".to_string())],
                account_id: None,
                ad_campaign_id: None,
                status: None,
                query: None,
                order: None,
                direction: None,
                created_before: None,
                created_after: None,
                stats_from: None,
                stats_to: None,
                time_zone: None,
                attribution_model: None,
                first: None,
                after: None,
                last: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account whose ad groups to list. Defaults to the authenticated account.
    
</dd>
</dl>

<dl>
<dd>

**ad_campaign_id:** `Option<String>` — Filter to ad groups in this campaign.
    
</dd>
</dl>

<dl>
<dd>

**ad_campaign_ids:** `Option<String>` — Filter to ad groups in these campaigns (max 100). Repeat the parameter for each id (ad_campaign_ids=a&ad_campaign_ids=b).
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListAdGroupsRequestStatus>` — Filter to ad groups with this status.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Filter ad groups by a title or ID substring.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListAdGroupsRequestOrder>` — The field to sort by. Defaults to created_at. Stat columns (spend, impressions, …) rank over the stats_from/stats_to window across the whole list, not just the current page. results, cost_per_result and return_on_ad_spend rank by the same Whop pixel-attributed values the response reports.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListAdGroupsRequestDirection>` — The sort direction. Defaults to desc.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return ad groups created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return ad groups created after this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**stats_from:** `Option<String>` — Start of the stats window. Defaults to all-time.
    
</dd>
</dl>

<dl>
<dd>

**stats_to:** `Option<String>` — End of the stats window. Defaults to now.
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — IANA timezone (e.g. America/New_York) the stats window is interpreted in. Bare stats_from/stats_to dates resolve to day boundaries on this clock. Defaults to UTC.
    
</dd>
</dl>

<dl>
<dd>

**attribution_model:** `Option<ListAdGroupsRequestAttributionModel>` — Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of ad groups to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of ad groups to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to fetch the page before (from page_info.start_cursor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_groups.<a href="/src/api/resources/ad_groups/client.rs">create</a>(request: CreateAdGroupsRequest) -> Result&lt;AdGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates an ad group (ad set) in a campaign.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_groups
        .create(
            &CreateAdGroupsRequest {
                ad_campaign_id: "adcamp_xxxxxxxxxxxxxx".to_string(),
                audiences: None,
                bid_type: None,
                budget_amount: None,
                budget_type: None,
                conversion_event: None,
                conversion_location: None,
                demographics: None,
                desired_cost_per_result: None,
                detailed_targeting: None,
                devices: None,
                dynamic_creative: None,
                ends_at: None,
                frequency_cap: None,
                languages: None,
                message_apps: None,
                minimum_daily_spend: None,
                optimization_goal: None,
                placements: None,
                regions: None,
                starts_at: None,
                status: None,
                title: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ad_campaign_id:** `String` — The ad campaign to create the ad group in, prefixed `adcamp_`.
    
</dd>
</dl>

<dl>
<dd>

**audiences:** `Option<AdGroupAudiencesBody>` — Saved audiences to deliver to or exclude. Can't be combined with demographics.automatic.
    
</dd>
</dl>

<dl>
<dd>

**bid_type:** `Option<CreateAdGroupsRequestBidType>` — How delivery bids are set in the ad auction. Target-based strategies use `desired_cost_per_result`.
    
</dd>
</dl>

<dl>
<dd>

**budget_amount:** `Option<f64>` — This ad group's budget, in the ad account's currency. Omit when the budget is set on the campaign instead.
    
</dd>
</dl>

<dl>
<dd>

**budget_type:** `Option<CreateAdGroupsRequestBudgetType>` — Whether budget_amount is spent per day (`daily`) or over the ad group's full run (`lifetime`).
    
</dd>
</dl>

<dl>
<dd>

**conversion_event:** `Option<Option<ConversionEvent>>` 
    
</dd>
</dl>

<dl>
<dd>

**conversion_location:** `Option<CreateAdGroupsRequestConversionLocation>` — Where the outcome being optimized for occurs, such as a website visit, social-profile visit, messaging conversation, ad interaction, or lead-form submission. The lead form itself is set on the ad.
    
</dd>
</dl>

<dl>
<dd>

**demographics:** `Option<AdGroupDemographicsBody>` — Age, gender, and automatic-audience targeting.
    
</dd>
</dl>

<dl>
<dd>

**desired_cost_per_result:** `Option<f64>` — Cost per result to aim for (`average_target`) or never exceed (`maximum_target`).
    
</dd>
</dl>

<dl>
<dd>

**detailed_targeting:** `Option<AdGroupDetailedTargetingBody>` — Interest, behavior, and demographic targeting, using categories from the ad platform's targeting taxonomy. Entries across interests, behaviors, and demographics are OR'd together (anyone matching any entry is reached), matching Ads Manager's detailed-targeting box. At most 100 entries per section. Can't be combined with demographics.automatic, and unavailable to campaigns with special_ad_categories. Send the complete intended state — a section you omit is cleared.
    
</dd>
</dl>

<dl>
<dd>

**devices:** `Option<AdGroupDevicesBody>` — Device platforms and operating systems to target.
    
</dd>
</dl>

<dl>
<dd>

**dynamic_creative:** `Option<bool>` — Let the ad platform automatically mix and match this ad group's creatives and copy to find the best-performing combinations. Set at creation; can't be changed afterward.
    
</dd>
</dl>

<dl>
<dd>

**ends_at:** `Option<String>` — When the ad group stops delivering, as an ISO 8601 timestamp. Omit to run until paused.
    
</dd>
</dl>

<dl>
<dd>

**frequency_cap:** `Option<CreateAdGroupsRequestFrequencyCap>` — Cap on how often one person sees ads from this ad group. Only available on campaigns with the `awareness` objective.
    
</dd>
</dl>

<dl>
<dd>

**languages:** `Option<Vec<String>>` — Languages to target, as ISO 639 codes such as `en` or `es`. Empty or omitted targets all languages.
    
</dd>
</dl>

<dl>
<dd>

**message_apps:** `Option<Vec<CreateAdGroupsRequestMessageAppsItem>>` — Apps the conversation opens in. Required when setting `conversion_location` to `messaging`, and rejected unless the ad group's conversion location is `messaging`.
    
</dd>
</dl>

<dl>
<dd>

**minimum_daily_spend:** `Option<f64>` — Minimum the ad group tries to spend each day.
    
</dd>
</dl>

<dl>
<dd>

**optimization_goal:** `Option<CreateAdGroupsRequestOptimizationGoal>` — The result the ad group's delivery is optimized to get the most of.
    
</dd>
</dl>

<dl>
<dd>

**placements:** `Option<CreateAdGroupsRequestPlacements>` 

`automatic` to let the ad platform choose placements, or the list of platforms and positions to target. Omit a platform's positions to target all of them.

Valid positions per platform:

- `facebook`: `feed`, `right_hand_column`, `marketplace`, `search`, `profile_feed`, `notification`, `story`, `instream_video`, `facebook_reels`, `facebook_reels_overlay`, `biz_disco_feed`
- `instagram`: `stream`, `story`, `explore`, `explore_home`, `reels`, `profile_feed`, `profile_reels`, `ig_search`
- `messenger`: `story`
- `audience_network`: `classic`, `rewarded_video`
- `threads`: `threads_stream`
- `whatsapp`: `status`
    
</dd>
</dl>

<dl>
<dd>

**regions:** `Option<AdGroupRegionsBody>` — Locations to target and exclude.
    
</dd>
</dl>

<dl>
<dd>

**starts_at:** `Option<String>` — When the ad group starts delivering, as an ISO 8601 timestamp. Omit to start as soon as it's active.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<CreateAdGroupsRequestStatus>` — Initial status (default: `active`).
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The display name of the ad group.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_groups.<a href="/src/api/resources/ad_groups/client.rs">estimate_reach</a>(request: EstimateReachAdGroupsRequest) -> Result&lt;ReachEstimate, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Estimates how many people a draft targeting spec can reach, before an ad group is created. The body takes the same targeting fields as creating an ad group — `regions`, `demographics`, `detailed_targeting`, `audiences`, `languages`, and `devices` — and nothing is persisted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_groups
        .estimate_reach(
            &EstimateReachAdGroupsRequest {
                platform: EstimateReachAdGroupsRequestPlatform::Meta,
                account_id: None,
                audiences: None,
                demographics: None,
                detailed_targeting: None,
                devices: None,
                languages: None,
                regions: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account to estimate on behalf of. Defaults to the authenticated account.
    
</dd>
</dl>

<dl>
<dd>

**audiences:** `Option<AdGroupAudiencesBody>` — Saved audiences to deliver to or exclude. Can't be combined with demographics.automatic.
    
</dd>
</dl>

<dl>
<dd>

**demographics:** `Option<AdGroupDemographicsBody>` — Age, gender, and automatic-audience targeting.
    
</dd>
</dl>

<dl>
<dd>

**detailed_targeting:** `Option<AdGroupDetailedTargetingBody>` — Interest, behavior, and demographic targeting, using categories from the ad platform's targeting taxonomy. At most 100 entries per section.
    
</dd>
</dl>

<dl>
<dd>

**devices:** `Option<AdGroupDevicesBody>` — Device platforms and operating systems to target.
    
</dd>
</dl>

<dl>
<dd>

**languages:** `Option<Vec<String>>` — Languages to target, as ISO 639 codes such as `en` or `es`. Empty or omitted targets all languages.
    
</dd>
</dl>

<dl>
<dd>

**platform:** `EstimateReachAdGroupsRequestPlatform` — The ad network the estimate runs on.
    
</dd>
</dl>

<dl>
<dd>

**regions:** `Option<AdGroupRegionsBody>` — Locations to target and exclude.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_groups.<a href="/src/api/resources/ad_groups/client.rs">search_targeting_options</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, platform: Option&lt;SearchTargetingOptionsAdGroupsRequestPlatform&gt;, query: Option&lt;Option&lt;String&gt;&gt;, country: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;SearchTargetingOptionsAdGroupsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Searches the ad platform's targeting taxonomy for options to target an ad group with. Each result comes back in the exact shape the ad-group body accepts for its `type`, so it can be used in `detailed_targeting`, `regions`, or `languages` as-is. A blank `query` browses the small fixed lists (behaviors, browse demographic categories, languages); interests, work employers, job titles, schools, majors, and locations need a search term.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_groups
        .search_targeting_options(
            &SearchTargetingOptionsQueryRequest {
                platform: SearchTargetingOptionsAdGroupsRequestPlatform::Meta,
                account_id: None,
                query: None,
                types: vec![],
                location_types: vec![],
                country: None,
                limit: None,
                special_ad_categories: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account to search on behalf of. Defaults to the authenticated account.
    
</dd>
</dl>

<dl>
<dd>

**platform:** `SearchTargetingOptionsAdGroupsRequestPlatform` — The ad network whose targeting taxonomy to search.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — The search term. Blank browses the fixed lists; interests, work employers, job titles, schools, majors, and locations return nothing without one.
    
</dd>
</dl>

<dl>
<dd>

**types:** `Option<SearchTargetingOptionsAdGroupsRequestTypesItem>` — Kinds of targeting options to search. Defaults to all of them.
    
</dd>
</dl>

<dl>
<dd>

**location_types:** `Option<SearchTargetingOptionsAdGroupsRequestLocationTypesItem>` — Narrow location results to these kinds of places. Only applies when `types` includes `locations`.
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — Narrow location results to one country, as an ISO 3166-1 code such as `US`. Only applies when `types` includes `locations`.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Maximum number of results per requested type.
    
</dd>
</dl>

<dl>
<dd>

**special_ad_categories:** `Option<SearchTargetingOptionsAdGroupsRequestSpecialAdCategoriesItem>` — The campaign's declared special ad categories. Under `housing`, `employment`, or `financial_products` the ad platform allows interests only, drawn from a short approved list, so results are narrowed to what such a campaign can launch with and other kinds return nothing. Blank `query` browses that approved list instead of the usual fixed lists.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_groups.<a href="/src/api/resources/ad_groups/client.rs">retrieve</a>(id: String, stats_from: Option&lt;Option&lt;String&gt;&gt;, stats_to: Option&lt;Option&lt;String&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;, attribution_model: Option&lt;Option&lt;RetrieveAdGroupsRequestAttributionModel&gt;&gt;) -> Result&lt;AdGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single ad group.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_groups
        .retrieve(
            &"id".to_string(),
            &AdGroupsRetrieveQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad group ID.
    
</dd>
</dl>

<dl>
<dd>

**stats_from:** `Option<String>` — Start of the stats window.
    
</dd>
</dl>

<dl>
<dd>

**stats_to:** `Option<String>` — End of the stats window.
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — IANA timezone the stats window is interpreted in. Defaults to UTC.
    
</dd>
</dl>

<dl>
<dd>

**attribution_model:** `Option<RetrieveAdGroupsRequestAttributionModel>` — Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_groups.<a href="/src/api/resources/ad_groups/client.rs">delete</a>(id: String) -> Result&lt;DeleteAdGroupsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an ad group.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.ad_groups.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad group ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_groups.<a href="/src/api/resources/ad_groups/client.rs">update</a>(id: String, request: UpdateAdGroupsRequest) -> Result&lt;AdGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an ad group's editable fields. Only the keys you send are changed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_groups
        .update(
            &"id".to_string(),
            &UpdateAdGroupsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad group ID.
    
</dd>
</dl>

<dl>
<dd>

**audiences:** `Option<AdGroupAudiencesBody>` — Saved audiences to deliver to or exclude. Can't be combined with demographics.automatic.
    
</dd>
</dl>

<dl>
<dd>

**bid_type:** `Option<UpdateAdGroupsRequestBidType>` — How delivery bids are set in the ad auction. Target-based strategies use `desired_cost_per_result`.
    
</dd>
</dl>

<dl>
<dd>

**budget_amount:** `Option<f64>` — This ad group's budget, in the ad account's currency. Omit when the budget is set on the campaign instead.
    
</dd>
</dl>

<dl>
<dd>

**budget_type:** `Option<UpdateAdGroupsRequestBudgetType>` — Whether budget_amount is spent per day (`daily`) or over the ad group's full run (`lifetime`).
    
</dd>
</dl>

<dl>
<dd>

**conversion_event:** `Option<Option<ConversionEvent>>` 
    
</dd>
</dl>

<dl>
<dd>

**conversion_location:** `Option<UpdateAdGroupsRequestConversionLocation>` — Where the outcome being optimized for occurs, such as a website visit, social-profile visit, messaging conversation, ad interaction, or lead-form submission. The lead form itself is set on the ad.
    
</dd>
</dl>

<dl>
<dd>

**demographics:** `Option<AdGroupDemographicsBody>` — Age, gender, and automatic-audience targeting.
    
</dd>
</dl>

<dl>
<dd>

**desired_cost_per_result:** `Option<f64>` — Cost per result to aim for (`average_target`) or never exceed (`maximum_target`).
    
</dd>
</dl>

<dl>
<dd>

**detailed_targeting:** `Option<AdGroupDetailedTargetingBody>` — Interest, behavior, and demographic targeting, using categories from the ad platform's targeting taxonomy. Entries across interests, behaviors, and demographics are OR'd together (anyone matching any entry is reached), matching Ads Manager's detailed-targeting box. At most 100 entries per section. Can't be combined with demographics.automatic, and unavailable to campaigns with special_ad_categories. Send the complete intended state — a section you omit is cleared.
    
</dd>
</dl>

<dl>
<dd>

**devices:** `Option<AdGroupDevicesBody>` — Device platforms and operating systems to target.
    
</dd>
</dl>

<dl>
<dd>

**ends_at:** `Option<String>` — When the ad group stops delivering, as an ISO 8601 timestamp. Omit to run until paused.
    
</dd>
</dl>

<dl>
<dd>

**frequency_cap:** `Option<UpdateAdGroupsRequestFrequencyCap>` — Cap on how often one person sees ads from this ad group. Only available on campaigns with the `awareness` objective.
    
</dd>
</dl>

<dl>
<dd>

**languages:** `Option<Vec<String>>` — Languages to target, as ISO 639 codes such as `en` or `es`. Empty or omitted targets all languages.
    
</dd>
</dl>

<dl>
<dd>

**message_apps:** `Option<Vec<UpdateAdGroupsRequestMessageAppsItem>>` — Apps the conversation opens in. Required when setting `conversion_location` to `messaging`, and rejected unless the ad group's conversion location is `messaging`.
    
</dd>
</dl>

<dl>
<dd>

**minimum_daily_spend:** `Option<f64>` — Minimum the ad group tries to spend each day.
    
</dd>
</dl>

<dl>
<dd>

**optimization_goal:** `Option<UpdateAdGroupsRequestOptimizationGoal>` — The result the ad group's delivery is optimized to get the most of.
    
</dd>
</dl>

<dl>
<dd>

**placements:** `Option<UpdateAdGroupsRequestPlacements>` 

`automatic` to let the ad platform choose placements, or the list of platforms and positions to target. Omit a platform's positions to target all of them.

Valid positions per platform:

- `facebook`: `feed`, `right_hand_column`, `marketplace`, `search`, `profile_feed`, `notification`, `story`, `instream_video`, `facebook_reels`, `facebook_reels_overlay`, `biz_disco_feed`
- `instagram`: `stream`, `story`, `explore`, `explore_home`, `reels`, `profile_feed`, `profile_reels`, `ig_search`
- `messenger`: `story`
- `audience_network`: `classic`, `rewarded_video`
- `threads`: `threads_stream`
- `whatsapp`: `status`
    
</dd>
</dl>

<dl>
<dd>

**regions:** `Option<AdGroupRegionsBody>` — Locations to target and exclude.
    
</dd>
</dl>

<dl>
<dd>

**starts_at:** `Option<String>` — When the ad group starts delivering, as an ISO 8601 timestamp. Omit to start as soon as it's active.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<UpdateAdGroupsRequestStatus>` — Initial status (default: `active`).
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The display name of the ad group.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_groups.<a href="/src/api/resources/ad_groups/client.rs">duplicate</a>(id: String, request: DuplicateAdGroupsRequest) -> Result&lt;DuplicateAdGroupsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates copies of the ad group in `duplicating` status and returns them — into its own campaign, or into target_ad_campaign_id (which must belong to the same account and be compatible with the ad group's targeting and goals); each copy transitions to its final status (matching the source's active/paused state) once duplication completes. Poll each returned ad group until it leaves `duplicating` — a copy that could not be completed is deleted and returns 404.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_groups
        .duplicate(
            &"id".to_string(),
            &DuplicateAdGroupsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad group ID.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — Number of copies to create (1-10). Defaults to 1.
    
</dd>
</dl>

<dl>
<dd>

**preserve_engagement:** `Option<bool>` — Whether the copied ads keep the original posts' engagement (likes, comments, shares). Defaults to false.
    
</dd>
</dl>

<dl>
<dd>

**target_ad_campaign_id:** `Option<String>` — Campaign to duplicate into. Defaults to the ad group's own campaign.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_groups.<a href="/src/api/resources/ad_groups/client.rs">pause</a>(id: String) -> Result&lt;AdGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Pauses delivery of an ad group.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.ad_groups.pause(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad group ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ad_groups.<a href="/src/api/resources/ad_groups/client.rs">unpause</a>(id: String) -> Result&lt;AdGroup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Resumes delivery of a paused ad group.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.ad_groups.unpause(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad group ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AdReports
<details><summary><code>client.ad_reports.<a href="/src/api/resources/ad_reports/client.rs">retrieve</a>(breakdown: Option&lt;Option&lt;AdReportBreakdownLevels&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, currency: Option&lt;Option&lt;String&gt;&gt;, from: Option&lt;String&gt;, granularity: Option&lt;Option&lt;Granularities&gt;&gt;, to: Option&lt;String&gt;) -> Result&lt;AdReport, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Performance report for a company, ad campaigns, ad groups, or ads. Always returns aggregate `summary` totals summed across the scope. Set `granularity` to additionally get a time series, or set `breakdown` (`campaign`/`ad_group`/`ad`) to additionally get per-entity rows inside the requested scope. Exactly one of `companyId`, `adCampaignIds`, `adGroupIds`, or `adIds` must be provided.

Required permissions:
 - `ad_campaign:stats:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ad_reports
        .retrieve(
            &AdReportsRetrieveQueryRequest {
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                from: DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap(),
                to: DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap(),
                ad_campaign_ids: vec![],
                ad_group_ids: vec![],
                ad_ids: vec![],
                breakdown: None,
                currency: None,
                granularity: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ad_campaign_ids:** `Option<String>` — Scope the report to these ad campaigns (max 100); stats are summed across them. Mutually exclusive with `companyId`, `adGroupIds`, and `adIds`.
    
</dd>
</dl>

<dl>
<dd>

**ad_group_ids:** `Option<String>` — Scope the report to these ad groups (max 100); stats are summed across them. Mutually exclusive with `companyId`, `adCampaignIds`, and `adIds`.
    
</dd>
</dl>

<dl>
<dd>

**ad_ids:** `Option<String>` — Scope the report to these ads (max 100); stats are summed across them. Mutually exclusive with `companyId`, `adCampaignIds`, and `adGroupIds`.
    
</dd>
</dl>

<dl>
<dd>

**breakdown:** `Option<AdReportBreakdownLevels>` 
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of a company. Mutually exclusive with `adCampaignIds`, `adGroupIds`, and `adIds`. Use with `breakdown` to fan out across every campaign, ad group, or ad in the company without paging.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — ISO 4217 currency code to report `spend` in. Defaults to the company's ads reporting currency.
    
</dd>
</dl>

<dl>
<dd>

**from:** `String` — Inclusive start of the reporting window.
    
</dd>
</dl>

<dl>
<dd>

**granularity:** `Option<Granularities>` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `String` — Inclusive end of the reporting window.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Ads
<details><summary><code>client.ads.<a href="/src/api/resources/ads/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, ad_campaign_id: Option&lt;Option&lt;String&gt;&gt;, ad_group_id: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListAdsRequestStatus&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListAdsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListAdsRequestDirection&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, stats_from: Option&lt;Option&lt;String&gt;&gt;, stats_to: Option&lt;Option&lt;String&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;, attribution_model: Option&lt;Option&lt;ListAdsRequestAttributionModel&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListAdsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the ads for an account, with stats over the requested window.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ads
        .list(
            &AdsListQueryRequest {
                ad_campaign_ids: vec![Some("adcamp_xxxxxxxxxxxxxx".to_string())],
                ad_group_ids: vec![Some("adgrp_xxxxxxxxxxxxxx".to_string())],
                account_id: None,
                ad_campaign_id: None,
                ad_group_id: None,
                status: None,
                query: None,
                order: None,
                direction: None,
                created_before: None,
                created_after: None,
                stats_from: None,
                stats_to: None,
                time_zone: None,
                attribution_model: None,
                first: None,
                after: None,
                last: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The account the ads belong to. Defaults to the account-scoped key's own account.
    
</dd>
</dl>

<dl>
<dd>

**ad_campaign_id:** `Option<String>` — Only return ads in this ad campaign.
    
</dd>
</dl>

<dl>
<dd>

**ad_campaign_ids:** `Option<String>` — Only return ads in these ad campaigns (max 100). Repeat the parameter for each id (ad_campaign_ids=a&ad_campaign_ids=b).
    
</dd>
</dl>

<dl>
<dd>

**ad_group_id:** `Option<String>` — Only return ads in this ad group.
    
</dd>
</dl>

<dl>
<dd>

**ad_group_ids:** `Option<String>` — Only return ads in these ad groups (max 100). Repeat the parameter for each id (ad_group_ids=a&ad_group_ids=b).
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListAdsRequestStatus>` — Only return ads with this status.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Filter ads by a title or ID substring.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListAdsRequestOrder>` — The field to sort by. Defaults to created_at. Stat columns (spend, impressions, …) rank over the stats_from/stats_to window across the whole list, not just the current page. results, cost_per_result and return_on_ad_spend rank by the same Whop pixel-attributed values the response reports.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListAdsRequestDirection>` — The sort direction. Defaults to desc.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return ads created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return ads created after this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**stats_from:** `Option<String>` — Start of the stats window. Defaults to all-time.
    
</dd>
</dl>

<dl>
<dd>

**stats_to:** `Option<String>` — End of the stats window. Defaults to now.
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — IANA timezone (e.g. America/New_York) the stats window is interpreted in. Bare stats_from/stats_to dates resolve to day boundaries on this clock. Defaults to UTC.
    
</dd>
</dl>

<dl>
<dd>

**attribution_model:** `Option<ListAdsRequestAttributionModel>` — Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of ads to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of ads to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to fetch the page before (from page_info.start_cursor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ads.<a href="/src/api/resources/ads/client.rs">create</a>(request: CreateAdsRequest) -> Result&lt;Ad, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates an ad in an ad group.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ads
        .create(
            &CreateAdsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ad_group:** `Option<std::collections::HashMap<String, serde_json::Value>>` — An inline ad group to create (same shape as POST /ad_groups, including ad_campaign_id). Creates the ad group and the ad together. Provide this OR ad_group_id.
    
</dd>
</dl>

<dl>
<dd>

**ad_group_id:** `Option<String>` — The existing ad group to create the ad in. Provide this OR ad_group, not both.
    
</dd>
</dl>

<dl>
<dd>

**call_to_action:** `Option<CreateAdsRequestCallToAction>` — The call-to-action button shown on the ad.
    
</dd>
</dl>

<dl>
<dd>

**creatives:** `Option<Vec<CreateAdsRequestCreativesItem>>` — The ad's creative assets. Each entry is an uploaded file id with an optional format; omit format for the original asset. Two or more entries with no format become a carousel (2-10 attachments), in order, sharing the ad's copy.
    
</dd>
</dl>

<dl>
<dd>

**descriptions:** `Option<Vec<String>>` — The description variants shown on the ad.
    
</dd>
</dl>

<dl>
<dd>

**existing_post_id:** `Option<String>` — Promote a post you already published instead of uploading creatives — a Facebook post or Instagram media id. Mutually exclusive with creatives. Pair with post_source.
    
</dd>
</dl>

<dl>
<dd>

**headlines:** `Option<Vec<String>>` — The headline variants shown on the ad.
    
</dd>
</dl>

<dl>
<dd>

**lead_form:** `Option<CreateAdsRequestLeadForm>` — Instant lead form for the ad. Only allowed when the ad group's conversion_location is an instant-form destination (instant_forms, instant_forms_and_messenger, website_and_instant_forms). Mutually exclusive with lead_form_id.
    
</dd>
</dl>

<dl>
<dd>

**lead_form_id:** `Option<String>` — Use an existing instant form instead of creating one — the form's platform ID, from a form already on the ad's Facebook page. Only allowed when the ad group's conversion_location is an instant-form destination. Mutually exclusive with lead_form.
    
</dd>
</dl>

<dl>
<dd>

**messaging_config:** `Option<CreateAdsRequestMessagingConfig>` — Click-to-message welcome copy: the greeting (message) and the ice-breaker prompt (keyword).
    
</dd>
</dl>

<dl>
<dd>

**multi_advertiser_ads:** `Option<bool>` — Whether the ad can appear alongside other advertisers' ads in the same unit. Defaults to true.
    
</dd>
</dl>

<dl>
<dd>

**post_source:** `Option<CreateAdsRequestPostSource>` — Identifies the network that owns `existing_post_id`. The source is inferred from the ID shape when omitted.
    
</dd>
</dl>

<dl>
<dd>

**primary_texts:** `Option<Vec<String>>` — The primary text variants shown in the ad body.
    
</dd>
</dl>

<dl>
<dd>

**social_accounts:** `Option<Vec<CreateAdsRequestSocialAccountsItem>>` — The social accounts the ad runs under — a connected Facebook page and, optionally, an Instagram profile.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The display name of the ad.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — The URL the ad links to. Query parameters are merged into url_parameters, so the stored URL is always bare.
    
</dd>
</dl>

<dl>
<dd>

**url_parameters:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Query parameters to append to the destination URL, keyed by parameter name. Merged with any query string on `url`. Whop adds its own click-attribution parameters; those are reserved and rejected if you set them (utm_meta_ad_id, utm_meta_adset_id, utm_meta_campaign_id, utm_source, utm_placement, utm_medium, utm_content, utm_adset, utm_whop, wacid, wasid, waid, tw_source, tw_adid).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ads.<a href="/src/api/resources/ads/client.rs">retrieve</a>(id: String, stats_from: Option&lt;Option&lt;String&gt;&gt;, stats_to: Option&lt;Option&lt;String&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;, attribution_model: Option&lt;Option&lt;RetrieveAdsRequestAttributionModel&gt;&gt;) -> Result&lt;Ad, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single ad with stats over the requested window.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ads
        .retrieve(
            &"id".to_string(),
            &AdsRetrieveQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad ID.
    
</dd>
</dl>

<dl>
<dd>

**stats_from:** `Option<String>` — Start of the stats window.
    
</dd>
</dl>

<dl>
<dd>

**stats_to:** `Option<String>` — End of the stats window.
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — IANA timezone the stats window is interpreted in. Defaults to UTC.
    
</dd>
</dl>

<dl>
<dd>

**attribution_model:** `Option<RetrieveAdsRequestAttributionModel>` — Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ads.<a href="/src/api/resources/ads/client.rs">delete</a>(id: String) -> Result&lt;DeleteAdsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an ad.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.ads.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ads.<a href="/src/api/resources/ads/client.rs">update</a>(id: String, request: UpdateAdsRequest) -> Result&lt;Ad, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an ad's editable fields.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ads
        .update(
            &"id".to_string(),
            &UpdateAdsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad ID.
    
</dd>
</dl>

<dl>
<dd>

**call_to_action:** `Option<UpdateAdsRequestCallToAction>` — The call-to-action button shown on the ad.
    
</dd>
</dl>

<dl>
<dd>

**creatives:** `Option<Vec<UpdateAdsRequestCreativesItem>>` — The ad's creative assets. Each entry is an uploaded file id with an optional format; omit format for the original asset. Replaces a live ad's creative on the platform. Two or more entries with no format replace it with a carousel (2-10 attachments), in order, sharing the ad's copy.
    
</dd>
</dl>

<dl>
<dd>

**descriptions:** `Option<Vec<String>>` — The description variants shown on the ad.
    
</dd>
</dl>

<dl>
<dd>

**existing_post_id:** `Option<String>` — Promote a post you already published instead of uploading creatives — a Facebook post or Instagram media id. Mutually exclusive with creatives. Pair with post_source.
    
</dd>
</dl>

<dl>
<dd>

**headlines:** `Option<Vec<String>>` — The headline variants shown on the ad.
    
</dd>
</dl>

<dl>
<dd>

**lead_form:** `Option<UpdateAdsRequestLeadForm>` — Instant lead form for the ad. Only allowed when the ad group's conversion_location is an instant-form destination (instant_forms, instant_forms_and_messenger, website_and_instant_forms). Mutually exclusive with lead_form_id.
    
</dd>
</dl>

<dl>
<dd>

**lead_form_id:** `Option<String>` — Use an existing instant form instead of creating one — the form's platform ID, from a form already on the ad's Facebook page. Only allowed when the ad group's conversion_location is an instant-form destination. Mutually exclusive with lead_form. Replaces a stored lead_form.
    
</dd>
</dl>

<dl>
<dd>

**messaging_config:** `Option<UpdateAdsRequestMessagingConfig>` — Click-to-message welcome copy: the greeting (message) and the ice-breaker prompt (keyword).
    
</dd>
</dl>

<dl>
<dd>

**multi_advertiser_ads:** `Option<bool>` — Whether the ad can appear alongside other advertisers' ads in the same unit. Defaults to true.
    
</dd>
</dl>

<dl>
<dd>

**post_source:** `Option<UpdateAdsRequestPostSource>` — Identifies the network that owns `existing_post_id`. The source is inferred from the ID shape when omitted.
    
</dd>
</dl>

<dl>
<dd>

**primary_texts:** `Option<Vec<String>>` — The primary text variants shown in the ad body.
    
</dd>
</dl>

<dl>
<dd>

**social_accounts:** `Option<Vec<UpdateAdsRequestSocialAccountsItem>>` — The social accounts the ad runs under — a connected Facebook page and, optionally, an Instagram profile.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The display name of the ad.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — The URL the ad links to. Query parameters are merged into url_parameters, so the stored URL is always bare.
    
</dd>
</dl>

<dl>
<dd>

**url_parameters:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Query parameters to append to the destination URL, keyed by parameter name. Merged with any query string on `url`. Whop adds its own click-attribution parameters; those are reserved and rejected if you set them (utm_meta_ad_id, utm_meta_adset_id, utm_meta_campaign_id, utm_source, utm_placement, utm_medium, utm_content, utm_adset, utm_whop, wacid, wasid, waid, tw_source, tw_adid).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ads.<a href="/src/api/resources/ads/client.rs">duplicate</a>(id: String, request: DuplicateAdsRequest) -> Result&lt;DuplicateAdsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Copies the ad into its own ad group, or into target_ad_group_id (which must belong to the same account and be compatible with the ad). Copies keep the source ad's active/paused state.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ads
        .duplicate(
            &"id".to_string(),
            &DuplicateAdsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad ID.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — Number of copies to create (1-10). Defaults to 1.
    
</dd>
</dl>

<dl>
<dd>

**preserve_engagement:** `Option<bool>` — Whether the copies keep the original post's engagement (likes, comments, shares). Defaults to false.
    
</dd>
</dl>

<dl>
<dd>

**target_ad_group_id:** `Option<String>` — Ad group to duplicate into. Defaults to the ad's own ad group.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ads.<a href="/src/api/resources/ads/client.rs">pause</a>(id: String) -> Result&lt;Ad, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Pauses an active ad.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.ads.pause(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ads.<a href="/src/api/resources/ads/client.rs">unpause</a>(id: String) -> Result&lt;Ad, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Resumes a paused ad.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.ads.unpause(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ad ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Affiliates
<details><summary><code>client.affiliates.<a href="/src/api/resources/affiliates/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;String&gt;, direction: Option&lt;Option&lt;Direction&gt;&gt;, order: Option&lt;Option&lt;AffiliatesSortableColumns&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;Status&gt;&gt;) -> Result&lt;ListAffiliatesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of affiliates for the actor in context, with optional filtering by status, search, and sorting.

Required permissions:
 - `affiliate:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .affiliates
        .list(
            &AffiliatesListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                after: None,
                before: None,
                direction: None,
                order: None,
                query: None,
                status: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to list affiliates for.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<Direction>` 
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<AffiliatesSortableColumns>` 
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Search affiliates by username.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<Status>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.affiliates.<a href="/src/api/resources/affiliates/client.rs">create</a>(request: CreateAffiliatesRequest) -> Result&lt;Affiliate, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates or finds an affiliate for a company and user.

Required permissions:
 - `affiliate:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .affiliates
        .create(
            &CreateAffiliatesRequest {
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                user_identifier: "user_identifier".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `String` — The ID of the company to create the affiliate for.
    
</dd>
</dl>

<dl>
<dd>

**user_identifier:** `String` — The user identifier (username, email, user ID, or Discord ID).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.affiliates.<a href="/src/api/resources/affiliates/client.rs">retrieve</a>(id: String) -> Result&lt;Affiliate, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing affiliate.

Required permissions:
 - `affiliate:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .affiliates
        .retrieve(&"aff_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the affiliate.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.affiliates.<a href="/src/api/resources/affiliates/client.rs">archive</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Archives an existing Affiliate

Required permissions:
 - `affiliate:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .affiliates
        .archive(&"aff_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The internal ID of the affiliate to archive.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.affiliates.<a href="/src/api/resources/affiliates/client.rs">unarchive</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Unarchives an existing Affiliate

Required permissions:
 - `affiliate:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .affiliates
        .unarchive(&"aff_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The internal ID of the affiliate to archive.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AiChats
<details><summary><code>client.ai_chats.<a href="/src/api/resources/ai_chats/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, only_active_crons: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListAiChatsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of AI chat threads for the current authenticated user.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ai_chats
        .list(
            &AiChatsListQueryRequest {
                first: Some(42),
                last: Some(42),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**only_active_crons:** `Option<bool>` — When true, returns only chats with an active cron schedule
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ai_chats.<a href="/src/api/resources/ai_chats/client.rs">create</a>(request: CreateAiChatsRequest) -> Result&lt;AiChat, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new AI chat thread and send the first message to the AI agent.

Required permissions:
 - `ai_chat:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ai_chats
        .create(
            &CreateAiChatsRequest {
                message_text: "message_text".to_string(),
                current_company_id: None,
                message_attachments: None,
                message_source: None,
                suggestion_type: None,
                title: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**current_company_id:** `Option<Option<String>>` — The unique identifier of the company to set as context for the AI chat (e.g., "biz_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**message_attachments:** `Option<Option<Vec<CreateAiChatsRequestMessageAttachmentsItem>>>` — A list of previously uploaded file attachments to include with the first message.
    
</dd>
</dl>

<dl>
<dd>

**message_source:** `Option<Option<AiChatMessageSourceTypes>>` — The source of the message.
    
</dd>
</dl>

<dl>
<dd>

**message_text:** `String` — The text content of the first message to send to the AI agent.
    
</dd>
</dl>

<dl>
<dd>

**suggestion_type:** `Option<Option<String>>` — The type of suggestion prompt that was clicked, when message_source is 'suggestion'.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — An optional display title for the AI chat thread (e.g., "Help with billing").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ai_chats.<a href="/src/api/resources/ai_chats/client.rs">retrieve</a>(id: String) -> Result&lt;AiChat, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing AI chat.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ai_chats
        .retrieve(&"aich_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the AI chat to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ai_chats.<a href="/src/api/resources/ai_chats/client.rs">delete</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete an AI chat thread so it no longer appears in the user's chat list.

Required permissions:
 - `ai_chat:delete`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ai_chats
        .delete(&"aich_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the AI chat to delete (e.g., "ai_chat_XXXXX").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ai_chats.<a href="/src/api/resources/ai_chats/client.rs">update</a>(id: String, request: UpdateAiChatsRequest) -> Result&lt;AiChat, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update an AI chat's title, notification preferences, or associated company context.

Required permissions:
 - `ai_chat:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ai_chats
        .update(
            &"aich_xxxxxxxxxxxxx".to_string(),
            &UpdateAiChatsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the AI chat to update (e.g., "ai_chat_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**current_company_id:** `Option<Option<String>>` — The unique identifier of the company to set as context for the AI chat (e.g., "biz_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**notification_preference:** `Option<Option<AiChatNotificationPreferences>>` — The notification preference for the AI chat.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The new display title for the AI chat thread (e.g., "Help with billing").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## API Keys
<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">list</a>(resource_id: Option&lt;String&gt;, resource_type: Option&lt;ListApiKeysRequestResourceType&gt;, created_before: Option&lt;Option&lt;ListApiKeysRequestCreatedBefore&gt;&gt;, created_after: Option&lt;Option&lt;ListApiKeysRequestCreatedAfter&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListApiKeysRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListApiKeysRequestDirection&gt;&gt;) -> Result&lt;ListApiKeysResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the API keys of an account or app, newest first. Responses never include the full secret — only its obfuscated form.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .api_keys
        .list(
            &APIKeysListQueryRequest {
                resource_id: "resource_id".to_string(),
                resource_type: ListAPIKeysRequestResourceType::Account,
                created_before: None,
                created_after: None,
                first: None,
                after: None,
                last: None,
                before: None,
                order: None,
                direction: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**resource_id:** `String` — The account (`biz_`) or app (`app_`) tag to list API keys for.
    
</dd>
</dl>

<dl>
<dd>

**resource_type:** `ListApiKeysRequestResourceType` — The type of resource that owns the API keys.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<ListApiKeysRequestCreatedBefore>` — Only return API keys created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<ListApiKeysRequestCreatedAfter>` — Only return API keys created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of API keys to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns API keys after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of API keys to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns API keys before this position.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListApiKeysRequestOrder>` — The field to sort API keys by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListApiKeysRequestDirection>` — Sort direction.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">create</a>(request: CreateApiKeysRequest) -> Result&lt;ApiKey, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates an API key for an account or app. The response is the only place the full `secret_key` is returned — store it immediately. Requires a user session; API keys cannot manage API keys.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .api_keys
        .create(
            &CreateAPIKeysRequest {
                name: "Shine Time Booking (production)".to_string(),
                permissions: CreateAPIKeysRequestPermissions {
                    ..Default::default()
                },
                resource_id: "biz_xxxxxxxxxxxxxx".to_string(),
                resource_type: CreateAPIKeysRequestResourceType::Account,
                api_version_date: None,
                expires_at: None,
                ip_allowlist: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_version_date:** `Option<CreateApiKeysRequestApiVersionDate>` — Dated API version used when requests authenticated with this key omit the `Api-Version-Date` header. New keys default to the latest version.
    
</dd>
</dl>

<dl>
<dd>

**expires_at:** `Option<Option<String>>` — When the API key should stop working, as an ISO 8601 timestamp. Omit (or pass `null` on update) for a key that never expires.
    
</dd>
</dl>

<dl>
<dd>

**ip_allowlist:** `Option<Option<Vec<String>>>` — IPv4/IPv6 CIDR ranges allowed to use this key, for example `["203.0.113.0/24"]`. Empty or `null` allows any IP.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — A human-readable name for the API key, such as 'Production API Key'.
    
</dd>
</dl>

<dl>
<dd>

**permissions:** `CreateApiKeysRequestPermissions` — The permissions policy for the API key: explicit permission statements, or a system role to inherit from. Statements without a `resources` array default to the owning account (Account API keys) or every key-addressable resource (App API keys).
    
</dd>
</dl>

<dl>
<dd>

**resource_id:** `String` — The account (`biz_`) or app (`app_`) tag to create the API key for.
    
</dd>
</dl>

<dl>
<dd>

**resource_type:** `CreateApiKeysRequestResourceType` — The type of resource that will own this API key.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">list_permissions</a>() -> Result&lt;ListPermissionsApiKeysResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the catalog of permission actions that can be granted to users, apps, and API keys — the source for the dashboard's permission pickers. Small and returned in full on one page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.api_keys.list_permissions(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">retrieve</a>(id: String) -> Result&lt;ApiKey, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves an API key with its effective permission grants. The full secret is never returned — rotate the key if it was lost.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.api_keys.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — API key ID, prefixed `apik_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">delete</a>(id: String) -> Result&lt;DeleteApiKeysResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently revokes an API key; requests using its secret stop authenticating immediately. Default and agent-backend keys cannot be deleted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.api_keys.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — API key ID, prefixed `apik_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">update</a>(id: String, request: UpdateApiKeysRequest) -> Result&lt;ApiKey, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an API key's name, permissions, API version, expiration, or IP allowlist. Fields that are omitted keep their current value; default keys cannot be modified.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .api_keys
        .update(
            &"id".to_string(),
            &UpdateAPIKeysRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — API key ID, prefixed `apik_`.
    
</dd>
</dl>

<dl>
<dd>

**api_version_date:** `Option<UpdateApiKeysRequestApiVersionDate>` — Dated API version used when requests authenticated with this key omit the `Api-Version-Date` header. New keys default to the latest version.
    
</dd>
</dl>

<dl>
<dd>

**expires_at:** `Option<Option<String>>` — When the API key should stop working, as an ISO 8601 timestamp. Omit (or pass `null` on update) for a key that never expires.
    
</dd>
</dl>

<dl>
<dd>

**ip_allowlist:** `Option<Option<Vec<String>>>` — IPv4/IPv6 CIDR ranges allowed to use this key, for example `["203.0.113.0/24"]`. Empty or `null` allows any IP.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>` — A new human-readable name for the API key.
    
</dd>
</dl>

<dl>
<dd>

**permissions:** `Option<UpdateApiKeysRequestPermissions>` — The permissions policy for the API key: explicit permission statements, or a system role to inherit from. Statements without a `resources` array default to the owning account (Account API keys) or every key-addressable resource (App API keys).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">rotate</a>(id: String) -> Result&lt;ApiKey, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Rotates the API key's secret, invalidating the previous secret immediately. The response is the only place the new `secret_key` is returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.api_keys.rotate(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — API key ID, prefixed `apik_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## App Builds
<details><summary><code>client.app_builds.<a href="/src/api/resources/app_builds/client.rs">list</a>(app_id: Option&lt;String&gt;, platform: Option&lt;Option&lt;ListAppBuildsRequestPlatform&gt;&gt;, status: Option&lt;Option&lt;ListAppBuildsRequestStatus&gt;&gt;, created_before: Option&lt;Option&lt;ListAppBuildsRequestCreatedBefore&gt;&gt;, created_after: Option&lt;Option&lt;ListAppBuildsRequestCreatedAfter&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListAppBuildsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of build artifacts for an app, newest first, with optional platform, status, and creation-date filters.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .app_builds
        .list(
            &AppBuildsListQueryRequest {
                app_id: "app_id".to_string(),
                platform: None,
                status: None,
                created_before: None,
                created_after: None,
                first: None,
                after: None,
                last: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**app_id:** `String` — The app to list builds for, prefixed `app_`.
    
</dd>
</dl>

<dl>
<dd>

**platform:** `Option<ListAppBuildsRequestPlatform>` — Filter builds by target platform.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListAppBuildsRequestStatus>` — Filter builds by review status.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<ListAppBuildsRequestCreatedBefore>` — Only return builds created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<ListAppBuildsRequestCreatedAfter>` — Only return builds created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of builds to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns builds after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of builds to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns builds before this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.app_builds.<a href="/src/api/resources/app_builds/client.rs">create</a>(request: CreateAppBuildsRequest) -> Result&lt;AppBuild, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Uploads a new build artifact for an app. Upload the file first (POST /files or a direct upload), then reference it here; iOS and Android take a .zip bundle, web takes a JavaScript file or a .zip archive of the hosted site.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .app_builds
        .create(
            &CreateAppBuildsRequest {
                attachment: CreateAppBuildsRequestAttachment {
                    ..Default::default()
                },
                checksum: "xxxxxxxxxxxxxxx".to_string(),
                platform: CreateAppBuildsRequestPlatform::Ios,
                ai_prompt_id: None,
                app_id: None,
                source_attachment: None,
                supported_app_view_types: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ai_prompt_id:** `Option<String>` — The AI prompt that generated this build, if applicable.
    
</dd>
</dl>

<dl>
<dd>

**app_id:** `Option<String>` — The app to create the build for, prefixed `app_`. Defaults to the app behind the presented credential.
    
</dd>
</dl>

<dl>
<dd>

**attachment:** `CreateAppBuildsRequestAttachment` — The uploaded build file: `{ id }` for an existing file or `{ direct_upload_id }` for a completed direct upload.
    
</dd>
</dl>

<dl>
<dd>

**checksum:** `String` — A client-generated checksum of the build file, used to verify file integrity when unpacked.
    
</dd>
</dl>

<dl>
<dd>

**platform:** `CreateAppBuildsRequestPlatform` — The target platform for the build.
    
</dd>
</dl>

<dl>
<dd>

**source_attachment:** `Option<CreateAppBuildsRequestSourceAttachment>` — An optional compressed archive (.zip or .gz) of the source code that produced this build, stored alongside the build so it can be downloaded later. Referenced like `attachment`, and must be a different file.
    
</dd>
</dl>

<dl>
<dd>

**supported_app_view_types:** `Option<Vec<CreateAppBuildsRequestSupportedAppViewTypesItem>>` — The view types this build supports. Only list the ones its code implements.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.app_builds.<a href="/src/api/resources/app_builds/client.rs">retrieve</a>(id: String) -> Result&lt;AppBuild, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing app build.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.app_builds.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — App build ID, prefixed `abld_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.app_builds.<a href="/src/api/resources/app_builds/client.rs">promote</a>(id: String) -> Result&lt;AppBuild, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Promotes a draft or approved app build to production so it becomes the active version served to users. Draft builds enter review first.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.app_builds.promote(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — App build ID, prefixed `abld_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Apps
<details><summary><code>client.apps.<a href="/src/api/resources/apps/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, app_type: Option&lt;Option&lt;ListAppsRequestAppType&gt;&gt;, view_type: Option&lt;Option&lt;ListAppsRequestViewType&gt;&gt;, verified: Option&lt;Option&lt;bool&gt;&gt;, verified_apps_only: Option&lt;Option&lt;bool&gt;&gt;, recommended: Option&lt;Option&lt;bool&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListAppsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListAppsRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListAppsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists apps on the Whop platform: the app store's live apps, or — with `account_id` and developer access to that account — every app the account owns. Requires authentication except for Whop's public app and website discovery lists. Public website discovery includes built official blueprints (verified apps with a product) and built, live community blueprints that Whop recommends.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .apps
        .list(
            &AppsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Only return apps created by this account (`biz_` tag). With developer access to the account this includes its unlisted and hidden apps.
    
</dd>
</dl>

<dl>
<dd>

**app_type:** `Option<ListAppsRequestAppType>` — Filter apps by the type of end-user they are built for. Apps of type `website` are left out unless you ask for them by name.
    
</dd>
</dl>

<dl>
<dd>

**view_type:** `Option<ListAppsRequestViewType>` — Only return apps supporting this view type, such as `dashboard` or `hub`.
    
</dd>
</dl>

<dl>
<dd>

**verified:** `Option<bool>` — Only return apps whose Whop verification status matches this value. Omit this filter to include every verification status the caller can see.
    
</dd>
</dl>

<dl>
<dd>

**verified_apps_only:** `Option<bool>` — Legacy compatibility filter. Use `verified` for field equality. `true` returns verified apps; clients pinned before `2026-08-25-2` retain the earlier public website discovery behavior.
    
</dd>
</dl>

<dl>
<dd>

**recommended:** `Option<bool>` — Only return apps Whop recommends (or, with `false`, only those it does not), independently of verification status.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — A search string matched against app names.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListAppsRequestOrder>` — The field to sort apps by. Defaults to discoverable_at, showing the most recently published apps first. `template_usage` ranks Whop-verified apps first, then apps with a banner image, then by how many apps were created from each app as a template.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListAppsRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of apps to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns apps after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of apps to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns apps before this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.apps.<a href="/src/api/resources/apps/client.rs">create</a>(request: CreateAppsRequest) -> Result&lt;App, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Registers a new app on the Whop developer platform. Apps provide custom experiences that can be added to products.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .apps
        .create(
            &CreateAppsRequest {
                name: "Shine Time Booking".to_string(),
                account_id: None,
                app_type: None,
                base_url: None,
                icon: None,
                redirect_uris: None,
                route: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The account to create the app for (`biz_` tag). Defaults to the account behind the presented credential.
    
</dd>
</dl>

<dl>
<dd>

**app_type:** `Option<CreateAppsRequestAppType>` — The type of app to create. Defaults to `b2c_app`.
    
</dd>
</dl>

<dl>
<dd>

**base_url:** `Option<Option<String>>` — The base production URL where the app is hosted, such as `https://myapp.example.com`.
    
</dd>
</dl>

<dl>
<dd>

**icon:** `Option<CreateAppsRequestIcon>` — The icon image for the app in PNG, JPEG, or GIF format, referencing an uploaded file: `{ id }` for an existing attachment or `{ direct_upload_id }` for a new direct upload.
    
</dd>
</dl>

<dl>
<dd>

**name:** `String` — The display name for the app, shown to users on the app store and product pages.
    
</dd>
</dl>

<dl>
<dd>

**redirect_uris:** `Option<Vec<String>>` — The whitelisted OAuth callback URLs that users are redirected to after authorizing the app.
    
</dd>
</dl>

<dl>
<dd>

**route:** `Option<Option<String>>` — The subdomain route where the app's hosted web builds are served, such as `myapp` for myapp.whop.site.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.apps.<a href="/src/api/resources/apps/client.rs">update_permissions_app</a>(app_id: String, request: UpdatePermissionsAppRequest) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates the permission requirements for an app

Required permissions:
 - `developer:update_app_authorization`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .apps
        .update_permissions_app(
            &"app_id".to_string(),
            &UpdatePermissionsAppRequest {
                requested_permissions: vec![UpdatePermissionsAppRequestRequestedPermissionsItem {
                    action: "action".to_string(),
                    is_required: true,
                    justification: "justification".to_string(),
                    ..Default::default()
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**app_id:** `String` — The ID of the app the permission requirements are being updated for
    
</dd>
</dl>

<dl>
<dd>

**requested_permissions:** `Vec<UpdatePermissionsAppRequestRequestedPermissionsItem>` — The permissions that the app will request off of users when a user installs the app.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.apps.<a href="/src/api/resources/apps/client.rs">retrieve</a>(id: String) -> Result&lt;App, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves an app by ID, claimed route, or proxy domain id. Credential fields (api_key, default_api_key, secrets) render `null` unless the caller has the corresponding developer permission on the owning account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.apps.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — App ID (prefixed `app_`), the app's claimed route, or its proxy domain id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.apps.<a href="/src/api/resources/apps/client.rs">delete</a>(id: String) -> Result&lt;DeleteAppsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an app. The app stops resolving within seconds — a website's site stops serving, and any claimed subdomain is reserved for a month before it can be claimed again.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.apps.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — App ID (prefixed `app_`), the app's claimed route, or its proxy domain id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.apps.<a href="/src/api/resources/apps/client.rs">update</a>(id: String, request: UpdateAppsRequest) -> Result&lt;App, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates the settings, metadata, or status of an app. Fields that are omitted keep their current value.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .apps
        .update(
            &"id".to_string(),
            &UpdateAppsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — App ID (prefixed `app_`), the app's claimed route, or its proxy domain id.
    
</dd>
</dl>

<dl>
<dd>

**app_store_description:** `Option<String>` — The detailed description shown on the app store's in-depth app view page.
    
</dd>
</dl>

<dl>
<dd>

**app_type:** `Option<UpdateAppsRequestAppType>` — The type of end-user the app is built for. Cannot be changed on an app whose type is already `website`.
    
</dd>
</dl>

<dl>
<dd>

**base_url:** `Option<Option<String>>` — The base production URL where the app is hosted. Set to `null` to take the app proxy offline.
    
</dd>
</dl>

<dl>
<dd>

**dashboard_path:** `Option<Option<String>>` — The URL path for the account dashboard view.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — A short description of the app shown in listings and search results.
    
</dd>
</dl>

<dl>
<dd>

**discover_path:** `Option<Option<String>>` — The URL path for the discover view.
    
</dd>
</dl>

<dl>
<dd>

**experience_path:** `Option<Option<String>>` — The URL path for the member-facing hub view, such as `/experiences/[experienceId]`.
    
</dd>
</dl>

<dl>
<dd>

**icon:** `Option<UpdateAppsRequestIcon>` — The icon image for the app in PNG, JPEG, or GIF format, referencing an uploaded file: `{ id }` for an existing attachment or `{ direct_upload_id }` for a new direct upload.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — The display name for the app, shown to users on the app store and product pages.
    
</dd>
</dl>

<dl>
<dd>

**oauth_client_type:** `Option<UpdateAppsRequestOauthClientType>` — How the app authenticates at the OAuth token endpoint.
    
</dd>
</dl>

<dl>
<dd>

**openapi_path:** `Option<Option<String>>` — The URL path to the app's OpenAPI spec file (requires the ai_chat capability).
    
</dd>
</dl>

<dl>
<dd>

**production_android_build_id:** `Option<Option<String>>` — The app build (`abld_` tag) to serve as the Android production build, or `null` to unassign it. Same rules as `production_web_build_id`.
    
</dd>
</dl>

<dl>
<dd>

**production_ios_build_id:** `Option<Option<String>>` — The app build (`abld_` tag) to serve as the iOS production build, or `null` to unassign it. Same rules as `production_web_build_id`.
    
</dd>
</dl>

<dl>
<dd>

**production_web_build_id:** `Option<Option<String>>` — The app build (`abld_` tag) to serve as the web production build, or `null` to unassign it. The build must belong to this app, target web, and be in the draft or approved status; a draft build is queued for approval and takes over once approved. Requires the `developer:manage_builds` scope.
    
</dd>
</dl>

<dl>
<dd>

**redirect_uris:** `Option<Vec<String>>` — The whitelisted OAuth callback URLs users are redirected to after authorizing the app.
    
</dd>
</dl>

<dl>
<dd>

**required_scopes:** `Option<Vec<String>>` — The OAuth scopes the app requests from users when they install it.
    
</dd>
</dl>

<dl>
<dd>

**route:** `Option<String>` — The subdomain route where the app's hosted web builds are served.
    
</dd>
</dl>

<dl>
<dd>

**secrets:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Secrets to add or overwrite on the app, as an object of string values. Keys not included are left untouched; pass null or an empty string as the value to delete a secret. Encrypted at rest and injected into the app's hosted server runtime.
    
</dd>
</dl>

<dl>
<dd>

**skills_path:** `Option<Option<String>>` — The URL path to the app's skills directory (requires the ai_chat capability).
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<UpdateAppsRequestStatus>` — Controls whether the app is published on Whop discovery or accessible only through its direct link. Publishing requires a name, icon, and description.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.apps.<a href="/src/api/resources/apps/client.rs">deploy</a>(id: String, request: DeployAppsRequest) -> Result&lt;AppDeployment, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Builds the app's current source and ships it. Returns the run it started, so the caller can render progress from this response and then follow it on the app's `deployment` field. Only one deployment runs per app at a time — calling this while one is in flight reports that run rather than starting a second, and calling it with nothing to publish reports that instead of starting one.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .apps
        .deploy(
            &"id".to_string(),
            &DeployAppsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The app to deploy, prefixed `app_`.
    
</dd>
</dl>

<dl>
<dd>

**draft:** `Option<bool>` — Upload the build without making it live. Defaults to `false`, which deploys and promotes in one step.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.apps.<a href="/src/api/resources/apps/client.rs">logs</a>(id: String, app_build_id: Option&lt;Option&lt;String&gt;&gt;, level: Option&lt;Option&lt;LogsAppsRequestLevel&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;LogsAppsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists a hosted app's server runtime logs, most recent first: console output, uncaught exceptions, and failed-request summaries captured on whop.site hosting. Logs are retained for 7 days.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .apps
        .logs(
            &"id".to_string(),
            &LogsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the app, which will look like app_*************.
    
</dd>
</dl>

<dl>
<dd>

**app_build_id:** `Option<String>` — Only return logs from this build.
    
</dd>
</dl>

<dl>
<dd>

**level:** `Option<LogsAppsRequestLevel>` — Only return console lines of this level.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Only return logs whose message contains this text (case-insensitive).
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Start of the time window as an ISO 8601 timestamp. Defaults to 7 days before created_before.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — End of the time window as an ISO 8601 timestamp. Defaults to now.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of log lines to return (max 500).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor for fetching logs after a previous page.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor for fetching logs before a later page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.apps.<a href="/src/api/resources/apps/client.rs">update_permissions</a>(id: String, request: UpdatePermissionsAppsRequest) -> Result&lt;App, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replaces the set of permissions the app requests from users when they install it. Requires a user session: the `developer:update_app_authorization` scope cannot be delegated to API keys. Sensitive permissions require step-up verification.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .apps
        .update_permissions(
            &"id".to_string(),
            &UpdatePermissionsAppsRequest {
                requested_permissions: vec![UpdatePermissionsAppsRequestRequestedPermissionsItem {
                    action: "company:basic:read".to_string(),
                    is_required: true,
                    justification: "Reads basic account info to render the dashboard home."
                        .to_string(),
                    ..Default::default()
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — App ID, prefixed `app_`.
    
</dd>
</dl>

<dl>
<dd>

**requested_permissions:** `Vec<UpdatePermissionsAppsRequestRequestedPermissionsItem>` — The full set of permissions the app requests on install; permissions not listed are removed.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Audiences
<details><summary><code>client.audiences.<a href="/src/api/resources/audiences/client.rs">list</a>(account_id: Option&lt;String&gt;, audience_id: Option&lt;Option&lt;String&gt;&gt;, audience_type: Option&lt;Option&lt;ListAudiencesRequestAudienceType&gt;&gt;, source_type: Option&lt;Option&lt;ListAudiencesRequestSourceType&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListAudiencesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists uploaded customer-list audiences for an account. Pass `audience_id` to return a specific audience.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .audiences
        .list(
            &AudiencesListQueryRequest {
                account_id: "account_id".to_string(),
                audience_id: None,
                audience_type: None,
                source_type: None,
                first: None,
                after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Account ID, prefixed `biz_`.
    
</dd>
</dl>

<dl>
<dd>

**audience_id:** `Option<String>` — Audience ID, prefixed `adaud_`, used to filter the response to one audience.
    
</dd>
</dl>

<dl>
<dd>

**audience_type:** `Option<ListAudiencesRequestAudienceType>` — Filter by audience type: `custom` (uploaded lists) or `lookalike`.
    
</dd>
</dl>

<dl>
<dd>

**source_type:** `Option<ListAudiencesRequestSourceType>` — Filter by member source: `csv_upload` (uploaded lists) or `people_filter` (automatic audiences built from saved People filters).
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of audiences to return. Defaults to 20; maximum 100.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor for the next page of audiences.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.audiences.<a href="/src/api/resources/audiences/client.rs">create</a>(request: CreateAudiencesRequest) -> Result&lt;CreateAudiencesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates an audience. Default (`audience_type` omitted or `custom`): creates one audience from an uploaded customer identity CSV file (`name`, `column_mapping`, and `file_id` required) and starts processing it; responds with the audience object. With `filters`: creates an audience from saved People filters (`name` required) — membership is built from the account's People data, and `auto_refresh` decides whether it keeps tracking the filters or keeps whoever matched at creation. With `audience_type: lookalike`: creates a ladder of Meta lookalike audiences from an existing ready custom audience (`source_audience_id`, `count`, and `percentage` required) — `count` equal similarity bands slicing the top `percentage`% (3 audiences at 6% = 0–2%, 2–4%, 4–6%), each returned as its own audience in a `{ data: [...] }` envelope.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .audiences
        .create(
            &CreateAudiencesRequest {
                account_id: "biz_xxxxxxxxxxxxxx".to_string(),
                audience_type: None,
                auto_refresh: None,
                column_mapping: None,
                count: None,
                file_id: None,
                filters: None,
                name: None,
                percentage: None,
                source_audience_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Account ID, prefixed `biz_`.
    
</dd>
</dl>

<dl>
<dd>

**audience_type:** `Option<CreateAudiencesRequestAudienceType>` — What to create. Defaults to `custom` (CSV upload).
    
</dd>
</dl>

<dl>
<dd>

**auto_refresh:** `Option<bool>` — Filter audiences only, and set only at creation. `true` (the default) rebuilds membership from the filters twice a day. `false` keeps whoever matched at creation and never rebuilds.
    
</dd>
</dl>

<dl>
<dd>

**column_mapping:** `Option<CreateAudiencesRequestColumnMapping>` — Custom audiences only. Maps supported identity fields to CSV column headers. Map at least one of `email` or `phone`.
    
</dd>
</dl>

<dl>
<dd>

**count:** `Option<i64>` — Lookalikes only. Number of lookalike audiences to create (1–6).
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `Option<String>` — Custom audiences only. The uploaded customer CSV — a file id (`file_...`) returned by `POST /files`.
    
</dd>
</dl>

<dl>
<dd>

**filters:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Filter audiences only. The People filters that define membership, keyed exactly as `GET /people` accepts them — for example `{"os": "iOS", "country": "US"}`. Date filters must be rolling windows — `first_seen_within_days` or `last_seen_within_days` — so the audience re-anchors on every refresh; fixed dates such as `first_seen_after` are rejected. Source values are canonical source paths (`whop:<campaign>:<group>:<ad>`, `ext:<platform>:...`, `referrer:<domain>`, `direct`), exact or with a trailing `:*` wildcard.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — Audience display name. Required for custom audiences; lookalike names are generated from the source audience.
    
</dd>
</dl>

<dl>
<dd>

**percentage:** `Option<i64>` — Lookalikes only. Total similarity reach as a whole percent (1–20), sliced evenly across `count` — must be divisible by `count`.
    
</dd>
</dl>

<dl>
<dd>

**source_audience_id:** `Option<String>` — Lookalikes only. The ready custom audience (`adaud_`) to build from; it needs at least 100 matched people.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.audiences.<a href="/src/api/resources/audiences/client.rs">delete</a>(id: String) -> Result&lt;DeleteAudiencesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an audience so it is no longer available for targeting.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.audiences.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Audience ID, prefixed `adaud_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.audiences.<a href="/src/api/resources/audiences/client.rs">update</a>(id: String, request: UpdateAudiencesRequest) -> Result&lt;Audience, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Renames an audience. For an audience built from People filters that keeps itself up to date, pass `filters` to replace them, which rebuilds membership immediately. Whether an audience auto refreshes is set when it is created.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .audiences
        .update(
            &"id".to_string(),
            &UpdateAudiencesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Audience ID, prefixed `adaud_`.
    
</dd>
</dl>

<dl>
<dd>

**filters:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Replaces the People filters that define membership. The whole definition is replaced rather than merged, so send every filter you want to keep — a filter you leave out stops applying. Keys and values are the ones `GET /people` accepts, such as an `os` of `iOS` or a `country` of `US`, and at least one filter is required. Date filters must be rolling windows — `first_seen_within_days` or `last_seen_within_days` — so the audience re-anchors every time it rebuilds; fixed dates such as `first_seen_after` are rejected, as is `audience_id`. An array value holds at most 500 items, and each value at most 10 KB. Only an audience with a `source_type` of `people_filter` and `auto_refresh` of `true` accepts filters: an uploaded list has no filters to replace, and with auto refresh off the audience keeps the people it matched when it was built, so create a new audience instead.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — New audience display name. A blank value is ignored rather than clearing the name.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.audiences.<a href="/src/api/resources/audiences/client.rs">add_people</a>(id: String, request: AddPeopleAudiencesRequest) -> Result&lt;Audience, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Adds users from a new CSV file to an existing uploaded custom audience. The file uses the audience's saved column mapping, processing happens in the background, and existing audience members remain unchanged.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .audiences
        .add_people(
            &"id".to_string(),
            &AddPeopleAudiencesRequest {
                file_id: "file_xxxxxxxxxxxxxx".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**file_id:** `String` — The new customer CSV — a file id (`file_...`) returned by `POST /files`. Its headers must match the audience's saved column mapping.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AuthorizedUsers
<details><summary><code>client.authorized_users.<a href="/src/api/resources/authorized_users/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, role: Option&lt;Option&lt;AuthorizedUserRoles&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListAuthorizedUsersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of authorized team members for a company, with optional filtering by user, role, and creation date.

Required permissions:
 - `company:authorized_user:read`
 - `member:email:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .authorized_users
        .list(
            &AuthorizedUsersListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                user_id: Some("user_xxxxxxxxxxxxx".to_string()),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of the company to list authorized users for.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Filter results to a specific user to check if they are an authorized team member.
    
</dd>
</dl>

<dl>
<dd>

**role:** `Option<AuthorizedUserRoles>` 
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return authorized users created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return authorized users created after this timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.authorized_users.<a href="/src/api/resources/authorized_users/client.rs">create</a>(request: CreateAuthorizedUsersRequest) -> Result&lt;AuthorizedUser, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new authorized user to a company.

Required permissions:
 - `authorized_user:create`
 - `member:email:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .authorized_users
        .create(
            &CreateAuthorizedUsersRequest {
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                role: GrantableAuthorizedUserRoles::Owner,
                user_id: "user_xxxxxxxxxxxxx".to_string(),
                elevation: None,
                send_emails: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `String` — The ID of the company to add the authorized user to.
    
</dd>
</dl>

<dl>
<dd>

**elevation:** `Option<Option<CreateAuthorizedUsersRequestElevation>>` — Re-authentication proof required to perform this sensitive action.
    
</dd>
</dl>

<dl>
<dd>

**role:** `GrantableAuthorizedUserRoles` — The role to assign to the authorized user within the company. Supported roles: 'moderator', 'sales_manager'.
    
</dd>
</dl>

<dl>
<dd>

**send_emails:** `Option<Option<bool>>` — Whether to send notification emails to the user on creation.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `String` — The ID of the user to add as an authorized user.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.authorized_users.<a href="/src/api/resources/authorized_users/client.rs">retrieve</a>(id: String) -> Result&lt;AuthorizedUser, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing authorized user.

Required permissions:
 - `company:authorized_user:read`
 - `member:email:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .authorized_users
        .retrieve(&"ausr_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the authorized user to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.authorized_users.<a href="/src/api/resources/authorized_users/client.rs">delete</a>(id: String, company_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove an authorized user from a company.

Required permissions:
 - `authorized_user:delete`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .authorized_users
        .delete(
            &"ausr_xxxxxxxxxxxxx".to_string(),
            &AuthorizedUsersDeleteQueryRequest {
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the authorized user or user to remove.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The ID of the company the authorized user belongs to. Optional if the authorized user ID is provided.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Bounties
<details><summary><code>client.bounties.<a href="/src/api/resources/bounties/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListBountiesRequestStatus&gt;&gt;, business_goal_type: Option&lt;Option&lt;ListBountiesRequestBusinessGoalType&gt;&gt;, country: Option&lt;Option&lt;String&gt;&gt;, experience_id: Option&lt;Option&lt;String&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListBountiesRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListBountiesRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListBountiesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists bounties visible to the credential — for an account API key, the account's bounties including scheduled drafts; for a user token, the bounties the user can see and work.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .bounties
        .list(
            &BountiesListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Scope the list to this account (`biz_` tag). Requires read access to the account; account API keys may pass their own account or a connected account.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — List the bounties this user participated in (`user_` tag). Must be the authenticated user.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListBountiesRequestStatus>` — Filter by lifecycle state.
    
</dd>
</dl>

<dl>
<dd>

**business_goal_type:** `Option<ListBountiesRequestBusinessGoalType>` — Filter by the poster's declared goal. Bounties created before the goal taxonomy carry no goal and never match this filter.
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — Only bounties workable from this country, as an ISO 3166-1 alpha-2 code. Bounties with no country targeting are workable worldwide and always match.
    
</dd>
</dl>

<dl>
<dd>

**experience_id:** `Option<String>` — Only bounties posted to this forum experience, prefixed `exp_`. An unknown experience, or one outside the caller's scope, matches nothing.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Substring match on the bounty title or ID.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only bounties created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only bounties created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListBountiesRequestOrder>` — Sort field.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListBountiesRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of bounties to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to paginate forwards from.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of bounties to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to paginate backwards from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bounties.<a href="/src/api/resources/bounties/client.rs">create</a>(request: CreateBountiesRequest) -> Result&lt;Bounty, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a bounty and escrows its reward pool. Publishes immediately, or as a scheduled draft when you set `publish_at`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.bounties.create(&CreateBountiesRequest {
        description: "Record one continuous pass of a full interior detail, dash to trunk, on a customer vehicle.".to_string(),
        gross_reward_amount: 40.0,
        title: "Record interior detailing passes".to_string(),
        accepted_submissions_limit: None,
        accepted_submissions_per_user_limit: None,
        account_id: None,
        allowed_country_codes: None,
        business_goal_type: None,
        capture_spec: None,
        experience_id: None,
        frequency: None,
        publish_at: None,
        publish_at_timezone: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**accepted_submissions_limit:** `Option<Option<i64>>` — Number of submissions that can be accepted (winner slots). Defaults to 1. The escrowed total is `gross_reward_amount` times this limit and must be at least $5.
    
</dd>
</dl>

<dl>
<dd>

**accepted_submissions_per_user_limit:** `Option<Option<i64>>` — How many winner slots one worker can win. Defaults to `1`. Wins plus proofs awaiting review never exceed this number, and a worker runs one attempt at a time. Cannot exceed `accepted_submissions_limit`.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<Option<String>>` — Account whose balance funds the bounty pool (`biz_` tag). Defaults to the caller's personal balance. Requires permission to move the account's funds.
    
</dd>
</dl>

<dl>
<dd>

**allowed_country_codes:** `Option<Option<Vec<String>>>` — Countries whose residents can work the bounty, as ISO 3166 alpha-2 codes. Empty means worldwide.
    
</dd>
</dl>

<dl>
<dd>

**business_goal_type:** `Option<CreateBountiesRequestBusinessGoalType>` — What the poster wants the work to achieve, declared once here.
    
</dd>
</dl>

<dl>
<dd>

**capture_spec:** `Option<CreateBountiesRequestCaptureSpec>` — Per-bounty overrides of the served capture contract. Only accepted when `business_goal_type` is `data_capture`; omitted fields keep the platform defaults, and the resulting contract is echoed back as `capture_spec` on the bounty.
    
</dd>
</dl>

<dl>
<dd>

**description:** `String` — Full task instructions shown to workers.
    
</dd>
</dl>

<dl>
<dd>

**experience_id:** `Option<Option<String>>` — Experience to host the bounty in (`exp_` tag). Any visibility — public for an open bounty, private for an invited one. Required unless account_id is set, in which case the bounty anchors in that account's public forum.
    
</dd>
</dl>

<dl>
<dd>

**frequency:** `Option<CreateBountiesRequestFrequency>` — How often the schedule creates a new bounty. Each occurrence is a separate bounty. Defaults to `once`; only applies with `publish_at`.
    
</dd>
</dl>

<dl>
<dd>

**gross_reward_amount:** `f64` — Gross bounty-pool amount (USD) escrowed per accepted submission, in whole dollars. Platform fees and affiliate shares are paid from this amount.
    
</dd>
</dl>

<dl>
<dd>

**publish_at:** `Option<Option<String>>` — ISO 8601 time to publish the bounty. When set, the bounty is created as a hidden draft and funded + published at this time instead of immediately.
    
</dd>
</dl>

<dl>
<dd>

**publish_at_timezone:** `Option<Option<String>>` — IANA timezone for recurring occurrences. Required when publish_at is set.
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` — Short name of the task shown to workers.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bounties.<a href="/src/api/resources/bounties/client.rs">retrieve</a>(id: String) -> Result&lt;Bounty, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a bounty by ID. Authentication is optional: a request with no credential reads the bounty when it is publicly visible — published or completed, and not restricted to a private experience's members. Bounties outside the caller's scope, and bounties not publicly visible to an anonymous caller, return `404`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.bounties.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Bounty ID (`bnty_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bounties.<a href="/src/api/resources/bounties/client.rs">update</a>(id: String, request: UpdateBountiesRequest) -> Result&lt;Bounty, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a bounty. A published bounty accepts title, description, and country targeting while it is still open with nothing under review. A scheduled (not-yet-published) draft additionally accepts the reward, winner slots, and schedule.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .bounties
        .update(
            &"id".to_string(),
            &UpdateBountiesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Bounty ID (`bnty_` tag).
    
</dd>
</dl>

<dl>
<dd>

**accepted_submissions_limit:** `Option<Option<i64>>` — Scheduled drafts only. Number of submissions that can be accepted (winner slots).
    
</dd>
</dl>

<dl>
<dd>

**accepted_submissions_per_user_limit:** `Option<Option<i64>>` — How many winner slots one worker can win. Defaults to `1`. Wins plus proofs awaiting review never exceed this number, and a worker runs one attempt at a time. Cannot exceed `accepted_submissions_limit`. Editable while the bounty is still open with nothing under review.
    
</dd>
</dl>

<dl>
<dd>

**allowed_country_codes:** `Option<Option<Vec<String>>>` — Replace the countries whose residents can work the bounty, as ISO 3166 alpha-2 codes. Empty means worldwide.
    
</dd>
</dl>

<dl>
<dd>

**business_goal_type:** `Option<UpdateBountiesRequestBusinessGoalType>` — What the poster wants the work to achieve, declared once here.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — New full task instructions.
    
</dd>
</dl>

<dl>
<dd>

**frequency:** `Option<UpdateBountiesRequestFrequency>` — Scheduled drafts only. How often the schedule creates a new bounty.
    
</dd>
</dl>

<dl>
<dd>

**gross_reward_amount:** `Option<Option<f64>>` — Scheduled drafts only. Gross bounty-pool amount (USD) escrowed per accepted submission. The escrowed total (this times accepted_submissions_limit) must stay at least $5.
    
</dd>
</dl>

<dl>
<dd>

**publish_at:** `Option<Option<String>>` — Scheduled drafts only. New ISO 8601 time to publish the draft. Must be in the future.
    
</dd>
</dl>

<dl>
<dd>

**publish_at_timezone:** `Option<Option<String>>` — Scheduled drafts only. IANA timezone for recurring occurrences.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — New short name of the task.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bounties.<a href="/src/api/resources/bounties/client.rs">cancel</a>(id: String) -> Result&lt;Bounty, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancels a bounty. With no in-flight work, it cancels immediately and refunds the funder. Otherwise it stops new submissions and cancels once the in-flight work resolves and pays out. Repeating the request is a no-op. A bounty that already paid out every slot returns `400`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.bounties.cancel(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Bounty ID (`bnty_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Bounty Submissions
<details><summary><code>client.bounty_submissions.<a href="/src/api/resources/bounty_submissions/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, bounty_id: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListBountySubmissionsRequestStatus&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListBountySubmissionsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListBountySubmissionsRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListBountySubmissionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists bounty submissions visible to the credential — for a user token, the submissions they authored plus those on bounties they posted; for an account API key, the submissions on the account's bounties. For the anonymous view of one bounty's reviewed work, use the submissions list under the bounty instead.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .bounty_submissions
        .list(
            &BountySubmissionsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Scope the list to submissions on this account's bounties (`biz_` tag). Requires read access to the account.
    
</dd>
</dl>

<dl>
<dd>

**bounty_id:** `Option<String>` — Only submissions on this bounty (`bnty_` tag).
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListBountySubmissionsRequestStatus>` — Filter by lifecycle state.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only submissions created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only submissions created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListBountySubmissionsRequestOrder>` — Sort field.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListBountySubmissionsRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of submissions to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to paginate forwards from.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of submissions to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to paginate backwards from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bounty_submissions.<a href="/src/api/resources/bounty_submissions/client.rs">create</a>(request: CreateBountySubmissionsRequest) -> Result&lt;BountySubmission, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a submission on a workforce bounty. Include a `deliverable` payload — any combination of links and uploaded files, with at least one of the two — and the submission goes straight to review; create is the only step. For `data_capture` bounties, omit the deliverable: this starts a claimed attempt whose proof accumulates server-side, and the separate submit endpoint sends it to review once complete. Requires a user credential — account API keys cannot author submissions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .bounty_submissions
        .create(
            &CreateBountySubmissionsRequest {
                bounty_id: "bnty_xxxxxxxxxxxxxx".to_string(),
                affiliate_code: None,
                deliverable: None,
                metadata: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**affiliate_code:** `Option<Option<String>>` — Affiliate code crediting the referrer, when the worker arrived through one.
    
</dd>
</dl>

<dl>
<dd>

**bounty_id:** `String` — The bounty to submit to (`bnty_` tag).
    
</dd>
</dl>

<dl>
<dd>

**deliverable:** `Option<Option<CreateBountySubmissionsRequestDeliverable>>` — The submitted work. Combine `urls`, `file_ids`, and `caption` freely; at least one link or file is required.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<CreateBountySubmissionsRequestMetadata>>` — Optional capture metadata describing where and how the footage was recorded. Persisted on the submission. On a `data_capture` bounty every field except `fov` is required whenever metadata is provided.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bounty_submissions.<a href="/src/api/resources/bounty_submissions/client.rs">retrieve</a>(id: String, account_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;BountySubmission, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves one bounty submission the credential can see — one the caller authored, or one on a bounty they posted or their account owns. Reading another member's work on an account's bounty takes `account_id`, the same way the list does.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .bounty_submissions
        .retrieve(
            &"id".to_string(),
            &BountySubmissionsRetrieveQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The bounty submission to act on (`btys_` tag).
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — Read the submission as this account (`biz_` tag), scoping the lookup to its bounties rather than the caller's own work. Requires read access to the account. Without it the lookup covers only what the credential owns — the submissions the caller authored plus those on bounties they posted.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bounty_submissions.<a href="/src/api/resources/bounty_submissions/client.rs">delete</a>(id: String) -> Result&lt;DeleteBountySubmissionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancels the caller's own active attempt on a bounty and discards any accumulated capture clips. Only the worker who started the attempt can cancel it — account API keys cannot.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .bounty_submissions
        .delete(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The bounty submission to act on (`btys_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bounty_submissions.<a href="/src/api/resources/bounty_submissions/client.rs">submit</a>(id: String, request: SubmitBountySubmissionsRequest) -> Result&lt;BountySubmission, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Submits a claimed attempt for review. A livestream attempt needs an ended proof stream and can attach an optional `deliverable` — links, files, and a caption in any combination; if the attempt already went to review when its stream ended, the payload attaches to it once, until reviewers start voting. A data capture attempt instead needs enough validated clip time and takes no payload. Only the worker who started the attempt can submit it — account API keys cannot.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .bounty_submissions
        .submit(
            &"id".to_string(),
            &SubmitBountySubmissionsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The claimed attempt to submit for review (`btys_` tag).
    
</dd>
</dl>

<dl>
<dd>

**deliverable:** `Option<Option<SubmitBountySubmissionsRequestDeliverable>>` — Work to attach to the submission. Combine `urls`, `file_ids`, and `caption` freely; all are optional.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CardTransactions
<details><summary><code>client.card_transactions.<a href="/src/api/resources/card_transactions/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListCardTransactionsRequestStatus&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListCardTransactionsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListCardTransactionsRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListCardTransactionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists an account's card transactions, newest first. Defaults to the account the credential belongs to. Covers every card the owner has ever had, including canceled cards and spend that predates a re-application, and team members only see transactions on the cards assigned to them. Pass `transaction_ids` to fetch specific transactions instead of paging for them.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .card_transactions
        .list(
            &CardTransactionsListQueryRequest {
                transaction_ids: vec![Some("citx_xxxxxxxxxxxxxx".to_string())],
                card_id: vec![Some("icrd_xxxxxxxxxxxxxx".to_string())],
                cardholder_id: vec![Some("user_xxxxxxxxxxxxxx".to_string())],
                account_id: None,
                status: None,
                created_after: None,
                created_before: None,
                order: None,
                direction: None,
                first: None,
                after: None,
                last: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The account whose card transactions to list, prefixed `biz_`. Defaults to the credential's account.
    
</dd>
</dl>

<dl>
<dd>

**transaction_ids:** `Option<String>` — Return only these card transactions, each prefixed `citx_`. Repeat the parameter, or pass one comma-separated value.
    
</dd>
</dl>

<dl>
<dd>

**card_id:** `Option<String>` — Return only transactions charged to these cards, each prefixed `icrd_`.
    
</dd>
</dl>

<dl>
<dd>

**cardholder_id:** `Option<String>` — Return only transactions on cards assigned to these users, each prefixed `user_`.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListCardTransactionsRequestStatus>` — Return only transactions with this status.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Return only transactions authorized at or after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Return only transactions authorized at or before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListCardTransactionsRequestOrder>` — The field to sort by. Defaults to `created_at`.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListCardTransactionsRequestDirection>` — The sort direction. Defaults to `desc`.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of card transactions to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns card transactions after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of card transactions to return, counting back from the end.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns card transactions before this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.card_transactions.<a href="/src/api/resources/card_transactions/client.rs">retrieve</a>(id: String, account_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;CardTransaction, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetches a single card transaction by its `citx_` identifier. The owner defaults to the account the credential belongs to.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .card_transactions
        .retrieve(
            &"id".to_string(),
            &CardTransactionsRetrieveQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The card transaction ID, prefixed `citx_`.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — The account that owns the transaction, prefixed `biz_`. Defaults to the credential's account.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Cards
<details><summary><code>client.cards.<a href="/src/api/resources/cards/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListCardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the Whop cards of an account or user, including ones still being set up. Team members only see the cards assigned to them.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .cards
        .list(
            &CardsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The owning account ID (a biz_ identifier). Provide this or user_id.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The owning user ID (a user_ identifier). Provide this or account_id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cards.<a href="/src/api/resources/cards/client.rs">create</a>(request: CreateCardsRequest) -> Result&lt;CreateCardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Issue a virtual card, or apply for card issuing.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .cards
        .create(
            &CreateCardsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The owning account ID (a biz_ identifier). Provide this or user_id.
    
</dd>
</dl>

<dl>
<dd>

**assigned_user_id:** `Option<String>` — The account member (a user_ identifier) to assign the card to. Required for business card issuing accounts.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — A display name for the card.
    
</dd>
</dl>

<dl>
<dd>

**spend_limit:** `Option<f64>` — Spending limit amount, in dollars.
    
</dd>
</dl>

<dl>
<dd>

**spend_limit_frequency:** `Option<CreateCardsRequestSpendLimitFrequency>` — The window the spend limit applies to.
    
</dd>
</dl>

<dl>
<dd>

**transaction_limit:** `Option<f64>` — Per-transaction limit amount, in dollars.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The owning user ID (a user_ identifier). Provide this or account_id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cards.<a href="/src/api/resources/cards/client.rs">retrieve</a>(id: String, account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;RetrieveCardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a single card.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .cards
        .retrieve(
            &"id".to_string(),
            &CardsRetrieveQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Card ID to retrieve, prefixed `icrd_`.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — The owning account ID (a biz_ identifier). Provide this or user_id.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The owning user ID (a user_ identifier). Provide this or account_id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cards.<a href="/src/api/resources/cards/client.rs">update</a>(id: String, request: UpdateCardsRequest) -> Result&lt;UpdateCardsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update, freeze, or cancel a card. Updating the card's name, billing address, or limits requires both `payout:account:update` and `company:balance:read`; a card's assigned holder may update their own card's pin and frozen state with any user token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .cards
        .update(
            &"id".to_string(),
            &UpdateCardsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Card ID to retrieve, prefixed `icrd_`.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — The owning account ID (a biz_ identifier). Provide this or user_id.
    
</dd>
</dl>

<dl>
<dd>

**billing:** `Option<UpdateCardsRequestBilling>` — New billing address. Requires line1, city, region, postal_code, and country_code. On an invited card, passing billing alone (as the invited user) completes onboarding and starts card provisioning.
    
</dd>
</dl>

<dl>
<dd>

**canceled:** `Option<bool>` — Pass `true` to permanently cancel the card. A canceled card cannot be uncanceled. Cannot be combined with other fields.
    
</dd>
</dl>

<dl>
<dd>

**frozen:** `Option<bool>` — Pass `true` to freeze the card, `false` to unfreeze it. The assigned cardholder may freeze their own card without the payout:account:update scope.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` — A display name for the card.
    
</dd>
</dl>

<dl>
<dd>

**pin:** `Option<String>` — New 4-digit PIN. Can only be set on a card assigned to the acting user, who may set it without the payout:account:update scope.
    
</dd>
</dl>

<dl>
<dd>

**remove_limit:** `Option<bool>` — Pass `true` to remove the spending limit (make the card unlimited).
    
</dd>
</dl>

<dl>
<dd>

**spend_limit:** `Option<f64>` — Spending limit amount, in dollars.
    
</dd>
</dl>

<dl>
<dd>

**spend_limit_frequency:** `Option<UpdateCardsRequestSpendLimitFrequency>` — The window the spend limit applies to.
    
</dd>
</dl>

<dl>
<dd>

**transaction_limit:** `Option<f64>` — Per-transaction limit amount, in dollars.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The owning user ID (a user_ identifier). Provide this or account_id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ChatChannels
<details><summary><code>client.chat_channels.<a href="/src/api/resources/chat_channels/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;String&gt;, product_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListChatChannelsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of chat channels within a specific company, with optional filtering by product.

Required permissions:
 - `chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .chat_channels
        .list(
            &ChatChannelsListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                product_id: Some("prod_xxxxxxxxxxxxx".to_string()),
                after: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to list chat channels for.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<String>` — The unique identifier of a product to filter by. When set, only chat channels connected to this product are returned.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.chat_channels.<a href="/src/api/resources/chat_channels/client.rs">retrieve</a>(id: String) -> Result&lt;ChatChannel, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing chat channel.

Required permissions:
 - `chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.chat_channels.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the chat channel or experience to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.chat_channels.<a href="/src/api/resources/chat_channels/client.rs">update</a>(id: String, request: UpdateChatChannelsRequest) -> Result&lt;ChatChannel, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update moderation settings for a chat channel, such as who can post, banned words, and media restrictions.

Required permissions:
 - `chat:moderate`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .chat_channels
        .update(
            &"id".to_string(),
            &UpdateChatChannelsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the chat channel to update. Accepts either an experience ID (e.g. 'exp_xxxxx') or a chat channel ID.
    
</dd>
</dl>

<dl>
<dd>

**ban_media:** `Option<Option<bool>>` — Whether media uploads such as images and videos are banned in this chat channel.
    
</dd>
</dl>

<dl>
<dd>

**ban_urls:** `Option<Option<bool>>` — Whether URLs and links are banned from being posted in this chat channel.
    
</dd>
</dl>

<dl>
<dd>

**banned_words:** `Option<Option<Vec<String>>>` — A list of words that are automatically blocked from messages in this chat channel. For example, ['spam', 'scam'].
    
</dd>
</dl>

<dl>
<dd>

**user_posts_cooldown_seconds:** `Option<Option<i64>>` — The minimum number of seconds a user must wait between sending messages in this chat channel.
    
</dd>
</dl>

<dl>
<dd>

**who_can_post:** `Option<Option<WhoCanPostTypes>>` — Controls which roles are allowed to send messages in this chat channel.
    
</dd>
</dl>

<dl>
<dd>

**who_can_react:** `Option<Option<WhoCanReactTypes>>` — Controls which roles are allowed to add reactions to messages in this chat channel.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Checkout Configurations
<details><summary><code>client.checkout_configurations.<a href="/src/api/resources/checkout_configurations/client.rs">list</a>(account_id: Option&lt;String&gt;, plan_id: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListCheckoutConfigurationsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListCheckoutConfigurationsRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListCheckoutConfigurationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists checkout configurations for an account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .checkout_configurations
        .list(
            &CheckoutConfigurationsListQueryRequest {
                account_id: "account_id".to_string(),
                plan_id: None,
                created_before: None,
                created_after: None,
                order: None,
                direction: None,
                first: None,
                after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Account ID, prefixed `biz_`.
    
</dd>
</dl>

<dl>
<dd>

**plan_id:** `Option<String>` — Only return checkout configurations for this plan ID, prefixed `plan_`.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return checkout configurations created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return checkout configurations created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListCheckoutConfigurationsRequestOrder>` — Field used to sort checkout configurations.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListCheckoutConfigurationsRequestDirection>` — Sort direction. Defaults to `desc`.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of checkout configurations to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor for the next page of results.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.checkout_configurations.<a href="/src/api/resources/checkout_configurations/client.rs">create</a>(request: CreateCheckoutConfigurationsRequest) -> Result&lt;CreateCheckoutConfigurationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a reusable checkout configuration for an existing or inline plan.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .checkout_configurations
        .create(
            &CreateCheckoutConfigurationsRequest {
                account_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                plan_id: Some("plan_xxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account ID, prefixed `biz_`.
    
</dd>
</dl>

<dl>
<dd>

**affiliate_code:** `Option<Option<String>>` — Affiliate code to apply to the checkout.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<Option<String>>` — Currency used for setup-mode payment method availability.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Custom key-value metadata copied to payments and memberships.
    
</dd>
</dl>

<dl>
<dd>

**mode:** `Option<CreateCheckoutConfigurationsRequestMode>` — Controls whether checkout charges the buyer immediately or saves payment details for later. Defaults to `payment`.
    
</dd>
</dl>

<dl>
<dd>

**payment_method_configuration:** `Option<Option<CreateCheckoutConfigurationsRequestPaymentMethodConfiguration>>` — Payment method overrides for this checkout. `null` uses the plan or platform defaults.
    
</dd>
</dl>

<dl>
<dd>

**plan:** `Option<Option<CreateCheckoutConfigurationsRequestPlan>>` — Plan attributes used to create or find a plan for this checkout configuration. Mutually exclusive with `plan_id`.
    
</dd>
</dl>

<dl>
<dd>

**plan_id:** `Option<Option<String>>` — Existing plan ID, prefixed `plan_`. Mutually exclusive with `plan`.
    
</dd>
</dl>

<dl>
<dd>

**redirect_url:** `Option<Option<String>>` — URL customers are sent to after checkout.
    
</dd>
</dl>

<dl>
<dd>

**three_ds_level:** `Option<Option<CreateCheckoutConfigurationsRequestThreeDsLevel>>` — 3D Secure behavior for this checkout.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.checkout_configurations.<a href="/src/api/resources/checkout_configurations/client.rs">retrieve</a>(id: String) -> Result&lt;RetrieveCheckoutConfigurationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a checkout configuration by ID. This endpoint is public so a checkout page can load from the configuration URL.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .checkout_configurations
        .retrieve(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the checkout configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.checkout_configurations.<a href="/src/api/resources/checkout_configurations/client.rs">delete</a>(id: String) -> Result&lt;DeleteCheckoutConfigurationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a checkout configuration so its checkout URL can no longer be used.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .checkout_configurations
        .delete(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the checkout configuration.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Companies
<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, parent_company_id: Option&lt;Option&lt;String&gt;&gt;, direction: Option&lt;Option&lt;Direction&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListCompaniesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of companies. When parent_company_id is provided, lists connected accounts under that platform. When omitted, lists companies the current user has access to.

Required permissions:
 - `company:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .companies
        .list(
            &CompaniesListQueryRequest {
                first: Some(42),
                last: Some(42),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**parent_company_id:** `Option<String>` — The unique identifier of the parent platform company. When provided, lists connected accounts under that platform. Omit to list the current user's own companies.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<Direction>` 
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return companies created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return companies created after this timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">create</a>(request: CreateCompaniesRequest) -> Result&lt;Company, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new company. Pass parent_company_id to create a connected account under a platform, or omit it to create a company for the current user.

Required permissions:
 - `company:create`
 - `company:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .companies
        .create(
            &CreateCompaniesRequest {
                title: "title".to_string(),
                country: None,
                description: None,
                email: None,
                logo: None,
                metadata: None,
                parent_company_id: None,
                send_customer_emails: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**country:** `Option<Option<Countries>>` — The country the company is located in. Defaults to the parent company's country for connected accounts, or the owner's IP-derived country.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — A promotional pitch displayed to potential customers on the company's store page.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<Option<String>>` — The email address of the user who will own the connected account. Required when parent_company_id is provided.
    
</dd>
</dl>

<dl>
<dd>

**logo:** `Option<Option<CreateCompaniesRequestLogo>>` — The company's logo image. Accepts PNG, JPEG, or GIF format.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — A key-value JSON object of custom metadata to store on the company.
    
</dd>
</dl>

<dl>
<dd>

**parent_company_id:** `Option<Option<String>>` — The unique identifier of the parent platform company. When provided, creates a connected account under that platform. Omit to create a company for the current user.
    
</dd>
</dl>

<dl>
<dd>

**send_customer_emails:** `Option<Option<bool>>` — Whether Whop sends transactional emails to customers on behalf of this company. Only applies when creating a connected account.
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` — The display name of the company shown to customers.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">retrieve</a>(id: String) -> Result&lt;Company, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing company.

Required permissions:
 - `company:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .companies
        .retrieve(&"biz_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier or route slug of the company.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">update</a>(id: String, request: UpdateCompaniesRequest) -> Result&lt;Company, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a company's title, description, logo, and other settings.

Required permissions:
 - `company:update`
 - `company:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .companies
        .update(
            &"biz_xxxxxxxxxxxxxx".to_string(),
            &UpdateCompaniesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the company to update.
    
</dd>
</dl>

<dl>
<dd>

**affiliate_application_required:** `Option<Option<bool>>` — Whether prospective affiliates must submit an application before they can promote this company.
    
</dd>
</dl>

<dl>
<dd>

**affiliate_instructions:** `Option<Option<String>>` — Guidelines and instructions shown to affiliates explaining how to promote this company's products.
    
</dd>
</dl>

<dl>
<dd>

**banner_image:** `Option<Option<UpdateCompaniesRequestBannerImage>>` — The company's banner image. Accepts PNG or JPEG format.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — A promotional pitch displayed to potential customers on the company's store page.
    
</dd>
</dl>

<dl>
<dd>

**featured_affiliate_product_id:** `Option<Option<String>>` — The ID of the product to feature on this company's affiliate page. Pass null to clear.
    
</dd>
</dl>

<dl>
<dd>

**logo:** `Option<Option<UpdateCompaniesRequestLogo>>` — The company's logo image. Accepts PNG, JPEG, or GIF format.
    
</dd>
</dl>

<dl>
<dd>

**route:** `Option<Option<String>>` — The unique URL slug for the company's store page. Must be lowercase and can include hyphens (e.g., 'my-company'). If not provided, the route will remain unchanged.
    
</dd>
</dl>

<dl>
<dd>

**send_customer_emails:** `Option<Option<bool>>` — Whether Whop sends transactional emails (receipts, renewals, cancelations) to customers on behalf of this company.
    
</dd>
</dl>

<dl>
<dd>

**social_links:** `Option<Option<Vec<UpdateCompaniesRequestSocialLinksItem>>>` — The social media links to display on the company's store page. Pass the full list of desired social links — any existing links not included will be removed.
    
</dd>
</dl>

<dl>
<dd>

**target_audience:** `Option<Option<String>>` — The target audience for this company (e.g., 'beginner day traders aged 18-25 looking to learn options').
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The display name of the company shown to customers.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.companies.<a href="/src/api/resources/companies/client.rs">create_api_key</a>(parent_company_id: String, request: CreateApiKeyCompaniesRequest) -> Result&lt;CreateApiKeyCompaniesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create an API key for a connected account (child company) owned by a parent company.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .companies
        .create_api_key(
            &"parent_company_id".to_string(),
            &CreateAPIKeyCompaniesRequest {
                child_company_id: "child_company_id".to_string(),
                name: None,
                permissions: None,
                role: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**parent_company_id:** `String` — The unique identifier of the parent platform company (e.g. 'biz_xxx').
    
</dd>
</dl>

<dl>
<dd>

**child_company_id:** `String` — The unique identifier of the connected account to create the API key for (e.g. 'biz_xxx').
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>` — A human-readable name for the API key, such as 'Production API Key'.
    
</dd>
</dl>

<dl>
<dd>

**permissions:** `Option<Option<Vec<CreateApiKeyCompaniesRequestPermissionsItem>>>` — Granular permission statements defining which actions this API key can perform. Either permissions or role must be provided.
    
</dd>
</dl>

<dl>
<dd>

**role:** `Option<Option<PermissionSystemRoles>>` — A system role to inherit permissions from (e.g. owner, admin, moderator). Either role or permissions must be provided.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CompanyTokenTransactions
<details><summary><code>client.company_token_transactions.<a href="/src/api/resources/company_token_transactions/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;String&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, transaction_type: Option&lt;Option&lt;CompanyTokenTransactionTypes&gt;&gt;) -> Result&lt;ListCompanyTokenTransactionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of token transactions for a user or company, depending on the authenticated actor, with optional filtering by user and transaction type.

Required permissions:
 - `company_token_transaction:read`
 - `member:basic:read`
 - `company:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .company_token_transactions
        .list(
            &CompanyTokenTransactionsListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                user_id: Some("user_xxxxxxxxxxxxx".to_string()),
                after: None,
                before: None,
                transaction_type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to list token transactions for.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Filter transactions to only those involving this specific user.
    
</dd>
</dl>

<dl>
<dd>

**transaction_type:** `Option<CompanyTokenTransactionTypes>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.company_token_transactions.<a href="/src/api/resources/company_token_transactions/client.rs">create</a>(request: CreateCompanyTokenTransactionsRequestBody) -> Result&lt;CompanyTokenTransaction, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a token transaction to add, subtract, or transfer tokens for a member within a company.

Required permissions:
 - `company_token_transaction:create`
 - `member:basic:read`
 - `company:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .company_token_transactions
        .create(
            &CreateCompanyTokenTransactionsRequestBody::Transfer {
                data: CreateCompanyTokenTransactionsRequestBodyTransfer {
                    amount: 6.9,
                    company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                    destination_user_id: "destination_user_id".to_string(),
                    user_id: "user_xxxxxxxxxxxxx".to_string(),
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.company_token_transactions.<a href="/src/api/resources/company_token_transactions/client.rs">retrieve</a>(id: String) -> Result&lt;CompanyTokenTransaction, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing company token transaction.

Required permissions:
 - `company_token_transaction:read`
 - `member:basic:read`
 - `company:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .company_token_transactions
        .retrieve(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the token transaction to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CourseChapters
<details><summary><code>client.course_chapters.<a href="/src/api/resources/course_chapters/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, course_id: Option&lt;String&gt;) -> Result&lt;ListCourseChaptersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of chapters within a course, ordered by position.

Required permissions:
 - `courses:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_chapters
        .list(
            &CourseChaptersListQueryRequest {
                first: Some(42),
                last: Some(42),
                course_id: "cors_xxxxxxxxxxxxx".to_string(),
                after: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**course_id:** `String` — The unique identifier of the course to list chapters for.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_chapters.<a href="/src/api/resources/course_chapters/client.rs">create</a>(request: CreateCourseChaptersRequest) -> Result&lt;CourseChapter, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new chapter within a course to organize lessons into sections.

Required permissions:
 - `courses:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_chapters
        .create(
            &CreateCourseChaptersRequest {
                course_id: "cors_xxxxxxxxxxxxx".to_string(),
                title: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**course_id:** `String` — The unique identifier of the course to create the chapter in (e.g., "course_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The display title of the chapter (e.g., "Module 1: Introduction").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_chapters.<a href="/src/api/resources/course_chapters/client.rs">retrieve</a>(id: String) -> Result&lt;CourseChapter, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing course chapter.

Required permissions:
 - `courses:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_chapters
        .retrieve(&"chap_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the chapter to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_chapters.<a href="/src/api/resources/course_chapters/client.rs">delete</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a chapter and all of its lessons from a course.

Required permissions:
 - `courses:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_chapters
        .delete(&"chap_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the chapter to delete (e.g., "chap_XXXXX").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_chapters.<a href="/src/api/resources/course_chapters/client.rs">update</a>(id: String, request: UpdateCourseChaptersRequest) -> Result&lt;CourseChapter, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a chapter's title within a course.

Required permissions:
 - `courses:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_chapters
        .update(
            &"chap_xxxxxxxxxxxxx".to_string(),
            &UpdateCourseChaptersRequest {
                title: "title".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the chapter to update (e.g., "chap_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` — The new display title of the chapter (e.g., "Module 1: Introduction").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CourseLessonInteractions
<details><summary><code>client.course_lesson_interactions.<a href="/src/api/resources/course_lesson_interactions/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, lesson_id: Option&lt;Option&lt;String&gt;&gt;, course_id: Option&lt;Option&lt;String&gt;&gt;, completed: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListCourseLessonInteractionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of lesson interactions, filtered by lesson, course, user, or completion status.

Required permissions:
 - `courses:read`
 - `course_analytics:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_lesson_interactions
        .list(
            &CourseLessonInteractionsListQueryRequest {
                first: Some(42),
                last: Some(42),
                user_id: Some("user_xxxxxxxxxxxxx".to_string()),
                lesson_id: Some("lesn_xxxxxxxxxxxxx".to_string()),
                course_id: Some("cors_xxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The unique identifier of the user to filter lesson interactions for.
    
</dd>
</dl>

<dl>
<dd>

**lesson_id:** `Option<String>` — The unique identifier of the lesson to filter interactions for.
    
</dd>
</dl>

<dl>
<dd>

**course_id:** `Option<String>` — The unique identifier of the course to filter interactions for.
    
</dd>
</dl>

<dl>
<dd>

**completed:** `Option<bool>` — Whether to filter for completed or in-progress lesson interactions.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_lesson_interactions.<a href="/src/api/resources/course_lesson_interactions/client.rs">retrieve</a>(id: String) -> Result&lt;CourseLessonInteraction, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing course lesson interaction.

Required permissions:
 - `courses:read`
 - `course_analytics:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_lesson_interactions
        .retrieve(&"crsli_xxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the lesson interaction to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CourseLessons
<details><summary><code>client.course_lessons.<a href="/src/api/resources/course_lessons/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, course_id: Option&lt;Option&lt;String&gt;&gt;, chapter_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListCourseLessonsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of lessons within a course or chapter, ordered by position.

Required permissions:
 - `courses:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_lessons
        .list(
            &CourseLessonsListQueryRequest {
                first: Some(42),
                last: Some(42),
                course_id: Some("cors_xxxxxxxxxxxxx".to_string()),
                chapter_id: Some("chap_xxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**course_id:** `Option<String>` — The unique identifier of the course to return all lessons across all chapters.
    
</dd>
</dl>

<dl>
<dd>

**chapter_id:** `Option<String>` — The unique identifier of a chapter to return only its lessons.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_lessons.<a href="/src/api/resources/course_lessons/client.rs">create</a>(request: CreateCourseLessonsRequest) -> Result&lt;CourseLesson, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new lesson within a course chapter. Lessons can contain video, text, or assessment content.

Required permissions:
 - `courses:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_lessons
        .create(
            &CreateCourseLessonsRequest {
                chapter_id: "chap_xxxxxxxxxxxxx".to_string(),
                lesson_type: LessonTypes::Text,
                content: None,
                days_from_course_start_until_unlock: None,
                embed_id: None,
                embed_type: None,
                thumbnail: None,
                title: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**chapter_id:** `String` — The unique identifier of the chapter to create the lesson in (e.g., "chap_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**content:** `Option<Option<String>>` — The Markdown content body of the lesson.
    
</dd>
</dl>

<dl>
<dd>

**days_from_course_start_until_unlock:** `Option<Option<i64>>` — The number of days after a student starts the course before this lesson becomes accessible.
    
</dd>
</dl>

<dl>
<dd>

**embed_id:** `Option<Option<String>>` — The external video identifier for embedded content (e.g., a YouTube video ID or Loom share ID).
    
</dd>
</dl>

<dl>
<dd>

**embed_type:** `Option<Option<EmbedTypes>>` — The type of video embed for this lesson, such as YouTube or Loom.
    
</dd>
</dl>

<dl>
<dd>

**lesson_type:** `LessonTypes` — The content type of the lesson, such as video, text, quiz, or knowledge check.
    
</dd>
</dl>

<dl>
<dd>

**thumbnail:** `Option<Option<CreateCourseLessonsRequestThumbnail>>` — The thumbnail image for the lesson in PNG, JPEG, or GIF format.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The display title of the lesson (e.g., "Getting Started with APIs").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_lessons.<a href="/src/api/resources/course_lessons/client.rs">retrieve</a>(id: String) -> Result&lt;CourseLesson, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing course lesson.

Required permissions:
 - `courses:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_lessons
        .retrieve(&"lesn_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the lesson to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_lessons.<a href="/src/api/resources/course_lessons/client.rs">delete</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a lesson and remove it from its chapter.

Required permissions:
 - `courses:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_lessons
        .delete(&"lesn_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the lesson to delete (e.g., "les_XXXXX").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_lessons.<a href="/src/api/resources/course_lessons/client.rs">update</a>(id: String, request: UpdateCourseLessonsRequest) -> Result&lt;CourseLesson, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a lesson's content, type, visibility, assessment questions, or media attachments.

Required permissions:
 - `courses:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_lessons
        .update(
            &"lesn_xxxxxxxxxxxxx".to_string(),
            &UpdateCourseLessonsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the lesson to update (e.g., "les_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**assessment_completion_requirement:** `Option<Option<UpdateCourseLessonsRequestAssessmentCompletionRequirement>>` — The passing criteria for quiz or knowledge check lessons, such as minimum grade or correct answers.
    
</dd>
</dl>

<dl>
<dd>

**assessment_questions:** `Option<Option<Vec<UpdateCourseLessonsRequestAssessmentQuestionsItem>>>` — The full list of assessment questions for quiz or knowledge check lessons. Replaces all existing questions.
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Option<Vec<UpdateCourseLessonsRequestAttachmentsItem>>>` — File attachments for the lesson such as PDFs or documents. Replaces all existing attachments.
    
</dd>
</dl>

<dl>
<dd>

**content:** `Option<Option<String>>` — The Markdown content body of the lesson.
    
</dd>
</dl>

<dl>
<dd>

**days_from_course_start_until_unlock:** `Option<Option<i64>>` — The number of days after a student starts the course before this lesson becomes accessible.
    
</dd>
</dl>

<dl>
<dd>

**embed_id:** `Option<Option<String>>` — The external video identifier for embedded content (e.g., a YouTube video ID or Loom share ID).
    
</dd>
</dl>

<dl>
<dd>

**embed_type:** `Option<Option<EmbedTypes>>` — The type of video embed for this lesson, such as YouTube or Loom.
    
</dd>
</dl>

<dl>
<dd>

**lesson_type:** `Option<Option<LessonTypes>>` — The content type of the lesson, such as video, text, quiz, or knowledge check.
    
</dd>
</dl>

<dl>
<dd>

**main_pdf:** `Option<Option<UpdateCourseLessonsRequestMainPdf>>` — The primary PDF document attached to this lesson for student reference.
    
</dd>
</dl>

<dl>
<dd>

**max_attempts:** `Option<Option<i64>>` — The maximum number of attempts a student is allowed for assessment lessons.
    
</dd>
</dl>

<dl>
<dd>

**mux_asset_id:** `Option<Option<String>>` — The identifier of a Mux video asset to attach to this lesson (e.g., "mux_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**thumbnail:** `Option<Option<UpdateCourseLessonsRequestThumbnail>>` — The thumbnail image for the lesson in PNG, JPEG, or GIF format.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The display title of the lesson (e.g., "Getting Started with APIs").
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<Option<LessonVisibilities>>` — Controls whether this lesson is visible to students or hidden as a draft.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_lessons.<a href="/src/api/resources/course_lessons/client.rs">mark_as_completed</a>(lesson_id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Mark a lesson as completed for the current user after they finish the content.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_lessons
        .mark_as_completed(&"lesson_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**lesson_id:** `String` — The unique identifier of the lesson to mark as completed (e.g., "les_XXXXX").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_lessons.<a href="/src/api/resources/course_lessons/client.rs">start</a>(lesson_id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Record that the current user has started viewing a lesson, creating progress tracking records.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_lessons
        .start(&"lesson_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**lesson_id:** `String` — The unique identifier of the lesson the user is starting (e.g., "les_XXXXX").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_lessons.<a href="/src/api/resources/course_lessons/client.rs">submit_assessment</a>(lesson_id: String, request: SubmitAssessmentCourseLessonsRequest) -> Result&lt;SubmitAssessmentCourseLessonsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Submit answers for a quiz or knowledge check lesson and receive a graded result.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_lessons
        .submit_assessment(
            &"lesson_id".to_string(),
            &SubmitAssessmentCourseLessonsRequest {
                answers: vec![SubmitAssessmentCourseLessonsRequestAnswersItem {
                    question_id: "question_id".to_string(),
                    ..Default::default()
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**lesson_id:** `String` — The unique identifier of the quiz or knowledge check lesson to submit answers for (e.g., "les_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**answers:** `Vec<SubmitAssessmentCourseLessonsRequestAnswersItem>` — The list of answers to submit for each assessment question.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CourseStudents
<details><summary><code>client.course_students.<a href="/src/api/resources/course_students/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, course_id: Option&lt;String&gt;, keyword: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListCourseStudentsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of students enrolled in a course, with optional name filtering.

Required permissions:
 - `courses:read`
 - `course_analytics:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_students
        .list(
            &CourseStudentsListQueryRequest {
                first: Some(42),
                last: Some(42),
                course_id: "cors_xxxxxxxxxxxxx".to_string(),
                after: None,
                before: None,
                keyword: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**course_id:** `String` — The unique identifier of the course to list enrolled students for.
    
</dd>
</dl>

<dl>
<dd>

**keyword:** `Option<String>` — A search term to filter students by name or username.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.course_students.<a href="/src/api/resources/course_students/client.rs">retrieve</a>(id: String) -> Result&lt;CourseStudent, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing course student.

Required permissions:
 - `courses:read`
 - `course_analytics:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .course_students
        .retrieve(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the course student record to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Courses
<details><summary><code>client.courses.<a href="/src/api/resources/courses/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, experience_id: Option&lt;Option&lt;String&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListCoursesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of courses, filtered by either an experience or a company.

Required permissions:
 - `courses:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .courses
        .list(
            &CoursesListQueryRequest {
                first: Some(42),
                last: Some(42),
                experience_id: Some("exp_xxxxxxxxxxxxxx".to_string()),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**experience_id:** `Option<String>` — The unique identifier of the experience to list courses for.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of the company to list courses for.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.courses.<a href="/src/api/resources/courses/client.rs">create</a>(request: CreateCoursesRequest) -> Result&lt;Course, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new course within an experience, with optional chapters, lessons, and a certificate.

Required permissions:
 - `courses:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .courses
        .create(
            &CreateCoursesRequest {
                experience_id: "exp_xxxxxxxxxxxxxx".to_string(),
                title: "title".to_string(),
                certificate_after_completion_enabled: None,
                order: None,
                require_completing_lessons_in_order: None,
                tagline: None,
                thumbnail: None,
                visibility: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**certificate_after_completion_enabled:** `Option<Option<bool>>` — Whether the course awards students a PDF certificate after completing all lessons.
    
</dd>
</dl>

<dl>
<dd>

**experience_id:** `String` — The unique identifier of the experience to create the course in (e.g., "exp_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<Option<String>>` — The decimal order position of the course within its experience. Use fractional values (e.g., "1.5") to place between existing courses.
    
</dd>
</dl>

<dl>
<dd>

**require_completing_lessons_in_order:** `Option<Option<bool>>` — Whether students must complete each lesson sequentially before advancing to the next one.
    
</dd>
</dl>

<dl>
<dd>

**tagline:** `Option<Option<String>>` — A short tagline displayed beneath the course title (e.g., "Master the fundamentals of design").
    
</dd>
</dl>

<dl>
<dd>

**thumbnail:** `Option<Option<CreateCoursesRequestThumbnail>>` — The thumbnail image for the course in PNG, JPEG, or GIF format.
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` — The display title of the course (e.g., "Introduction to Web Development").
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<Option<CourseVisibilities>>` — Controls whether this course is visible to students or hidden as a draft.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.courses.<a href="/src/api/resources/courses/client.rs">retrieve</a>(id: String) -> Result&lt;Course, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing course.

Required permissions:
 - `courses:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .courses
        .retrieve(&"cors_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the course to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.courses.<a href="/src/api/resources/courses/client.rs">delete</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a course and all of its chapters, lessons, and student progress.

Required permissions:
 - `courses:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .courses
        .delete(&"cors_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the course to delete (e.g., "course_XXXXX").
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.courses.<a href="/src/api/resources/courses/client.rs">update</a>(id: String, request: UpdateCoursesRequest) -> Result&lt;Course, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a course's title, description, visibility, thumbnail, or chapter ordering.

Required permissions:
 - `courses:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .courses
        .update(
            &"cors_xxxxxxxxxxxxx".to_string(),
            &UpdateCoursesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the course to update (e.g., "course_XXXXX").
    
</dd>
</dl>

<dl>
<dd>

**certificate_after_completion_enabled:** `Option<Option<bool>>` — Whether the course awards students a PDF certificate after completing all lessons.
    
</dd>
</dl>

<dl>
<dd>

**chapters:** `Option<Option<Vec<UpdateCoursesRequestChaptersItem>>>` — A list of chapters with nested lessons to reorder or rename in bulk.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — A short description of the course displayed to students on the course page.
    
</dd>
</dl>

<dl>
<dd>

**language:** `Option<Option<Languages>>` — The primary language spoken in the video content of the course.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<Option<String>>` — The decimal order position of the course within its experience. Use fractional values (e.g., "1.5") to place between existing courses.
    
</dd>
</dl>

<dl>
<dd>

**require_completing_lessons_in_order:** `Option<Option<bool>>` — Whether students must complete each lesson sequentially before advancing to the next one.
    
</dd>
</dl>

<dl>
<dd>

**tagline:** `Option<Option<String>>` — A short tagline displayed beneath the course title (e.g., "Master the fundamentals of design").
    
</dd>
</dl>

<dl>
<dd>

**thumbnail:** `Option<Option<UpdateCoursesRequestThumbnail>>` — The thumbnail image for the course in PNG, JPEG, or GIF format.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The display title of the course (e.g., "Introduction to Web Development").
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<Option<CourseVisibilities>>` — Controls whether this course is visible to students or hidden as a draft.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Deposits
<details><summary><code>client.deposits.<a href="/src/api/resources/deposits/client.rs">create</a>(request: CreateDepositsRequest) -> Result&lt;CreateDepositsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the deposit methods for an account, including crypto and bank transfer.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .deposits
        .create(
            &CreateDepositsRequest {
                destination: CreateDepositsRequestDestination::String("destination".to_string()),
                amount: None,
                metadata: None,
                network: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**amount:** `Option<f64>` — Amount to prefill on hosted deposit page.
    
</dd>
</dl>

<dl>
<dd>

**destination:** `CreateDepositsRequestDestination` — Destination account ID or wallet address. Object form is supported for compatibility. Any business resolves by its account ID without authentication; a user account resolves only for that same authenticated user.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Metadata to include with the deposit response.
    
</dd>
</dl>

<dl>
<dd>

**network:** `Option<Option<CreateDepositsRequestNetwork>>` — Destination network override. Defaults to the destination wallet's own network.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Dispute alerts
<details><summary><code>client.dispute_alerts.<a href="/src/api/resources/dispute_alerts/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, payment_id: Option&lt;Option&lt;String&gt;&gt;, type_: Option&lt;Option&lt;ListDisputeAlertsRequestType&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListDisputeAlertsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListDisputeAlertsRequestDirection&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListDisputeAlertsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the dispute alerts and early fraud warnings across the accounts you can read.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .dispute_alerts
        .list(
            &DisputeAlertsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Only alerts on this account's payments (`biz_` tag). Omit it to cover every account you can read.
    
</dd>
</dl>

<dl>
<dd>

**payment_id:** `Option<String>` — Only alerts on this payment (`pay_` tag). A payment can carry several.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<ListDisputeAlertsRequestType>` — Only alerts of this kind. `early_fraud_warning` for issuer fraud reports, `dispute_alert` for pre-dispute notices, `rapid_dispute_resolution` for Visa RDR cases the network already closed.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of alerts to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns alerts after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of alerts to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns alerts before this position.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListDisputeAlertsRequestOrder>` — The field to sort alerts by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListDisputeAlertsRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only alerts Whop received before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only alerts Whop received after this ISO 8601 timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dispute_alerts.<a href="/src/api/resources/dispute_alerts/client.rs">retrieve</a>(id: String) -> Result&lt;DisputeAlert, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single dispute alert or early fraud warning by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .dispute_alerts
        .retrieve(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The dispute alert ID, prefixed `dspa_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Disputes
<details><summary><code>client.disputes.<a href="/src/api/resources/disputes/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListDisputesRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListDisputesRequestDirection&gt;&gt;, currency: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListDisputesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the disputes across the accounts you can read.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .disputes
        .list(
            &DisputesListQueryRequest {
                account_id: None,
                first: None,
                after: None,
                last: None,
                before: None,
                order: None,
                direction: None,
                status: vec![],
                currency: None,
                created_before: None,
                created_after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Only disputes filed against this account (`biz_` tag). Omit it to cover every account you can read.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of disputes to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns disputes after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of disputes to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns disputes before this position.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListDisputesRequestOrder>` — The field to sort disputes by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListDisputesRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListDisputesRequestStatusItem>` — Only disputes in these statuses. Repeat the parameter to pass several — one paginated list covers all of them. Covers both chargebacks and inquiries at each stage. A `needs_response` dispute whose evidence deadline has passed reports and filters as `under_review` instead.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — Only disputes in this three-letter ISO currency.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only disputes opened before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only disputes opened after this ISO 8601 timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.disputes.<a href="/src/api/resources/disputes/client.rs">summary</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, currency: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;SummaryDisputesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Totals up the same disputes the list returns, so you can build status tabs and totals without paging through them.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .disputes
        .summary(
            &DisputesSummaryQueryRequest {
                groups: vec![],
                account_id: None,
                status: vec![],
                currency: None,
                created_before: None,
                created_after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**groups:** `Option<SummaryDisputesRequestGroupsItem>` — Which breakdowns to return, keyed by these names under `groups`. Repeat the parameter to ask for several; omit it for all of them.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — Only disputes filed against this account (`biz_` tag). Omit it to cover every account you can read.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<SummaryDisputesRequestStatusItem>` — Only disputes in these statuses. Repeat the parameter to pass several. A `needs_response` dispute whose evidence deadline has passed reports and filters as `under_review` instead.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — Only disputes in this three-letter ISO currency.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only disputes opened before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only disputes opened after this ISO 8601 timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.disputes.<a href="/src/api/resources/disputes/client.rs">retrieve</a>(id: String) -> Result&lt;Dispute, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single dispute.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.disputes.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The dispute ID (`dspt_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.disputes.<a href="/src/api/resources/disputes/client.rs">update</a>(id: String, request: UpdateDisputesRequest) -> Result&lt;Dispute, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Edits a dispute's evidence, while it is still editable. Sending it is a separate call.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .disputes
        .update(
            &"id".to_string(),
            &UpdateDisputesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The dispute ID (`dspt_` tag).
    
</dd>
</dl>

<dl>
<dd>

**evidence:** `Option<UpdateDisputesRequestEvidence>` — The evidence packet to send to the processor. Only the fields you provide are changed.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.disputes.<a href="/src/api/resources/disputes/client.rs">submit</a>(id: String) -> Result&lt;Dispute, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Sends a dispute's evidence to the payment processor. This is final — it cannot be edited or sent again.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.disputes.submit(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The dispute ID (`dspt_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.disputes.<a href="/src/api/resources/disputes/client.rs">submit_evidence_dispute</a>(id: String) -> Result&lt;Dispute, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Submit a payment dispute to the payment processor for review. Once submitted, no further edits can be made.

Required permissions:
 - `payment:dispute`
 - `plan:basic:read`
 - `access_pass:basic:read`
 - `company:basic:read`
 - `payment:basic:read`
 - `member:email:read`
 - `member:basic:read`
 - `member:phone:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .disputes
        .submit_evidence_dispute(&"dspt_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the dispute to submit to the payment processor for review.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.disputes.<a href="/src/api/resources/disputes/client.rs">update_evidence_dispute</a>(id: String, request: UpdateEvidenceDisputeRequest) -> Result&lt;Dispute, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a dispute with evidence data to attempt to win the dispute.

Required permissions:
 - `payment:dispute`
 - `plan:basic:read`
 - `access_pass:basic:read`
 - `company:basic:read`
 - `payment:basic:read`
 - `member:email:read`
 - `member:basic:read`
 - `member:phone:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .disputes
        .update_evidence_dispute(
            &"dspt_xxxxxxxxxxxxx".to_string(),
            &UpdateEvidenceDisputeRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the dispute to update.
    
</dd>
</dl>

<dl>
<dd>

**access_activity_log:** `Option<Option<String>>` — An IP access activity log showing the customer used the service.
    
</dd>
</dl>

<dl>
<dd>

**billing_address:** `Option<Option<String>>` — The billing address associated with the customer's payment method.
    
</dd>
</dl>

<dl>
<dd>

**cancellation_policy_attachment:** `Option<Option<UpdateEvidenceDisputeRequestCancellationPolicyAttachment>>` — A file upload containing the company's cancellation policy document.
    
</dd>
</dl>

<dl>
<dd>

**cancellation_policy_disclosure:** `Option<Option<String>>` — The company's cancellation policy text to submit as evidence.
    
</dd>
</dl>

<dl>
<dd>

**customer_communication_attachment:** `Option<Option<UpdateEvidenceDisputeRequestCustomerCommunicationAttachment>>` — A file upload containing evidence of customer communication. Must be a JPEG, PNG, GIF, or PDF.
    
</dd>
</dl>

<dl>
<dd>

**customer_email_address:** `Option<Option<String>>` — The email address of the customer associated with the disputed payment.
    
</dd>
</dl>

<dl>
<dd>

**customer_name:** `Option<Option<String>>` — The full name of the customer associated with the disputed payment.
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` — Additional notes or context to submit as part of the dispute evidence.
    
</dd>
</dl>

<dl>
<dd>

**product_description:** `Option<Option<String>>` — A description of the product or service that was provided to the customer.
    
</dd>
</dl>

<dl>
<dd>

**refund_policy_attachment:** `Option<Option<UpdateEvidenceDisputeRequestRefundPolicyAttachment>>` — A file upload containing the company's refund policy document.
    
</dd>
</dl>

<dl>
<dd>

**refund_policy_disclosure:** `Option<Option<String>>` — The company's refund policy text to submit as evidence.
    
</dd>
</dl>

<dl>
<dd>

**refund_refusal_explanation:** `Option<Option<String>>` — An explanation of why the refund request was refused.
    
</dd>
</dl>

<dl>
<dd>

**service_date:** `Option<Option<String>>` — The date when the product or service was delivered to the customer.
    
</dd>
</dl>

<dl>
<dd>

**uncategorized_attachment:** `Option<Option<UpdateEvidenceDisputeRequestUncategorizedAttachment>>` — A file upload for evidence that does not fit into the other categories.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.disputes.<a href="/src/api/resources/disputes/client.rs">upload_evidence</a>(id: String, request: UploadEvidenceDisputesRequest) -> Result&lt;Dispute, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replaces the full set of uploaded evidence documents on a dispute, beyond the four fixed evidence slots. Upload files through `POST /files` and reference them by `id`, or send the files as multipart file parts to upload and attach in one call. Send every document the packet should carry — up to 10, 10MB each and 25MB in total; an empty list removes them all. Accepted content types: application/pdf, application/json, image/jpeg, image/png, image/webp — any other type is rejected.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .disputes
        .upload_evidence(
            &"id".to_string(),
            &UploadEvidenceDisputesRequest {
                documents: vec![UploadEvidenceDisputesRequestDocumentsItem {
                    direct_upload_id: None,
                    document_type:
                        UploadEvidenceDisputesRequestDocumentsItemDocumentType::ReturnPolicy,
                    file: None,
                    id: None,
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The dispute ID (`dspt_` tag).
    
</dd>
</dl>

<dl>
<dd>

**documents:** `Vec<UploadEvidenceDisputesRequestDocumentsItem>` — The full set of evidence documents the dispute should carry. Replaces all previously uploaded documents.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## DmChannels
<details><summary><code>client.dm_channels.<a href="/src/api/resources/dm_channels/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListDmChannelsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of DM channels for the currently authenticated user, sorted by most recently active.

Required permissions:
 - `dms:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .dm_channels
        .list(
            &DmChannelsListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of a company to filter DM channels by. Only returns channels scoped to this company.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dm_channels.<a href="/src/api/resources/dm_channels/client.rs">create</a>(request: CreateDmChannelsRequest) -> Result&lt;DmChannel, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new DM channel between two or more users, optionally scoped to a specific company. Returns the existing channel if one already exists.

Required permissions:
 - `dms:channel:manage`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .dm_channels
        .create(
            &CreateDmChannelsRequest {
                with_user_ids: vec!["with_user_ids".to_string()],
                company_id: None,
                custom_name: None,
                notifications_enabled: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `Option<Option<String>>` — The unique identifier of the company to scope this DM channel to. When set, the channel is visible only within that company context.
    
</dd>
</dl>

<dl>
<dd>

**custom_name:** `Option<Option<String>>` — A custom display name for the DM channel. For example, 'Project Discussion'.
    
</dd>
</dl>

<dl>
<dd>

**notifications_enabled:** `Option<Option<bool>>` — Whether Whop app notifications are enabled for this direct message channel. Webhooks still fire.
    
</dd>
</dl>

<dl>
<dd>

**with_user_ids:** `Vec<String>` — The list of user identifiers to include in the DM channel. Each entry can be an email, username, or user ID (e.g. 'user_xxxxx').
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dm_channels.<a href="/src/api/resources/dm_channels/client.rs">retrieve</a>(id: String) -> Result&lt;DmChannel, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing DM channel.

Required permissions (one of):
 - `dms:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.dm_channels.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the DM channel to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dm_channels.<a href="/src/api/resources/dm_channels/client.rs">delete</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a DM channel and all of its messages. Only an admin of the channel can perform this action.

Required permissions (one of):
 - `dms:channel:manage`
 - `support_chat:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.dm_channels.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the DM channel to delete.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dm_channels.<a href="/src/api/resources/dm_channels/client.rs">update</a>(id: String, request: UpdateDmChannelsRequest) -> Result&lt;DmChannel, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the settings of an existing DM channel, such as its display name. Only an admin of the channel can perform this action.

Required permissions (one of):
 - `dms:channel:manage`
 - `support_chat:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .dm_channels
        .update(
            &"id".to_string(),
            &UpdateDmChannelsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the DM channel to update.
    
</dd>
</dl>

<dl>
<dd>

**custom_name:** `Option<Option<String>>` — A new custom display name for the DM channel. For example, 'Project Discussion'.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## DmMembers
<details><summary><code>client.dm_members.<a href="/src/api/resources/dm_members/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, channel_id: Option&lt;String&gt;) -> Result&lt;ListDmMembersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of members in a specific DM channel, sorted by the date they were added.

Required permissions (one of):
 - `dms:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .dm_members
        .list(
            &DmMembersListQueryRequest {
                first: Some(42),
                last: Some(42),
                channel_id: "channel_id".to_string(),
                after: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**channel_id:** `String` — The unique identifier of the DM channel to list members for.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dm_members.<a href="/src/api/resources/dm_members/client.rs">create</a>(request: CreateDmMembersRequest) -> Result&lt;DmMember, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a new user to an existing DM channel. Only an admin of the channel can add members.

Required permissions (one of):
 - `dms:message:manage`
 - `support_chat:message:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .dm_members
        .create(
            &CreateDmMembersRequest {
                channel_id: "channel_id".to_string(),
                user_id: "user_xxxxxxxxxxxxx".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**channel_id:** `String` — The unique identifier of the DM channel to add the new member to.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `String` — The unique identifier of the user to add to the DM channel. For example, 'user_xxxxx'.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dm_members.<a href="/src/api/resources/dm_members/client.rs">retrieve</a>(id: String) -> Result&lt;DmMember, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing DM member.

Required permissions (one of):
 - `dms:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.dm_members.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the DM channel member to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dm_members.<a href="/src/api/resources/dm_members/client.rs">delete</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a user from a DM channel. An admin can remove any member, and a member can remove themselves.

Required permissions (one of):
 - `dms:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.dm_members.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the DM channel member to remove.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dm_members.<a href="/src/api/resources/dm_members/client.rs">update</a>(id: String, request: UpdateDmMembersRequest) -> Result&lt;DmMember, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a DM channel member's settings, such as their notification preferences or membership status.

Required permissions (one of):
 - `dms:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .dm_members
        .update(
            &"id".to_string(),
            &UpdateDmMembersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the DM channel member to update.
    
</dd>
</dl>

<dl>
<dd>

**notification_preference:** `Option<Option<DmsFeedMemberNotificationPreferences>>` — The notification setting for this member, controlling how they receive alerts for new messages in this channel.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<Option<DmsFeedMemberStatuses>>` — The membership status for this member in the DM channel.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Entries
<details><summary><code>client.entries.<a href="/src/api/resources/entries/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;String&gt;, direction: Option&lt;Option&lt;Direction&gt;&gt;, order: Option&lt;Option&lt;EntriesSortableColumns&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListEntriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of waitlist entries for a company, with optional filtering by product, plan, status, and creation date.

Required permissions:
 - `plan:waitlist:read`
 - `member:email:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .entries
        .list(
            &EntriesListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                after: None,
                before: None,
                direction: None,
                order: None,
                product_ids: vec![],
                plan_ids: vec![],
                statuses: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to list waitlist entries for.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<Direction>` 
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<EntriesSortableColumns>` 
    
</dd>
</dl>

<dl>
<dd>

**product_ids:** `Option<String>` — Filter entries to only those for specific products.
    
</dd>
</dl>

<dl>
<dd>

**plan_ids:** `Option<String>` — Filter entries to only those for specific plans.
    
</dd>
</dl>

<dl>
<dd>

**statuses:** `Option<EntryStatus>` — Filter entries by their current status.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return entries created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return entries created after this timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entries.<a href="/src/api/resources/entries/client.rs">retrieve</a>(id: String) -> Result&lt;Entry, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing waitlist entry.

Required permissions:
 - `plan:waitlist:read`
 - `member:email:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .entries
        .retrieve(&"entry_xxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the waitlist entry to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entries.<a href="/src/api/resources/entries/client.rs">approve</a>(id: String) -> Result&lt;ApproveEntriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Approve a pending waitlist entry, triggering the checkout process to grant the user access to the plan.

Required permissions:
 - `plan:waitlist:manage`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .entries
        .approve(&"entry_xxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the waitlist entry to approve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entries.<a href="/src/api/resources/entries/client.rs">deny</a>(id: String) -> Result&lt;Entry, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deny a pending waitlist entry, preventing the user from gaining access to the plan.

Required permissions:
 - `plan:waitlist:manage`
 - `plan:basic:read`
 - `member:email:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .entries
        .deny(&"entry_xxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the waitlist entry to deny.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Events
<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">list</a>(identifier: Option&lt;Option&lt;String&gt;&gt;, account_id: Option&lt;Option&lt;String&gt;&gt;, from: Option&lt;Option&lt;String&gt;&gt;, to: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, direction: Option&lt;Option&lt;ListEventsRequestDirection&gt;&gt;, event: Option&lt;Option&lt;String&gt;&gt;, source: Option&lt;Option&lt;String&gt;&gt;, attribution_model: Option&lt;Option&lt;ListEventsRequestAttributionModel&gt;&gt;, country: Option&lt;Option&lt;String&gt;&gt;, city: Option&lt;Option&lt;String&gt;&gt;, device: Option&lt;Option&lt;String&gt;&gt;, browser: Option&lt;Option&lt;String&gt;&gt;, os: Option&lt;Option&lt;String&gt;&gt;, utm_source: Option&lt;Option&lt;String&gt;&gt;, hostname: Option&lt;Option&lt;String&gt;&gt;, page: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListEventsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists identity-linked events, most recent first by default. Pass identifier for one person's journey, or omit it to list events for an account within an explicit time range. Pass direction=asc to read a journey forwards from where it starts. Events are shaped like the POST /events intake: attribution in context, identity in user.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .events
        .list(
            &EventsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**identifier:** `Option<String>` — Any hard identifier of the person: a person ID (prsn_*), user ID, email, phone number, or a tracking cookie value (wuid, anonymous ID, fbp/fbc/ttp/ga). Omit to list recent events for the account.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — Account ID, prefixed `biz_`. Optional for account API keys; required for credentials that can access multiple accounts.
    
</dd>
</dl>

<dl>
<dd>

**from:** `Option<String>` — Start of the time range as an ISO 8601 timestamp. Required when identifier is omitted.
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<String>` — End of the time range as an ISO 8601 timestamp. Required when identifier is omitted; otherwise defaults to now.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of events to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor for fetching events after a previous page.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor for fetching events before a later page.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListEventsRequestDirection>` — The order events are returned in by time. Defaults to desc (most recent first); asc reads a journey forwards from where it starts. after and before always page forwards and backwards through that order.
    
</dd>
</dl>

<dl>
<dd>

**event:** `Option<String>` — Full event names to filter by, comma-separated (payment.completed, pixel.lead, pixel.page, pixel.custom:<name>) — the same vocabulary the events / people metrics use.
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<String>` — Canonical source path, exact or with a trailing :* prefix (whop:<campaign>:*, ext:meta:*, referrer:<domain>, direct). Restricts the list to conversion targets attributed to that source — the debuggability twin of a metric cell's source parameter.
    
</dd>
</dl>

<dl>
<dd>

**attribution_model:** `Option<ListEventsRequestAttributionModel>` — Attribution model for the source filter (defaults to last_touch).
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — Country codes to filter by, comma-separated.
    
</dd>
</dl>

<dl>
<dd>

**city:** `Option<String>` — Cities to filter by, comma-separated.
    
</dd>
</dl>

<dl>
<dd>

**device:** `Option<String>` — Device families to filter by, comma-separated (e.g. iPhone, Mac).
    
</dd>
</dl>

<dl>
<dd>

**browser:** `Option<String>` — Browser families to filter by, comma-separated (e.g. Chrome, Mobile Safari).
    
</dd>
</dl>

<dl>
<dd>

**os:** `Option<String>` — Operating system families to filter by, comma-separated (e.g. iOS, Windows).
    
</dd>
</dl>

<dl>
<dd>

**utm_source:** `Option<String>` — utm_source values to filter by, comma-separated.
    
</dd>
</dl>

<dl>
<dd>

**hostname:** `Option<String>` — Page hostnames to filter by, comma-separated.
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<String>` — Page paths to filter by, comma-separated.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">create</a>(request: CreateEventsRequest) -> Result&lt;CreateEventsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Tracks a conversion or engagement event for an account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .events
        .create(
            &CreateEventsRequest {
                account_id: "biz_xxxxxxxxxxxxxx".to_string(),
                event_name: "coating_deposit_paid".to_string(),
                action_source: None,
                app_build_id: None,
                app_id: None,
                context: None,
                currency: None,
                custom_name: None,
                duration: None,
                event_id: None,
                event_time: None,
                plan_id: None,
                product_id: None,
                referrer_url: None,
                resumed: None,
                source: None,
                title: None,
                url: None,
                user: None,
                value: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — The account to associate with this event.
    
</dd>
</dl>

<dl>
<dd>

**action_source:** `Option<Option<CreateEventsRequestActionSource>>` — Where the event originated.
    
</dd>
</dl>

<dl>
<dd>

**app_build_id:** `Option<Option<String>>` — The build of the hosted app that served the page where the event occurred.
    
</dd>
</dl>

<dl>
<dd>

**app_id:** `Option<Option<String>>` — The hosted app that served the page where the event occurred.
    
</dd>
</dl>

<dl>
<dd>

**context:** `Option<Option<CreateEventsRequestContext>>` — Tracking and attribution context.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<Option<CreateEventsRequestCurrency>>` — ISO 4217 currency code.
    
</dd>
</dl>

<dl>
<dd>

**custom_name:** `Option<Option<String>>` — Custom event name when event_name is 'custom'. Maximum 35 chars for this value.
    
</dd>
</dl>

<dl>
<dd>

**duration:** `Option<Option<i64>>` — For 'leave' events: milliseconds the visitor spent on the page.
    
</dd>
</dl>

<dl>
<dd>

**event_id:** `Option<Option<String>>` — Client-provided identifier for deduplication. Generated if omitted.
    
</dd>
</dl>

<dl>
<dd>

**event_name:** `String` 

The type of event.

Use a standard event (lead, submit_application, contact, complete_registration, schedule, view_content, add_to_cart) or pass your own name directly for a custom event.
    
</dd>
</dl>

<dl>
<dd>

**event_time:** `Option<Option<String>>` — When the event occurred. Defaults to now.
    
</dd>
</dl>

<dl>
<dd>

**plan_id:** `Option<Option<String>>` — The plan associated with the event.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<Option<String>>` — The product associated with the event.
    
</dd>
</dl>

<dl>
<dd>

**referrer_url:** `Option<Option<String>>` — The referring URL.
    
</dd>
</dl>

<dl>
<dd>

**resumed:** `Option<Option<bool>>` — For 'page' events: true when the page was restored from the back/forward cache.
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<Option<String>>` — For 'identify' events: where the identity was captured (url, form, manual, iframe).
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — For 'page' events: the document title.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<Option<String>>` — The URL where the event occurred.
    
</dd>
</dl>

<dl>
<dd>

**user:** `Option<Option<CreateEventsRequestUser>>` — User identity and profile data.
    
</dd>
</dl>

<dl>
<dd>

**value:** `Option<Option<f64>>` — Monetary value associated with the event.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">pulse</a>(event: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;PulseEventsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a fully anonymized feed of recent platform-wide money movement, most recent first: purchases, affiliate commissions, card and ad spend, app revenue, off-platform sales, wallet deposits, card loads, claimed drops, transfers between accounts, and referral bonuses. Items carry only a `type`, the underlying event name, a USD amount, a coarse location under `user`, and a timestamp coarsened to the start of the minute; missing fields are omitted, not nulled. The payload is identical for every caller; no auth is required.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .events
        .pulse(
            &PulseQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**event:** `Option<String>` — Filter to one or more types, comma separated — for example `purchase,card_spend`. These are the item's `type`, not its `event_name`: several types share the `ledger_line.created` event name. Omit for every type in the feed. Values outside the feed's own set are rejected.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of events to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor for fetching events after a previous page.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor for fetching events before a later page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.events.<a href="/src/api/resources/events/client.rs">validate_pixel</a>(request: ValidatePixelEventsRequest) -> Result&lt;PixelValidation, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Checks whether the Whop pixel is installed for an account. Recent pixel events count as proof on their own, so an account that has sent data lately comes back installed without a `url`. Pass a `url` and events from that page settle it; conversion events are also read across the hostname because they commonly fire on a later confirmation page. If the requested page hasn't sent any events lately, it is fetched and read for the pixel and conversion events wired on it. `installed` is only true when the pixel was actually seen — in the account's events or in the page.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .events
        .validate_pixel(
            &ValidatePixelEventsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account to check. Defaults to the authenticated account.
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — A page to read for the pixel, e.g. an ad destination. Omit it to check the account from its events alone.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Experiences
<details><summary><code>client.experiences.<a href="/src/api/resources/experiences/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;String&gt;, product_id: Option&lt;Option&lt;String&gt;&gt;, app_id: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListExperiencesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of experiences belonging to a company, with optional filtering by product and app.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .experiences
        .list(
            &ExperiencesListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                product_id: Some("prod_xxxxxxxxxxxxx".to_string()),
                app_id: Some("app_xxxxxxxxxxxxxx".to_string()),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                after: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to list experiences for.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<String>` — Filter to only experiences attached to this product identifier.
    
</dd>
</dl>

<dl>
<dd>

**app_id:** `Option<String>` — Filter to only experiences powered by this app identifier.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return experiences created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return experiences created after this timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.experiences.<a href="/src/api/resources/experiences/client.rs">create</a>(request: CreateExperiencesRequest) -> Result&lt;Experience, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Required permissions:
 - `experience:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .experiences
        .create(
            &CreateExperiencesRequest {
                app_id: "app_xxxxxxxxxxxxxx".to_string(),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                is_public: None,
                logo: None,
                name: None,
                notifications_enabled: None,
                section_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**app_id:** `String` — The unique identifier of the app that powers this experience.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to create this experience for.
    
</dd>
</dl>

<dl>
<dd>

**is_public:** `Option<Option<bool>>` — Whether the experience is publicly accessible without a membership.
    
</dd>
</dl>

<dl>
<dd>

**logo:** `Option<Option<CreateExperiencesRequestLogo>>` — A logo image displayed alongside the experience name.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>` — The display name of the experience. Defaults to the app's name if not provided.
    
</dd>
</dl>

<dl>
<dd>

**notifications_enabled:** `Option<Option<bool>>` — Whether Whop app notifications are enabled for this experience. Webhooks still fire.
    
</dd>
</dl>

<dl>
<dd>

**section_id:** `Option<Option<String>>` — The unique identifier of the section to place the experience in.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.experiences.<a href="/src/api/resources/experiences/client.rs">retrieve</a>(id: String) -> Result&lt;Experience, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing experience.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .experiences
        .retrieve(&"exp_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the experience.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.experiences.<a href="/src/api/resources/experiences/client.rs">delete</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Required permissions:
 - `experience:delete`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .experiences
        .delete(&"exp_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the experience to delete.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.experiences.<a href="/src/api/resources/experiences/client.rs">update</a>(id: String, request: UpdateExperiencesRequest) -> Result&lt;Experience, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Required permissions:
 - `experience:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .experiences
        .update(
            &"exp_xxxxxxxxxxxxxx".to_string(),
            &UpdateExperiencesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the experience to update.
    
</dd>
</dl>

<dl>
<dd>

**access_level:** `Option<Option<ExperienceAccessLevels>>` — The access level of the experience.
    
</dd>
</dl>

<dl>
<dd>

**is_public:** `Option<Option<bool>>` — Whether the experience is publicly accessible without a membership.
    
</dd>
</dl>

<dl>
<dd>

**logo:** `Option<Option<UpdateExperiencesRequestLogo>>` — A logo image displayed alongside the experience name.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>` — The display name of the experience.
    
</dd>
</dl>

<dl>
<dd>

**notifications_enabled:** `Option<Option<bool>>` — Whether Whop app notifications are enabled for this experience. Webhooks still fire.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<Option<String>>` — The position of the experience within its section for display ordering.
    
</dd>
</dl>

<dl>
<dd>

**section_id:** `Option<Option<String>>` — The unique identifier of the section to move the experience into.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.experiences.<a href="/src/api/resources/experiences/client.rs">attach</a>(id: String, request: AttachExperiencesRequest) -> Result&lt;Experience, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Attach an experience to a product, making it accessible to the product's customers.

Required permissions:
 - `experience:attach`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .experiences
        .attach(
            &"exp_xxxxxxxxxxxxxx".to_string(),
            &AttachExperiencesRequest {
                product_id: "prod_xxxxxxxxxxxxx".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the experience to attach.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The unique identifier of the product to attach the experience to.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.experiences.<a href="/src/api/resources/experiences/client.rs">detach</a>(id: String, request: DetachExperiencesRequest) -> Result&lt;Experience, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Detach an experience from a product, removing customer access to it through that product.

Required permissions:
 - `experience:detach`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .experiences
        .detach(
            &"exp_xxxxxxxxxxxxxx".to_string(),
            &DetachExperiencesRequest {
                product_id: "prod_xxxxxxxxxxxxx".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the experience to detach.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The unique identifier of the product to detach the experience from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.experiences.<a href="/src/api/resources/experiences/client.rs">duplicate</a>(id: String, request: DuplicateExperiencesRequest) -> Result&lt;Experience, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Duplicates an existing experience. The name will be copied, unless provided. The new experience will be attached to the same products as the original experience.
If duplicating a Forum or Chat experience, the new experience will have the same settings as the original experience, e.g. who can post, who can comment, etc.
No content, e.g. posts, messages, lessons from within the original experience will be copied.


Required permissions:
 - `experience:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .experiences
        .duplicate(
            &"exp_xxxxxxxxxxxxxx".to_string(),
            &DuplicateExperiencesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the experience to duplicate.
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<Option<String>>` — The display name for the duplicated experience. Defaults to the original experience's name.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Exports
<details><summary><code>client.exports.<a href="/src/api/resources/exports/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, resource: Option&lt;Option&lt;ListExportsRequestResource&gt;&gt;, status: Option&lt;Option&lt;ListExportsRequestStatus&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListExportsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListExportsRequestDirection&gt;&gt;) -> Result&lt;ListExportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the exports requested for an account, newest first. Only exports of resources the credential is allowed to export are returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .exports
        .list(
            &ExportsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The account to list exports for, prefixed `biz_`. Defaults to the credential's account.
    
</dd>
</dl>

<dl>
<dd>

**resource:** `Option<ListExportsRequestResource>` — Only return exports of this resource.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListExportsRequestStatus>` — Only return exports in this status.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return exports created at or after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return exports created at or before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListExportsRequestOrder>` — The field to sort by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListExportsRequestDirection>` — The sort direction.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.exports.<a href="/src/api/resources/exports/client.rs">create</a>(request: CreateExportsRequest) -> Result&lt;Export, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Starts an asynchronous export of a resource for an account. Returns the export in `pending`; poll `GET /exports/{id}` until `download_url` is set.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .exports
        .create(
            &CreateExportsRequest {
                resource: CreateExportsRequestResource::AdCampaigns,
                account_id: None,
                columns: None,
                filters: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The account to export from, prefixed `biz_`. Defaults to the credential's account.
    
</dd>
</dl>

<dl>
<dd>

**columns:** `Option<Vec<String>>` — Column keys to include. Empty means all columns for the resource.
    
</dd>
</dl>

<dl>
<dd>

**filters:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Resource-specific filters. For native REST resources (`payouts`, `transfers`, `products`) these are the resource's own list query params; for dashboard tables they mirror the dashboard table filters.
    
</dd>
</dl>

<dl>
<dd>

**resource:** `CreateExportsRequestResource` — The resource to export, e.g. `payouts`, `receipts`, or `members`.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — IANA timezone for date columns, e.g. `America/New_York`. Defaults to `UTC`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.exports.<a href="/src/api/resources/exports/client.rs">retrieve</a>(id: String) -> Result&lt;Export, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetches an export's status and, once complete, its download link.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.exports.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The export ID, prefixed `exprt_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## FeeMarkups
<details><summary><code>client.fee_markups.<a href="/src/api/resources/fee_markups/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;String&gt;) -> Result&lt;ListFeeMarkupsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of fee markups configured for a company. If the company is a platform account, returns the platform default markups.

Required permissions:
 - `company:update_child_fees`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .fee_markups
        .list(
            &FeeMarkupsListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                after: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to list fee markups for. Pass a platform account identifier to retrieve platform default markups.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.fee_markups.<a href="/src/api/resources/fee_markups/client.rs">create</a>(request: CreateFeeMarkupsRequest) -> Result&lt;FeeMarkup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create or update a fee markup for a company. If a markup for the specified fee type already exists, it will be updated with the new values.

Required permissions:
 - `company:update_child_fees`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .fee_markups
        .create(
            &CreateFeeMarkupsRequest {
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                fee_type: FeeMarkupTypes::CryptoWithdrawalMarkup,
                fixed_fee_usd: None,
                metadata: None,
                notes: None,
                percentage_fee: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to create or update the fee markup for.
    
</dd>
</dl>

<dl>
<dd>

**fee_type:** `FeeMarkupTypes` — The type of fee this markup applies to, such as processing or platform fees.
    
</dd>
</dl>

<dl>
<dd>

**fixed_fee_usd:** `Option<Option<f64>>` — The fixed fee amount in USD to charge per transaction. Must be between 0 and 50.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Custom key-value metadata to attach to this fee markup.
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` — Internal notes about this fee markup for record-keeping purposes.
    
</dd>
</dl>

<dl>
<dd>

**percentage_fee:** `Option<Option<f64>>` — The percentage fee to charge per transaction. Must be between 0 and 25.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.fee_markups.<a href="/src/api/resources/fee_markups/client.rs">delete</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a fee markup configuration for a company. This removes the custom fee override and reverts to the parent company's default fees.

Required permissions:
 - `company:update_child_fees`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.fee_markups.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the fee markup to delete.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Files
<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">list</a>(order: Option&lt;Option&lt;ListFilesRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListFilesRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListFilesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the files with the given IDs, newest first — fetch a batch in one request instead of retrieving each file individually. Only files you created are returned; IDs that do not exist, or that another credential created, are omitted. A request for up to 100 IDs answers in a single page by default; a larger batch pages at up to 100 files per response — follow `page_info` with the same `file_ids` to walk the rest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .files
        .list(
            &FilesListQueryRequest {
                file_ids: vec![Some("file_xxxxxxxxxxxxx".to_string())],
                order: None,
                direction: None,
                first: None,
                after: None,
                last: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**file_ids:** `Option<String>` — The files to return, each prefixed `file_`. Repeat the parameter to pass several, up to 250 per request. Batches of up to 100 answer in one page by default; larger batches page at up to 100 per response.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListFilesRequestOrder>` — The field to sort by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListFilesRequestDirection>` — The sort direction.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of files to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns files after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of files to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns files before this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">create</a>(request: CreateFilesRequest) -> Result&lt;File, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a file and returns a presigned destination to upload its bytes to. PUT the bytes to `upload_url` (single-part), or to each of `multipart_upload_urls` and then call Complete File Multipart Upload. Once the bytes land the file becomes `ready`, and its ID can be attached wherever a file is accepted — account legal documents, dispute evidence documents.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .files
        .create(
            &CreateFilesRequest {
                filename: "terms.pdf".to_string(),
                byte_size: None,
                multipart: None,
                visibility: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**byte_size:** `Option<i64>` — The file's size in bytes. Required when `multipart` is `true`. Multipart uploads support at most 10,000 parts of 5MB each (about 50 GB).
    
</dd>
</dl>

<dl>
<dd>

**filename:** `String` — The name of the file including its extension, e.g. `terms.pdf`.
    
</dd>
</dl>

<dl>
<dd>

**multipart:** `Option<bool>` — Upload the file in 5MB parts. Required for files larger than 5GB; useful above ~100MB. The file must be larger than 5MB.
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<CreateFilesRequestVisibility>` — `public` files are served via an unsigned CDN URL — use for assets anyone may see. `private` files are served via a signed, expiring URL — use for sensitive documents. Defaults to `private`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">retrieve</a>(id: String) -> Result&lt;File, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a file you uploaded — poll it after uploading the bytes to see `upload_status` become `ready`. Only the creator can retrieve a file this way; a file attached to another resource is read through that resource.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.files.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the file, prefixed `file_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.files.<a href="/src/api/resources/files/client.rs">complete</a>(id: String, request: CompleteFilesRequest) -> Result&lt;File, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Assembles the parts of a multipart upload after every part has been PUT to its presigned URL. Pass the `multipart_upload_id` from Create File and each part's `ETag` response header.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .files
        .complete(
            &"id".to_string(),
            &CompleteFilesRequest {
                multipart_parts: vec![CompleteFilesRequestMultipartPartsItem {
                    etag: "etag-1".to_string(),
                    part_number: 1,
                    ..Default::default()
                }],
                multipart_upload_id: "upload-id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the file, prefixed `file_`.
    
</dd>
</dl>

<dl>
<dd>

**multipart_parts:** `Vec<CompleteFilesRequestMultipartPartsItem>` — Every uploaded part, in order.
    
</dd>
</dl>

<dl>
<dd>

**multipart_upload_id:** `String` — The ID of the multipart upload, returned by Create File.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## FinancialActivity
<details><summary><code>client.financial_activity.<a href="/src/api/resources/financial_activity/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, include_owned_accounts: Option&lt;Option&lt;bool&gt;&gt;, include_resource: Option&lt;Option&lt;bool&gt;&gt;, direction: Option&lt;Option&lt;ListFinancialActivityRequestDirection&gt;&gt;, currency: Option&lt;Option&lt;String&gt;&gt;, posted_after: Option&lt;Option&lt;String&gt;&gt;, posted_before: Option&lt;Option&lt;String&gt;&gt;, available_after: Option&lt;Option&lt;String&gt;&gt;, available_before: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListFinancialActivityResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns an account's or user's activity feed: every movement of money in or out.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .financial_activity
        .list(
            &FinancialActivityListQueryRequest {
                account_id: None,
                user_id: None,
                include_owned_accounts: None,
                include_resource: None,
                line_types: vec![],
                direction: None,
                currency: None,
                posted_after: None,
                posted_before: None,
                available_after: None,
                available_before: None,
                limit: None,
                cursor: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The owning account ID (a biz_ identifier). Provide this or user_id.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The owning user ID (a user_ identifier). Provide this or account_id.
    
</dd>
</dl>

<dl>
<dd>

**include_owned_accounts:** `Option<bool>` — When true, aggregates the authenticated user's personal ledger with the businesses they own (owner role with balance read) into one feed. Requires user_id to be the authenticated user; cannot be combined with account_id or the settlement-date filters. Each returned row includes the owning `account`.
    
</dd>
</dl>

<dl>
<dd>

**include_resource:** `Option<bool>` — Whether to include the `resource` field in the response or not. Consider passing `false` if you need a fast response without as many rich details.
    
</dd>
</dl>

<dl>
<dd>

**line_types:** `Option<ListFinancialActivityRequestLineTypesItem>` — Optional ledger line categories to include. Some categories (for example `onchain_deposit`, which covers inbound crypto deposits such as MoonPay onramps) are only returned when explicitly requested here.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListFinancialActivityRequestDirection>` — Optional direction filter. `money_in` returns positive activity and `money_out` returns negative activity.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — Optional currency code filter, for example `usd`.
    
</dd>
</dl>

<dl>
<dd>

**posted_after:** `Option<String>` — Only include rows posted after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**posted_before:** `Option<String>` — Only include rows posted before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**available_after:** `Option<String>` — Only include rows whose funds became withdrawable on or after this `YYYY-MM-DD` settlement date (UTC), distinct from posted_at. Requires currency.
    
</dd>
</dl>

<dl>
<dd>

**available_before:** `Option<String>` — Only include rows whose funds became withdrawable on or before this `YYYY-MM-DD` settlement date (UTC). Set equal to available_after for a single day. Requires currency.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Maximum number of rows to return.
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Cursor returned by the previous page.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## FinancialReports
<details><summary><code>client.financial_reports.<a href="/src/api/resources/financial_reports/client.rs">retrieve</a>(account_id: Option&lt;String&gt;, report_type: Option&lt;RetrieveFinancialReportsRequestReportType&gt;, currency: Option&lt;Option&lt;String&gt;&gt;, in_currency: Option&lt;Option&lt;String&gt;&gt;, from_date: Option&lt;Option&lt;String&gt;&gt;, to_date: Option&lt;Option&lt;String&gt;&gt;, group_by: Option&lt;Option&lt;RetrieveFinancialReportsRequestGroupBy&gt;&gt;, timezone: Option&lt;Option&lt;String&gt;&gt;, direction: Option&lt;Option&lt;RetrieveFinancialReportsRequestDirection&gt;&gt;, cumulative: Option&lt;Option&lt;bool&gt;&gt;, scope_account_id: Option&lt;Option&lt;String&gt;&gt;, include_payment_fee_breakdown: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;RetrieveFinancialReportsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a financial report — balance activity, income statement, or balance summary — for an account over a date range.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .financial_reports
        .retrieve(
            &FinancialReportsRetrieveQueryRequest {
                account_id: "account_id".to_string(),
                report_type: RetrieveFinancialReportsRequestReportType::BalanceSummary,
                currency: None,
                in_currency: None,
                from_date: None,
                to_date: None,
                group_by: None,
                timezone: None,
                line_types: vec![],
                direction: None,
                cumulative: None,
                scope_account_id: None,
                include_payment_fee_breakdown: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — The owning account ID (a biz_ identifier), or `global` for a platform-wide report across all ledger accounts (requires internal admin access).
    
</dd>
</dl>

<dl>
<dd>

**report_type:** `RetrieveFinancialReportsRequestReportType` — The type of financial report to generate.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — Filter rows to this currency, for example `usd`. Defaults to `usd` unless `in_currency` is provided.
    
</dd>
</dl>

<dl>
<dd>

**in_currency:** `Option<String>` — Aggregate all activity into this display currency via FX conversion.
    
</dd>
</dl>

<dl>
<dd>

**from_date:** `Option<String>` — Start of the report window as an ISO 8601 timestamp (UTC). Required for platform-wide (global) reports.
    
</dd>
</dl>

<dl>
<dd>

**to_date:** `Option<String>` — End of the report window as an ISO 8601 timestamp (UTC). Required for platform-wide (global) reports.
    
</dd>
</dl>

<dl>
<dd>

**group_by:** `Option<RetrieveFinancialReportsRequestGroupBy>` — Grouping granularity for report rows.
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<String>` — IANA timezone (for example `America/New_York`) used to bucket report periods and to interpret calendar-day boundaries for balance snapshots. Defaults to UTC. from_date/to_date remain exact instants regardless of this setting.
    
</dd>
</dl>

<dl>
<dd>

**line_types:** `Option<RetrieveFinancialReportsRequestLineTypesItem>` — Account-level balance activity only: ledger line categories to include.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<RetrieveFinancialReportsRequestDirection>` — Account-level balance activity only: include money moving in or money moving out.
    
</dd>
</dl>

<dl>
<dd>

**cumulative:** `Option<bool>` — Platform-wide (global) reports only: when true, return cumulative balances as of to_date (all history, no lower bound) instead of activity within the period.
    
</dd>
</dl>

<dl>
<dd>

**scope_account_id:** `Option<String>` — Platform-wide (global) reports only: narrow the report to ledger lines on the ledger account owned by this account ID (a biz_ identifier). Ignored unless account_id is `global`.
    
</dd>
</dl>

<dl>
<dd>

**include_payment_fee_breakdown:** `Option<bool>` — Balance activity only: include payment costs grouped by payment method and provider.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ForumPosts
<details><summary><code>client.forum_posts.<a href="/src/api/resources/forum_posts/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, experience_id: Option&lt;String&gt;, include_bounty_anchors: Option&lt;Option&lt;bool&gt;&gt;, parent_id: Option&lt;Option&lt;String&gt;&gt;, pinned: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListForumPostsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of forum posts within a specific experience, with optional filtering by parent post or pinned status.

Required permissions:
 - `forum:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .forum_posts
        .list(
            &ForumPostsListQueryRequest {
                first: Some(42),
                last: Some(42),
                experience_id: "exp_xxxxxxxxxxxxxx".to_string(),
                after: None,
                before: None,
                include_bounty_anchors: None,
                parent_id: None,
                pinned: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**experience_id:** `String` — The unique identifier of the experience to list forum posts for.
    
</dd>
</dl>

<dl>
<dd>

**include_bounty_anchors:** `Option<bool>` — Whether to include top-level bounty discussion anchors as rich forum items.
    
</dd>
</dl>

<dl>
<dd>

**parent_id:** `Option<String>` — The unique identifier of a parent post to list comments for. When set, returns replies to that post.
    
</dd>
</dl>

<dl>
<dd>

**pinned:** `Option<bool>` — Whether to filter for only pinned posts. Set to true to return only pinned posts.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.forum_posts.<a href="/src/api/resources/forum_posts/client.rs">create</a>(request: CreateForumPostsRequest) -> Result&lt;ForumPost, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new forum post or comment within an experience. Supports text content, attachments, polls, paywalling, and pinning. Pass experience_id 'public' with a company_id to post to a company's public forum.

Required permissions:
 - `forum:post:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .forum_posts
        .create(
            &CreateForumPostsRequest {
                experience_id: "exp_xxxxxxxxxxxxxx".to_string(),
                attachments: None,
                company_id: None,
                content: None,
                is_mention: None,
                parent_id: None,
                paywall_amount: None,
                paywall_currency: None,
                pinned: None,
                poll: None,
                rich_content: None,
                title: None,
                visibility: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attachments:** `Option<Option<Vec<CreateForumPostsRequestAttachmentsItem>>>` — A list of file attachments to include with the post, such as images or videos.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<Option<String>>` — The unique identifier of the company whose public forum to post in. Required when experience_id is 'public'. For example, 'biz_xxxxx'.
    
</dd>
</dl>

<dl>
<dd>

**content:** `Option<Option<String>>` — The main body of the post in Markdown format. For example, 'Check out this **update**'. Hidden if the post is paywalled and the viewer has not purchased access.
    
</dd>
</dl>

<dl>
<dd>

**experience_id:** `String` — The unique identifier of the experience to create this post in. For example, 'exp_xxxxx'. Pass 'public' along with company_id to automatically use the company's public forum.
    
</dd>
</dl>

<dl>
<dd>

**is_mention:** `Option<Option<bool>>` — Whether to send this post as a mention notification to all users in the experience who have mentions enabled.
    
</dd>
</dl>

<dl>
<dd>

**parent_id:** `Option<Option<String>>` — The unique identifier of the parent post to comment on. Omit this field to create a top-level post.
    
</dd>
</dl>

<dl>
<dd>

**paywall_amount:** `Option<Option<f64>>` — The price to unlock this post in the specified paywall currency. For example, 5.00 for $5.00. When set, users must purchase access to view the post content.
    
</dd>
</dl>

<dl>
<dd>

**paywall_currency:** `Option<Option<Currencies>>` — The currency for the paywall price on this post. When set along with paywall_amount, users must purchase access to view the post content.
    
</dd>
</dl>

<dl>
<dd>

**pinned:** `Option<Option<bool>>` — Whether this post should be pinned to the top of the forum.
    
</dd>
</dl>

<dl>
<dd>

**poll:** `Option<Option<CreateForumPostsRequestPoll>>` — A poll to attach to this post, allowing members to vote on options.
    
</dd>
</dl>

<dl>
<dd>

**rich_content:** `Option<Option<String>>` — The rich content of the post in Tiptap JSON format. When provided, takes priority over the markdown content field for rendering.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The title of the post, displayed prominently at the top. Required for paywalled posts as it remains visible to non-purchasers.
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<Option<ForumPostVisibilityTypes>>` — Controls who can see this forum post, such as members only or public.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.forum_posts.<a href="/src/api/resources/forum_posts/client.rs">retrieve</a>(id: String) -> Result&lt;ForumPost, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing forum post.

Required permissions:
 - `forum:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.forum_posts.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the forum post to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.forum_posts.<a href="/src/api/resources/forum_posts/client.rs">update</a>(id: String, request: UpdateForumPostsRequest) -> Result&lt;ForumPost, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Edit the content, attachments, pinned status, or visibility of an existing forum post or comment.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .forum_posts
        .update(
            &"id".to_string(),
            &UpdateForumPostsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the forum post to update.
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Option<Vec<UpdateForumPostsRequestAttachmentsItem>>>` — A replacement list of file attachments for this post, such as images or videos.
    
</dd>
</dl>

<dl>
<dd>

**content:** `Option<Option<String>>` — The updated body of the post in Markdown format. For example, 'Check out this **update**'. Hidden if the post is paywalled and the viewer has not purchased access.
    
</dd>
</dl>

<dl>
<dd>

**is_pinned:** `Option<Option<bool>>` — Whether this post should be pinned to the top of the forum. Only top-level posts can be pinned, not comments.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The updated title of the post, displayed prominently at the top. Required for paywalled posts as it remains visible to non-purchasers.
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<Option<ForumPostVisibilityTypes>>` — Controls who can see this forum post, such as members only or public.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Forums
<details><summary><code>client.forums.<a href="/src/api/resources/forums/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;String&gt;, product_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListForumsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of forums within a specific company, with optional filtering by product.

Required permissions:
 - `forum:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .forums
        .list(
            &ForumsListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                product_id: Some("prod_xxxxxxxxxxxxx".to_string()),
                after: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to list forums for.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<String>` — The unique identifier of a product to filter by. When set, only forums connected to this product are returned.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.forums.<a href="/src/api/resources/forums/client.rs">retrieve</a>(id: String) -> Result&lt;Forum, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing forum.

Required permissions:
 - `forum:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.forums.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the forum or experience to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.forums.<a href="/src/api/resources/forums/client.rs">update</a>(id: String, request: UpdateForumsRequest) -> Result&lt;Forum, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update moderation and notification settings for a forum, such as who can post, who can comment, and email notification preferences.

Required permissions:
 - `forum:moderate`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .forums
        .update(
            &"id".to_string(),
            &UpdateForumsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the forum to update. Accepts either an experience ID (e.g. 'exp_xxxxx') or a forum ID.
    
</dd>
</dl>

<dl>
<dd>

**banned_words:** `Option<Option<Vec<String>>>` — A list of words that are automatically blocked from posts in this forum. For example, ['spam', 'scam'].
    
</dd>
</dl>

<dl>
<dd>

**banner_image:** `Option<Option<UpdateForumsRequestBannerImage>>` — The banner image displayed at the top of the forum page. Pass null to remove the existing banner.
    
</dd>
</dl>

<dl>
<dd>

**email_notification_preference:** `Option<Option<ForumEmailNotificationPreferences>>` — Controls how email notifications are sent to members when new posts are created in this forum.
    
</dd>
</dl>

<dl>
<dd>

**who_can_comment:** `Option<Option<ForumWhoCanCommentTypes>>` — Controls which roles are allowed to comment on posts in this forum.
    
</dd>
</dl>

<dl>
<dd>

**who_can_post:** `Option<Option<ForumWhoCanPostTypes>>` — Controls which roles are allowed to create new posts in this forum.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## IdentityProfiles
<details><summary><code>client.identity_profiles.<a href="/src/api/resources/identity_profiles/client.rs">list_identity_profile</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, profile_type: Option&lt;Option&lt;IdentityProfileKinds&gt;&gt;, status: Option&lt;Option&lt;IdentityProfileStatuses&gt;&gt;) -> Result&lt;ListIdentityProfileResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of identity profiles. When company_id is provided, lists IPs currently linked to that company's ledger. When omitted, lists IPs linked to any ledger the actor can read (including child companies under a parent).

Required permissions:
 - `identity:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .identity_profiles
        .list_identity_profile(
            &ListIdentityProfileQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of the company to filter to. When omitted, returns IPs across all ledgers the actor can read.
    
</dd>
</dl>

<dl>
<dd>

**profile_type:** `Option<IdentityProfileKinds>` 
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<IdentityProfileStatuses>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.identity_profiles.<a href="/src/api/resources/identity_profiles/client.rs">retrieve_identity_profile</a>(id: String) -> Result&lt;IdentityProfile, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing identity profile.

Required permissions:
 - `identity:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .identity_profiles
        .retrieve_identity_profile(&"idpf_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the identity profile (idpf_xxx).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.identity_profiles.<a href="/src/api/resources/identity_profiles/client.rs">unlink_identity_profile</a>(id: String, ledger_account_id: Option&lt;String&gt;) -> Result&lt;IdentityProfile, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Unlinks an IdentityProfile from a LedgerAccount (flips the matching link to is_current=false).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .identity_profiles
        .unlink_identity_profile(
            &"idpf_xxxxxxxxxxxxx".to_string(),
            &UnlinkIdentityProfileQueryRequest {
                ledger_account_id: "ldgr_xxxxxxxxxxxxx".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the IdentityProfile to unlink.
    
</dd>
</dl>

<dl>
<dd>

**ledger_account_id:** `String` — The ID of the LedgerAccount to unlink the identity profile from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.identity_profiles.<a href="/src/api/resources/identity_profiles/client.rs">list_verifications_identity_profile</a>(id: String, after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListVerificationsIdentityProfileResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of verifications attached to an identity profile, ordered by most recent first.

Required permissions:
 - `identity:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .identity_profiles
        .list_verifications_identity_profile(
            &"idpf_xxxxxxxxxxxxx".to_string(),
            &ListVerificationsIdentityProfileQueryRequest {
                first: Some(42),
                last: Some(42),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the identity profile (idpf_xxx).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Invoices
<details><summary><code>client.invoices.<a href="/src/api/resources/invoices/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, direction: Option&lt;Option&lt;Direction&gt;&gt;, order: Option&lt;Option&lt;InvoicesSortableColumns&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListInvoicesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of invoices for a company, with optional filtering by product, status, collection method, and creation date.

Required permissions:
 - `invoice:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .invoices
        .list(
            &InvoicesListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                after: None,
                before: None,
                direction: None,
                product_ids: vec![],
                collection_methods: vec![],
                statuses: vec![],
                order: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of the company to list invoices for.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<Direction>` 
    
</dd>
</dl>

<dl>
<dd>

**product_ids:** `Option<String>` — Filter invoices to only those associated with these specific product identifiers.
    
</dd>
</dl>

<dl>
<dd>

**collection_methods:** `Option<InvoiceCollectionMethods>` — Filter invoices by their collection method.
    
</dd>
</dl>

<dl>
<dd>

**statuses:** `Option<InvoiceStatuses>` — Filter invoices by their current status.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<InvoicesSortableColumns>` 
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return invoices created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return invoices created after this timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoices.<a href="/src/api/resources/invoices/client.rs">create</a>(request: CreateInvoicesRequestBody) -> Result&lt;Invoice, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create an invoice for a customer. The invoice can be charged automatically using a stored payment method, or sent to the customer for manual payment.

Required permissions:
 - `invoice:create`
 - `member:email:read`
 - `member:basic:read`
 - `payment:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .invoices
        .create(
            &CreateInvoicesRequestBody::CreateInvoicesRequestBodyProduct(
                CreateInvoicesRequestBodyProduct {
                    automatically_finalizes_at: None,
                    billing_address: None,
                    charge_buyer_fee: None,
                    collection_method: InvoiceCollectionMethods::SendInvoice,
                    company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                    customer_name: None,
                    due_date: None,
                    email_address: None,
                    line_items: None,
                    mailing_address_id: None,
                    member_id: None,
                    payment_method_id: None,
                    payment_token_id: None,
                    plan: CreateInvoicesRequestBodyProductPlan {
                        ..Default::default()
                    },
                    product: CreateInvoicesRequestBodyProductProduct {
                        title: "title".to_string(),
                        ..Default::default()
                    },
                    save_as_draft: None,
                    subscription_billing_anchor_at: None,
                },
            ),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoices.<a href="/src/api/resources/invoices/client.rs">retrieve</a>(id: String) -> Result&lt;Invoice, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing invoice.

Required permissions:
 - `invoice:basic:read`
 - `member:email:read`
 - `member:basic:read`
 - `payment:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .invoices
        .retrieve(&"inv_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the invoice, or a secure token.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoices.<a href="/src/api/resources/invoices/client.rs">delete</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a draft invoice.

Required permissions:
 - `invoice:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .invoices
        .delete(&"inv_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the draft invoice to delete.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoices.<a href="/src/api/resources/invoices/client.rs">update</a>(id: String, request: UpdateInvoicesRequest) -> Result&lt;Invoice, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a draft invoice's details.

Required permissions:
 - `invoice:update`
 - `member:email:read`
 - `member:basic:read`
 - `payment:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .invoices
        .update(
            &"inv_xxxxxxxxxxxxxx".to_string(),
            &UpdateInvoicesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the invoice to update.
    
</dd>
</dl>

<dl>
<dd>

**automatically_finalizes_at:** `Option<Option<String>>` — The date and time when the invoice will be automatically finalized. For charge_automatically, triggers an automatic charge. For send_invoice, sends the invoice email at the specified time.
    
</dd>
</dl>

<dl>
<dd>

**billing_address:** `Option<Option<UpdateInvoicesRequestBillingAddress>>` — Inline billing address to create or update a mailing address for this invoice.
    
</dd>
</dl>

<dl>
<dd>

**charge_buyer_fee:** `Option<Option<bool>>` — Whether to charge the customer a buyer fee on this invoice.
    
</dd>
</dl>

<dl>
<dd>

**collection_method:** `Option<Option<InvoiceCollectionMethods>>` — How the invoice should be collected.
    
</dd>
</dl>

<dl>
<dd>

**customer_name:** `Option<Option<String>>` — The name of the customer.
    
</dd>
</dl>

<dl>
<dd>

**due_date:** `Option<Option<String>>` — The date by which the invoice must be paid.
    
</dd>
</dl>

<dl>
<dd>

**email_address:** `Option<Option<String>>` — The email address of the customer.
    
</dd>
</dl>

<dl>
<dd>

**line_items:** `Option<Option<Vec<UpdateInvoicesRequestLineItemsItem>>>` — Line items that break down the invoice total. When provided, the sum of (quantity * unit_price) for all items must equal the plan price. Individual items may be negative to represent a credit, as long as the sum is not negative and clears the currency's minimum charge. Pass an empty list to remove the breakdown.
    
</dd>
</dl>

<dl>
<dd>

**mailing_address_id:** `Option<Option<String>>` — The unique identifier of an existing mailing address to attach.
    
</dd>
</dl>

<dl>
<dd>

**member_id:** `Option<Option<String>>` — The unique identifier of a member to assign as the customer.
    
</dd>
</dl>

<dl>
<dd>

**payment_method_id:** `Option<Option<String>>` — The unique identifier of the payment method to charge.
    
</dd>
</dl>

<dl>
<dd>

**plan:** `Option<Option<UpdateInvoicesRequestPlan>>` — Updated plan attributes.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<Option<String>>` — The unique identifier of an existing product to attach to this invoice. Only allowed while the invoice is still a draft.
    
</dd>
</dl>

<dl>
<dd>

**subscription_billing_anchor_at:** `Option<Option<String>>` — The date that defines when the subscription billing cycle should start.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoices.<a href="/src/api/resources/invoices/client.rs">mark_paid</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Mark an open invoice as paid when payment was collected outside of Whop.

Required permissions:
 - `invoice:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .invoices
        .mark_paid(&"inv_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the invoice to mark as paid.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoices.<a href="/src/api/resources/invoices/client.rs">mark_uncollectible</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Mark an open invoice as uncollectible when payment is not expected.

Required permissions:
 - `invoice:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .invoices
        .mark_uncollectible(&"inv_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the invoice to mark as uncollectible.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoices.<a href="/src/api/resources/invoices/client.rs">resend</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Resend the notification email for an existing invoice to the customer.

Required permissions:
 - `invoice:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .invoices
        .resend(&"inv_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the invoice to resend.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoices.<a href="/src/api/resources/invoices/client.rs">void</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Void an open invoice so it can no longer be paid. Voiding is permanent and cannot be undone.

Required permissions:
 - `invoice:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .invoices
        .void(&"inv_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the invoice to void.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Leads
<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;String&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListLeadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of leads for a company, with optional filtering by product and creation date.

Required permissions:
 - `lead:basic:read`
 - `member:email:read`
 - `access_pass:basic:read`
 - `member:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .leads
        .list(
            &LeadsListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                after: None,
                before: None,
                product_ids: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to list leads for.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return leads created after this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return leads created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**product_ids:** `Option<String>` — Filter leads to only those associated with these specific product identifiers.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">create</a>(request: CreateLeadsRequest) -> Result&lt;Lead, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Record a new lead for a company, capturing a potential customer's interest in a specific product.

Required permissions:
 - `lead:manage`
 - `member:email:read`
 - `access_pass:basic:read`
 - `member:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .leads
        .create(
            &CreateLeadsRequest {
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                metadata: None,
                product_id: None,
                referrer: None,
                user_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to create the lead for, starting with 'biz_'.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — A JSON object of custom metadata to attach to the lead for tracking purposes.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<Option<String>>` — The unique identifier of the product the lead is interested in, starting with 'prod_'.
    
</dd>
</dl>

<dl>
<dd>

**referrer:** `Option<Option<String>>` — The referral URL that brought the lead to the company, such as 'https://example.com/landing'.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<Option<String>>` — The unique identifier of the user to record as the lead. If authenticated as a user, that user is used automatically.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">retrieve</a>(id: String) -> Result&lt;Lead, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing lead.

Required permissions:
 - `lead:basic:read`
 - `member:email:read`
 - `access_pass:basic:read`
 - `member:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .leads
        .retrieve(&"lead_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the lead to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.leads.<a href="/src/api/resources/leads/client.rs">update</a>(id: String, request: UpdateLeadsRequest) -> Result&lt;Lead, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the metadata or referrer information on an existing lead record.

Required permissions:
 - `lead:manage`
 - `member:email:read`
 - `access_pass:basic:read`
 - `member:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .leads
        .update(
            &"lead_xxxxxxxxxxxxx".to_string(),
            &UpdateLeadsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the lead to update, starting with 'lead_'.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — A JSON object of custom metadata to set on the lead, replacing any existing metadata.
    
</dd>
</dl>

<dl>
<dd>

**referrer:** `Option<Option<String>>` — The updated referral URL for the lead, such as 'https://example.com/landing'.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## LedgerAccounts
<details><summary><code>client.ledger_accounts.<a href="/src/api/resources/ledger_accounts/client.rs">retrieve</a>(id: String) -> Result&lt;LedgerAccount, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing ledger account.

Required permissions:
 - `company:balance:read`
 - `payout:account:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .ledger_accounts
        .retrieve(&"ldgr_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The identifier to look up the ledger account. Accepts a user ID ('user_xxx'), company ID ('biz_xxx'), or ledger account ID ('ldgr_xxx').
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Media
<details><summary><code>client.media.<a href="/src/api/resources/media/client.rs">generate</a>(request: GenerateMediaRequest) -> Result&lt;MediaAsset, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Starts an AI media generation job billed from the account's balance. Generation is asynchronous — poll `GET /media/{id}` until the asset is `ready`, then use `file.id` anywhere attachments are accepted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .media
        .generate(
            &GenerateMediaRequest {
                prompt: "A 9:16 product showcase of a cordless power scrubber".to_string(),
                r#type: GenerateMediaRequestType::Video,
                account_id: None,
                duration_seconds: None,
                reference_media: None,
                resolution: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account ID, prefixed `biz_`. Defaults to the account the API key belongs to.
    
</dd>
</dl>

<dl>
<dd>

**duration_seconds:** `Option<i64>` — Video length in seconds. Video only; defaults to 5.
    
</dd>
</dl>

<dl>
<dd>

**prompt:** `String` — What to generate. Up to 2,000 characters.
    
</dd>
</dl>

<dl>
<dd>

**reference_media:** `Option<Vec<String>>` — Optional reference image file IDs (`file_` prefixed), up to 4. For video, a single reference seeds the opening frame; multiple references guide subject and style instead.
    
</dd>
</dl>

<dl>
<dd>

**resolution:** `Option<GenerateMediaRequestResolution>` — Video resolution. Video only; defaults to `1080p`. `1080p` is not supported by Seedance 2.0 Fast or Mini; `4k` is only supported by Seedance 2.0.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `GenerateMediaRequestType` — The kind of media to generate.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.media.<a href="/src/api/resources/media/client.rs">retrieve</a>(id: String) -> Result&lt;MediaAsset, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a media asset by ID. Poll this while the asset is `processing`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.media.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Media asset ID, prefixed `media_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Members
<details><summary><code>client.members.<a href="/src/api/resources/members/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, access_level: Option&lt;Option&lt;ListMembersRequestAccessLevel&gt;&gt;, status: Option&lt;Option&lt;ListMembersRequestStatus&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListMembersRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListMembersRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListMembersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the members of an account. A member is one buyer's relationship with the account, regardless of how many memberships they hold.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .members
        .list(
            &MembersListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The account to list members for (`biz_` tag). Defaults to the account the credential acts as.
    
</dd>
</dl>

<dl>
<dd>

**access_level:** `Option<ListMembersRequestAccessLevel>` — Filter by what the member can reach on the account.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListMembersRequestStatus>` — Filter by whether the member is still part of the account.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Search members by name or username. An exact email address also matches when the credential holds the member:email:read scope.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only members who joined after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only members who joined before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListMembersRequestOrder>` — Sort field.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListMembersRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of members to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to paginate forwards from.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of members to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to paginate backwards from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.members.<a href="/src/api/resources/members/client.rs">retrieve</a>(id: String) -> Result&lt;Member, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a member by ID. Accessible to the account and to the member's own user.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.members.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Member ID (`mber_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Memberships
<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListMembershipsRequestStatus&gt;&gt;, product_id: Option&lt;Option&lt;String&gt;&gt;, plan_id: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListMembershipsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListMembershipsRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListMembershipsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists every membership the caller can read: an account API key its account's; a user credential their own plus those of every account they manage. `account_id` and `user_id` only narrow that list — values outside the caller's reach return fewer results, not an error.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .memberships
        .list(
            &MembershipsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Narrow to one account (`biz_` tag). With read access to the account this lists all of its memberships; without, only the caller's own memberships in it.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Narrow to one user's memberships (`user_` tag, or `me` for the caller). A user outside the caller's visible set returns an empty list.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListMembershipsRequestStatus>` — Filter by billing state. `canceling` matches active memberships set to cancel at period end; `paused` matches memberships with payment collection paused.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<String>` — Filter to memberships of this product (`prod_` tag). Repeat as product_ids[] for several.
    
</dd>
</dl>

<dl>
<dd>

**plan_id:** `Option<String>` — Filter to memberships of this plan (`plan_` tag). Repeat as plan_ids[] for several.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only memberships created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only memberships created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListMembershipsRequestOrder>` — Sort field.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListMembershipsRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of memberships to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to paginate forwards from.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of memberships to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to paginate backwards from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">invite</a>(request: InviteMembershipsRequestBody) -> Result&lt;InviteMembershipsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Sends an email inviting one recipient to join the account through a free plan. Identify the recipient by exactly one of `user_id` or `email`. The invitation is bound to that recipient; after signing in, accepting it immediately grants the membership without checkout. This Experimental endpoint is available only to accounts enabled for membership invitations.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .memberships
        .invite(
            &InviteMembershipsRequestBody::InviteMembershipsRequestBodyUserID(
                InviteMembershipsRequestBodyUserID {
                    plan_id: "plan_xxxxxxxxxxxxxx".to_string(),
                    user_id: "user_xxxxxxxxxxxxxx".to_string(),
                    ..Default::default()
                },
            ),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">retrieve</a>(id: String) -> Result&lt;Membership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a membership by ID or license key. Accessible to the account and to the membership's own user.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.memberships.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Membership ID (`mem_` tag), or a software license key.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">update</a>(id: String, request: UpdateMembershipsRequest) -> Result&lt;Membership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a membership: merge metadata key-value pairs, or toggle `cancel_at_period_end` — `true` schedules the cancellation for the end of the current billing period, `false` reverses a pending one.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .memberships
        .update(
            &"id".to_string(),
            &UpdateMembershipsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Membership ID (`mem_` tag), or a software license key.
    
</dd>
</dl>

<dl>
<dd>

**cancel_at_period_end:** `Option<bool>` — `true` cancels at the end of the current billing period (the customer keeps access until then); `false` reverses a pending cancellation.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Key-value pairs to merge into the membership's metadata. Pass an empty object to clear it.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">add_free_days_membership</a>(id: String, request: AddFreeDaysMembershipRequest) -> Result&lt;Membership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add free days to extend a membership's current billing period, expiration date, or Stripe trial.

Required permissions:
 - `member:manage`
 - `member:email:read`
 - `member:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .memberships
        .add_free_days_membership(
            &"mem_xxxxxxxxxxxxxx".to_string(),
            &AddFreeDaysMembershipRequest { free_days: 42 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the membership.
    
</dd>
</dl>

<dl>
<dd>

**free_days:** `i64` — The number of free days to add (1-1095). Extends the billing period, expiration date, or Stripe trial depending on plan type.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">cancel</a>(id: String, request: CancelMembershipsRequest) -> Result&lt;Membership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancels a membership. Pass `cancel_at_period_end: true` to stop auto-renewal and keep access until the current billing period ends. Omit it (or pass `false`) to revoke access immediately. Buyers cannot cancel buy-now-pay-later (`splitit`, `sezzle`) or non-trial split-pay memberships.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .memberships
        .cancel(
            &"id".to_string(),
            &CancelMembershipsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Membership ID (`mem_` tag).
    
</dd>
</dl>

<dl>
<dd>

**cancel_at_period_end:** `Option<bool>` — `true` stops auto-renewal and keeps access until the current billing period ends. Omit or `false` revokes access immediately.
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<String>` — Free-form note recording why the membership was canceled.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">extend</a>(id: String, request: ExtendMembershipsRequest) -> Result&lt;Membership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Adds free days to a membership, extending its current billing period, expiration date, or trial depending on the plan type.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .memberships
        .extend(
            &"id".to_string(),
            &ExtendMembershipsRequest { days: 7 },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Membership ID (`mem_` tag).
    
</dd>
</dl>

<dl>
<dd>

**days:** `i64` — Number of free days to add (1-1095).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">pause</a>(id: String, request: PauseMembershipsRequest) -> Result&lt;Membership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Pauses a membership's recurring payment collection. The customer keeps access but is not charged until the membership is resumed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .memberships
        .pause(
            &"id".to_string(),
            &PauseMembershipsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Membership ID (`mem_` tag).
    
</dd>
</dl>

<dl>
<dd>

**until:** `Option<String>` — ISO 8601 time to automatically resume payment collection. Must be in the future; only supported for memberships billed by Whop.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">resume</a>(id: String) -> Result&lt;Membership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Resumes a previously paused membership's recurring payment collection. Billing resumes on the next cycle.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.memberships.resume(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Membership ID (`mem_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">resync_access_membership</a>(id: String) -> Result&lt;Membership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Re-run access fulfillment for a membership. Recomputes the member's content access on Whop, re-validates their Discord link (re-adding them to the server and re-assigning roles if needed), and re-fulfills TradingView indicator access. Telegram access is invite-based and cannot be resynced here. The outcome is written to the membership's logs.

Required permissions:
 - `membership:resync_access`
 - `member:email:read`
 - `member:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .memberships
        .resync_access_membership(&"mem_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the membership to resync access for.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">transfer</a>(id: String) -> Result&lt;TransferMembershipsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a one-use transfer URL for a membership. Opening the URL while logged into a different Whop account claims the membership onto that account. The membership's buyer can generate a link for their own membership with `membership:transfer` when the product allows transfers and the membership is `trialing`, `active`, or `completed`. An account credential with `membership:update` bypasses both restrictions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.memberships.transfer(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Membership ID (`mem_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.memberships.<a href="/src/api/resources/memberships/client.rs">uncancel_membership</a>(id: String) -> Result&lt;Membership, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Reverse a pending cancellation for a membership that was scheduled to cancel at period end.

Required permissions:
 - `member:manage`
 - `member:email:read`
 - `member:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .memberships
        .uncancel_membership(&"mem_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the membership to uncancel.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Messages
<details><summary><code>client.messages.<a href="/src/api/resources/messages/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, channel_id: Option&lt;String&gt;, direction: Option&lt;Option&lt;Direction&gt;&gt;) -> Result&lt;ListMessagesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of messages within a specific experience chat, DM, or group chat channel, sorted by creation time.

Required permissions (one of):
 - `chat:read`
 - `dms:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .messages
        .list(
            &MessagesListQueryRequest {
                first: Some(42),
                last: Some(42),
                channel_id: "channel_id".to_string(),
                after: None,
                before: None,
                direction: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**channel_id:** `String` — The unique identifier of the channel or experience to list messages for.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<Direction>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.messages.<a href="/src/api/resources/messages/client.rs">create</a>(request: CreateMessagesRequest) -> Result&lt;Message, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a new message in an experience chat, DM, or group chat channel. Supports text content, attachments, polls, and replies.

Required permissions (one of):
 - `chat:message:create`
 - `dms:message:manage`
 - `livestream:chat:write`
 - `support_chat:message:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .messages
        .create(
            &CreateMessagesRequest {
                channel_id: "channel_id".to_string(),
                content: "content".to_string(),
                attachments: None,
                auto_detect_links: None,
                poll: None,
                replying_to_message_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attachments:** `Option<Option<Vec<CreateMessagesRequestAttachmentsItem>>>` — A list of file attachments to include with the message, such as images or videos.
    
</dd>
</dl>

<dl>
<dd>

**auto_detect_links:** `Option<Option<bool>>` — Automatically detect URLs in the message and generate link previews.
    
</dd>
</dl>

<dl>
<dd>

**channel_id:** `String` — The unique identifier of the channel or experience to send the message in. For example, 'exp_xxxxx' or 'feed_xxxxx'.
    
</dd>
</dl>

<dl>
<dd>

**content:** `String` — The body of the message in Markdown format. For example, 'Hello **world**'.
    
</dd>
</dl>

<dl>
<dd>

**poll:** `Option<Option<CreateMessagesRequestPoll>>` — A poll to attach to this message, allowing recipients to vote on options.
    
</dd>
</dl>

<dl>
<dd>

**replying_to_message_id:** `Option<Option<String>>` — The unique identifier of the message this is replying to, creating a threaded reply.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.messages.<a href="/src/api/resources/messages/client.rs">retrieve</a>(id: String) -> Result&lt;Message, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing message.

Required permissions (one of):
 - `chat:read`
 - `dms:read`
 - `livestream:chat:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.messages.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the message to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.messages.<a href="/src/api/resources/messages/client.rs">delete</a>(id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a message from an experience chat, DM, or group chat channel. Only the message author or a channel admin can delete a message.

Required permissions (one of):
 - `chat:message:create` and `chat:read`
 - `dms:message:manage` and `dms:read`
 - `livestream:chat:write` and `livestream:chat:read`
 - `support_chat:message:create` and `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.messages.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the message to delete.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.messages.<a href="/src/api/resources/messages/client.rs">update</a>(id: String, request: UpdateMessagesRequest) -> Result&lt;Message, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Edit the content, attachments, or pinned status of an existing message in an experience chat, DM, or group chat channel.

Required permissions (one of):
 - `chat:message:create`
 - `dms:message:manage`
 - `livestream:chat:write`
 - `support_chat:message:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .messages
        .update(
            &"id".to_string(),
            &UpdateMessagesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the message to update.
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Option<Vec<UpdateMessagesRequestAttachmentsItem>>>` — A replacement list of file attachments for this message, such as images or videos.
    
</dd>
</dl>

<dl>
<dd>

**content:** `Option<Option<String>>` — The updated body of the message in Markdown format. For example, 'Hello **world**'.
    
</dd>
</dl>

<dl>
<dd>

**is_pinned:** `Option<Option<bool>>` — Whether this message should be pinned to the top of the channel.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Notifications
<details><summary><code>client.notifications.<a href="/src/api/resources/notifications/client.rs">list</a>(unread: Option&lt;Option&lt;bool&gt;&gt;, experience_id: Option&lt;Option&lt;String&gt;&gt;, account_id: Option&lt;Option&lt;String&gt;&gt;, mentions: Option&lt;Option&lt;bool&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListNotificationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the authenticated user's notifications, newest first. Requires a user credential — an account API key has no notification feed. Without filters the feed spans every experience the user belongs to plus the teams they are a member of.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .notifications
        .list(
            &NotificationsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**unread:** `Option<bool>` — Only return notifications created since the user last viewed their source.
    
</dd>
</dl>

<dl>
<dd>

**experience_id:** `Option<String>` — Only return notifications from this experience (`exp_` tag).
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — Only return team notifications for this account (`biz_` tag).
    
</dd>
</dl>

<dl>
<dd>

**mentions:** `Option<bool>` — Only return notifications that mention the user directly.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of notifications to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor (a notification `id` from a previous page); returns notifications older than it.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notifications.<a href="/src/api/resources/notifications/client.rs">create</a>(request: CreateNotificationsRequest) -> Result&lt;CreateNotificationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Queues a notification to every user of an experience or to an account's team, processed asynchronously. Every send is attributed to an app: use an app API key, or a credential acting on behalf of an app. Narrow the audience with `user_ids` to send a mention.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .notifications
        .create(
            &CreateNotificationsRequest {
                content: "Drop off at 4180 Burnet Rd. Plan on two days for the full coating."
                    .to_string(),
                title: "Your ceramic coating is booked".to_string(),
                account_id: None,
                experience_id: None,
                icon_user_id: None,
                rest_path: None,
                subtitle: None,
                user_ids: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account whose team members receive the notification (`biz_` tag). Exactly one of `experience_id` or `account_id` is required.
    
</dd>
</dl>

<dl>
<dd>

**content:** `String` — Main body text of the notification.
    
</dd>
</dl>

<dl>
<dd>

**experience_id:** `Option<String>` — Experience whose users receive the notification (`exp_` tag). Exactly one of `experience_id` or `account_id` is required.
    
</dd>
</dl>

<dl>
<dd>

**icon_user_id:** `Option<Option<String>>` — User whose profile picture is used as the notification icon. Defaults to the experience or account avatar.
    
</dd>
</dl>

<dl>
<dd>

**rest_path:** `Option<Option<String>>` — Path segment appended to the generated deep link that opens your app, for example `/settings/billing`.
    
</dd>
</dl>

<dl>
<dd>

**subtitle:** `Option<Option<String>>` — Optional secondary line displayed below the title.
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` — Headline text of the notification.
    
</dd>
</dl>

<dl>
<dd>

**user_ids:** `Option<Vec<String>>` — Optional `user_` tags narrowing the audience. When provided, only these users are notified (as a mention), provided they are in the targeted experience or account.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notifications.<a href="/src/api/resources/notifications/client.rs">badges</a>(last_fetched_at: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;BadgesNotificationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the authenticated user's per-experience unread badge state. Requires a user credential. Returns one row per experience the user belongs to (or per requested experience).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .notifications
        .badges(
            &BadgesQueryRequest {
                experience_ids: vec![Some("exp_xxxxxxxxxxxxxx".to_string())],
                last_fetched_at: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**experience_ids:** `Option<String>` — Only return badges for these experiences (`exp_` tags).
    
</dd>
</dl>

<dl>
<dd>

**last_fetched_at:** `Option<String>` — The client's last fetched-at ISO 8601 timestamp, used to partially refresh badges after a websocket message.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notifications.<a href="/src/api/resources/notifications/client.rs">mark_read</a>(request: MarkReadNotificationsRequest) -> Result&lt;MarkReadNotificationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Marks the authenticated user's notifications as read: one experience's (`experience_id`) or everything (`all: true`) — exactly one of the two. Requires a user credential. Responds with the refreshed badge rows for the affected scope.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .notifications
        .mark_read(
            &MarkReadNotificationsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**all:** `Option<bool>` — Pass `true` to mark every notification read. Exactly one of `experience_id` or `all` is required.
    
</dd>
</dl>

<dl>
<dd>

**experience_id:** `Option<String>` — Experience to mark read (`exp_` tag). Exactly one of `experience_id` or `all` is required.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notifications.<a href="/src/api/resources/notifications/client.rs">retrieve</a>(id: String) -> Result&lt;Notification, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single notification by id — either an `id` returned by List Notifications, or the ephemeral id delivered with a push/websocket event. Requires a user credential.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.notifications.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — A notification `id` from List Notifications, or the id delivered with a push/websocket event.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Partners
<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">create</a>() -> Result&lt;CreatePartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Enrolls the calling user in the Whop partner program, making their partner businesses eligible for earnings. Idempotent — enrolling again keeps the original enrollment time.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.partners.create(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">leaderboard</a>(period: Option&lt;Option&lt;LeaderboardPartnersRequestPeriod&gt;&gt;) -> Result&lt;LeaderboardPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Ranks referrers by partner business earnings — all-time by default, or over the current day, month, year, or trailing 30 days. Authentication is optional: authenticated callers also get their own standing, anonymous callers get the rankings alone.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .partners
        .leaderboard(
            &LeaderboardQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**period:** `Option<LeaderboardPartnersRequestPeriod>` — Time window for the rankings. `day`, `month`, and `year` count earnings since the start of the current calendar day, month, or year; `last_30_days` counts earnings over the trailing 30 days; `all_time` ranks lifetime earnings.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners.<a href="/src/api/resources/partners/client.rs">referred_users</a>(has_businesses: Option&lt;Option&lt;bool&gt;&gt;, has_earning_businesses: Option&lt;Option&lt;bool&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ReferredUsersPartnersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the users the caller referred onto Whop (newest first), each with the second-tier earnings the caller has made from that user's businesses.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .partners
        .referred_users(
            &ReferredUsersQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**has_businesses:** `Option<bool>` — When true, only referred users who brought at least one business onto Whop.
    
</dd>
</dl>

<dl>
<dd>

**has_earning_businesses:** `Option<bool>` — When true, only referred users with at least one business that has generated earnings.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of referred users to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of referred users to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to fetch the page before (from page_info.start_cursor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Payment Method Domains
<details><summary><code>client.payment_method_domains.<a href="/src/api/resources/payment_method_domains/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, hostname: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListPaymentMethodDomainsRequestStatus&gt;&gt;, provider: Option&lt;Option&lt;ListPaymentMethodDomainsRequestProvider&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListPaymentMethodDomainsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListPaymentMethodDomainsRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListPaymentMethodDomainsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists payment method domains. Without `account_id`, returns the caller's own domains and those of every connected account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payment_method_domains
        .list(
            &PaymentMethodDomainsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Only domains registered for this account (`biz_` tag). Defaults to the caller's account plus its connected accounts.
    
</dd>
</dl>

<dl>
<dd>

**hostname:** `Option<String>` — Only the domain with this exact hostname.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListPaymentMethodDomainsRequestStatus>` — Only domains with this verification status.
    
</dd>
</dl>

<dl>
<dd>

**provider:** `Option<ListPaymentMethodDomainsRequestProvider>` — Only domains registered with this wallet provider.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only domains created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only domains created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListPaymentMethodDomainsRequestOrder>` — Sort field.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListPaymentMethodDomainsRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of domains to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to paginate forwards from.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of domains to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to paginate backwards from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_method_domains.<a href="/src/api/resources/payment_method_domains/client.rs">create</a>(request: CreatePaymentMethodDomainsRequest) -> Result&lt;PaymentMethodDomain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Registers a hostname with the wallet provider and attempts verification inline. Returns `verified` when the provider fetched the domain-association file (for Apple Pay, `/.well-known/apple-developer-merchantid-domain-association`), or `pending` when it could not — host the file, then retry with the verify endpoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payment_method_domains
        .create(
            &CreatePaymentMethodDomainsRequest {
                hostname: "pending.shinetime.example".to_string(),
                account_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account to register the domain for (`biz_` tag). Defaults to the caller's account.
    
</dd>
</dl>

<dl>
<dd>

**hostname:** `String` — Hostname to register (e.g. `checkout.shinetime.example`).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_method_domains.<a href="/src/api/resources/payment_method_domains/client.rs">retrieve</a>(id: String) -> Result&lt;PaymentMethodDomain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a payment method domain to check its verification status.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payment_method_domains
        .retrieve(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payment method domain, prefixed `pmd_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_method_domains.<a href="/src/api/resources/payment_method_domains/client.rs">delete</a>(id: String) -> Result&lt;DeletePaymentMethodDomainsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Unregisters a payment method domain so its wallet payment methods stop rendering there.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payment_method_domains
        .delete(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payment method domain, prefixed `pmd_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_method_domains.<a href="/src/api/resources/payment_method_domains/client.rs">verify</a>(id: String) -> Result&lt;PaymentMethodDomain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Re-attempts provider verification of a pending domain once the association file is hosted. Fails with a `bad_request` explaining what to fix; verifying an already `verified` domain is a no-op.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payment_method_domains
        .verify(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payment method domain, prefixed `pmd_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## PaymentMethods
<details><summary><code>client.payment_methods.<a href="/src/api/resources/payment_methods/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, member_id: Option&lt;Option&lt;String&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, direction: Option&lt;Option&lt;Direction&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, future_usage: Option&lt;Option&lt;FutureUsageTypes&gt;&gt;, has_payer_document: Option&lt;Option&lt;bool&gt;&gt;, expired: Option&lt;Option&lt;bool&gt;&gt;, broken: Option&lt;Option&lt;bool&gt;&gt;) -> Result&lt;ListPaymentMethodsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of payment methods for a member or company, or for the authenticated user when neither is given, with optional filtering by creation date. A payment method is a stored representation of how a customer intends to pay, such as a card, bank account, or digital wallet.

Required permissions:
 - `member:payment_methods:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payment_methods
        .list(
            &PaymentMethodsListQueryRequest {
                first: Some(42),
                last: Some(42),
                member_id: Some("mber_xxxxxxxxxxxxx".to_string()),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                after: None,
                before: None,
                direction: None,
                future_usage: None,
                payment_method_types: vec![],
                card_brands: vec![],
                card_funding_types: vec![],
                has_payer_document: None,
                expired: None,
                broken: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**member_id:** `Option<String>` — The unique identifier of the member to list payment methods for. Omit this and company_id to list your own saved payment methods.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of the company. Provide either this or member_id, not both. Omit both to address your own saved payment methods.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<Direction>` 
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return payment methods created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return payment methods created after this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**future_usage:** `Option<FutureUsageTypes>` 
    
</dd>
</dl>

<dl>
<dd>

**payment_method_types:** `Option<PaymentMethodTypes>` — Only return payment methods of these types. Pass the eligible `type` values from the payment method types catalogue so the list holds nothing the purchase cannot take. An empty list returns no payment methods.
    
</dd>
</dl>

<dl>
<dd>

**card_brands:** `Option<CardBrands>` — Only return cards on these networks, such as the networks the seller accepts. Payment methods that are not cards are unaffected.
    
</dd>
</dl>

<dl>
<dd>

**card_funding_types:** `Option<CardFundingTypes>` — Only return cards funded this way. A card whose funding could not be determined is excluded, and payment methods that are not cards are unaffected.
    
</dd>
</dl>

<dl>
<dd>

**has_payer_document:** `Option<bool>` — Filter cards by whether they carry the payer identity document their payment provider requires. Payment methods that are not cards are unaffected.
    
</dd>
</dl>

<dl>
<dd>

**expired:** `Option<bool>` — Filter by expiry. Only a card can expire, so `false` keeps every payment method that is not past its expiration month and `true` returns expired cards alone.
    
</dd>
</dl>

<dl>
<dd>

**broken:** `Option<bool>` — Filter by whether the stored credential has permanently stopped charging, such as a vault entry its provider closed.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_methods.<a href="/src/api/resources/payment_methods/client.rs">retrieve</a>(id: String, company_id: Option&lt;Option&lt;String&gt;&gt;, member_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;PaymentMethod, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing payment method. Addresses a member's wallet when member_id or company_id is given, otherwise your own.

Required permissions:
 - `member:payment_methods:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payment_methods
        .retrieve(
            &"payt_xxxxxxxxxxxxx".to_string(),
            &PaymentMethodsRetrieveQueryRequest {
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                member_id: Some("mber_xxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payment method.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of the company. Provide either this or member_id, not both. Omit both to address your own saved payment methods.
    
</dd>
</dl>

<dl>
<dd>

**member_id:** `Option<String>` — The unique identifier of the member. Provide either this or company_id, not both. Omit both to address your own saved payment methods.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_methods.<a href="/src/api/resources/payment_methods/client.rs">delete_payment_method</a>(id: String, company_id: Option&lt;Option&lt;String&gt;&gt;, member_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a saved payment method. Cannot delete a payment method attached to an active subscription.

Required permissions:
 - `member:payment_methods:manage`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payment_methods
        .delete_payment_method(
            &"payt_xxxxxxxxxxxxx".to_string(),
            &DeletePaymentMethodQueryRequest {
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                member_id: Some("mber_xxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payment method to delete.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of the company. Provide either this or member_id, not both. Omit both to address your own saved payment methods.
    
</dd>
</dl>

<dl>
<dd>

**member_id:** `Option<String>` — The unique identifier of the member. Provide either this or company_id, not both. Omit both to address your own saved payment methods.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Payments
<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, direction: Option&lt;Option&lt;Direction&gt;&gt;, order: Option&lt;Option&lt;ReceiptV2Order&gt;&gt;, include_free: Option&lt;Option&lt;bool&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, updated_before: Option&lt;Option&lt;String&gt;&gt;, updated_after: Option&lt;Option&lt;String&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListPaymentsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of payments for the actor in context, with optional filtering by product, plan, status, billing reason, currency, and creation date.

Required permissions:
 - `payment:basic:read`
 - `plan:basic:read`
 - `access_pass:basic:read`
 - `member:email:read`
 - `member:basic:read`
 - `member:phone:read`
 - `promo_code:basic:read`
 - `shipment:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payments
        .list(
            &PaymentsListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                updated_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                updated_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                after: None,
                before: None,
                direction: None,
                order: None,
                product_ids: vec![],
                billing_reasons: vec![],
                currencies: vec![],
                plan_ids: vec![],
                statuses: vec![],
                substatuses: vec![],
                include_free: None,
                query: None,
                checkout_configuration_ids: vec![],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of the company to list payments for.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<Direction>` 
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ReceiptV2Order>` 
    
</dd>
</dl>

<dl>
<dd>

**product_ids:** `Option<String>` — Filter payments to only those associated with these specific product identifiers.
    
</dd>
</dl>

<dl>
<dd>

**billing_reasons:** `Option<BillingReasons>` — Filter payments by their billing reason.
    
</dd>
</dl>

<dl>
<dd>

**currencies:** `Option<Currencies>` — Filter payments by their currency code.
    
</dd>
</dl>

<dl>
<dd>

**plan_ids:** `Option<String>` — Filter payments to only those associated with these specific plan identifiers.
    
</dd>
</dl>

<dl>
<dd>

**statuses:** `Option<ReceiptStatus>` — Filter payments by their current status.
    
</dd>
</dl>

<dl>
<dd>

**substatuses:** `Option<FriendlyReceiptStatus>` — Filter payments by their current substatus for more granular filtering.
    
</dd>
</dl>

<dl>
<dd>

**include_free:** `Option<bool>` — Whether to include payments with a zero amount.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return payments created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return payments created after this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**updated_before:** `Option<String>` — Only return payments last updated before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**updated_after:** `Option<String>` — Only return payments last updated after this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Search payments by user ID, membership ID, user email, name, or username. Email filtering requires the member:email:read permission.
    
</dd>
</dl>

<dl>
<dd>

**checkout_configuration_ids:** `Option<String>` — Only return payments from these checkout configurations.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">create</a>(request: CreatePaymentsRequestBody) -> Result&lt;CreatePaymentsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Charge an existing member off-session using one of their stored payment methods. You can provide an existing plan, or create a new one in-line. This endpoint will respond with a payment object immediately, but the payment is processed asynchronously in the background. Use webhooks to be notified when the payment succeeds or fails.

Required permissions:
 - `payment:charge`
 - `plan:create`
 - `access_pass:create`
 - `access_pass:update`
 - `plan:basic:read`
 - `access_pass:basic:read`
 - `member:email:read`
 - `member:basic:read`
 - `member:phone:read`
 - `promo_code:basic:read`
 - `shipment:basic:read`
 - `payment:dispute:read`
 - `payment:resolution_center_case:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payments
        .create(
            &CreatePaymentsRequestBody::CreatePaymentsRequestBodyZero(
                CreatePaymentsRequestBodyZero {
                    capture: None,
                    company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                    confirmation_token: "confirmation_token".to_string(),
                    email: None,
                    metadata: None,
                    payment_method_id: None,
                    plan: CreatePaymentsRequestBodyZeroPlan {
                        application_fee_amount: None,
                        billing_period: None,
                        currency: Currencies::Usd,
                        description: None,
                        expiration_days: None,
                        force_create_new_plan: None,
                        initial_price: None,
                        internal_notes: None,
                        plan_type: None,
                        product: None,
                        product_id: None,
                        renewal_price: None,
                        title: None,
                        trial_period_days: None,
                        visibility: None,
                    },
                    promo_code_id: None,
                    return_url: None,
                },
            ),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">retrieve</a>(id: String) -> Result&lt;RetrievePaymentsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing payment.

Required permissions:
 - `payment:basic:read`
 - `plan:basic:read`
 - `access_pass:basic:read`
 - `member:email:read`
 - `member:basic:read`
 - `member:phone:read`
 - `promo_code:basic:read`
 - `shipment:basic:read`
 - `payment:dispute:read`
 - `payment:resolution_center_case:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payments
        .retrieve(&"pay_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payment.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">capture</a>(id: String) -> Result&lt;PaymentStatus, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Captures the full amount of a card payment created with `capture: false`. The payment must still be in `requires_capture` before `capture_expires_at`. Partial capture, multiple captures, capturing more than the authorized amount, and tips are not supported.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.payments.capture(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">list_fees</a>(id: String, after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;ListFeesPaymentsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the list of fees associated with a specific payment, including platform fees and processing fees.

Required permissions:
 - `payment:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payments
        .list_fees(
            &"pay_xxxxxxxxxxxxxx".to_string(),
            &ListFeesQueryRequest {
                first: Some(42),
                last: Some(42),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payment to list fees for.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">refund</a>(id: String, request: RefundPaymentsRequest) -> Result&lt;Payment, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Issue a full or partial refund for a payment. The refund is processed through the original payment processor and the membership status is updated accordingly.

Required permissions:
 - `payment:manage`
 - `plan:basic:read`
 - `access_pass:basic:read`
 - `member:email:read`
 - `member:basic:read`
 - `member:phone:read`
 - `promo_code:basic:read`
 - `shipment:basic:read`
 - `payment:dispute:read`
 - `payment:resolution_center_case:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payments
        .refund(
            &"pay_xxxxxxxxxxxxxx".to_string(),
            &RefundPaymentsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payment to refund.
    
</dd>
</dl>

<dl>
<dd>

**partial_amount:** `Option<Option<f64>>` — The amount to refund. For multi-currency payments, this is in the charge currency (what the buyer paid). For single-currency, this is in the payment currency. If omitted, the full payment amount is refunded.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">retry</a>(id: String) -> Result&lt;Payment, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retry a failed or pending payment. This re-attempts the charge using the original payment method and plan details.

Required permissions:
 - `payment:manage`
 - `plan:basic:read`
 - `access_pass:basic:read`
 - `member:email:read`
 - `member:basic:read`
 - `member:phone:read`
 - `promo_code:basic:read`
 - `shipment:basic:read`
 - `payment:dispute:read`
 - `payment:resolution_center_case:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payments
        .retry(&"pay_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payment to retry.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">void</a>(id: String) -> Result&lt;Payment, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Void a payment that has not yet been settled. Voiding cancels the payment before it is captured by the payment processor.

Required permissions:
 - `payment:manage`
 - `plan:basic:read`
 - `access_pass:basic:read`
 - `member:email:read`
 - `member:basic:read`
 - `member:phone:read`
 - `promo_code:basic:read`
 - `shipment:basic:read`
 - `payment:dispute:read`
 - `payment:resolution_center_case:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payments
        .void(&"pay_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payment to void.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">update_return_url</a>(payment_id: String, request: UpdateReturnUrlPaymentsRequest) -> Result&lt;PaymentStatus, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Changes where the buyer lands after completing an off-site step, up until they return. Accepts either a secret key or the payment's own `client_secret`, so the surface that knows the final destination can set it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payments
        .update_return_url(
            &"payment_id".to_string(),
            &UpdateReturnURLPaymentsRequest {
                return_url: "https://shinetime.example/checkout/thanks".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**payment_id:** `String` — The unique identifier of the payment.
    
</dd>
</dl>

<dl>
<dd>

**return_url:** `String` — Where the buyer continues after completing an off-site step. Must be an absolute https URL without credentials (http is allowed for localhost), at most 2,048 characters.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payments.<a href="/src/api/resources/payments/client.rs">retrieve_status</a>(payment_id: String) -> Result&lt;PaymentStatus, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves how far a payment has got and what the buyer must do next, if anything. A payment is collected in the background, so poll this rather than reading the create response. Accepts either a secret key or the payment's own `client_secret`, so the surface collecting the payment can poll it directly.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payments
        .retrieve_status(&"payment_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**payment_id:** `String` — The unique identifier of the payment.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## PayoutAccounts
<details><summary><code>client.payout_accounts.<a href="/src/api/resources/payout_accounts/client.rs">retrieve</a>(id: String) -> Result&lt;PayoutAccount, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing payout account.

Required permissions:
 - `payout:account:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payout_accounts
        .retrieve(&"poact_xxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payout account to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## PayoutMethods
<details><summary><code>client.payout_methods.<a href="/src/api/resources/payout_methods/client.rs">list_payout_method</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;String&gt;) -> Result&lt;ListPayoutMethodResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of active payout methods configured for a company, ordered by most recently created.

Required permissions:
 - `payout:destination:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payout_methods
        .list_payout_method(
            &ListPayoutMethodQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                after: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to list payout methods for.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payout_methods.<a href="/src/api/resources/payout_methods/client.rs">retrieve_payout_method</a>(id: String) -> Result&lt;PayoutMethod, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing payout method.

Required permissions:
 - `payout:destination:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payout_methods
        .retrieve_payout_method(&"potk_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the payout method to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Payouts
<details><summary><code>client.payouts.<a href="/src/api/resources/payouts/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, currency: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListPayoutsRequestStatus&gt;&gt;, source: Option&lt;Option&lt;ListPayoutsRequestSource&gt;&gt;, payout_method_id: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListPayoutsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists an account's or user's payouts, newest first.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payouts
        .list(
            &PayoutsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The owning account ID (a biz_ identifier). Provide this or user_id.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The owning user ID (a user_ identifier). Provide this or account_id.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — Optional currency code filter, for example `usd`.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListPayoutsRequestStatus>` — Filter to payouts whose `status` reads this word, matching exactly what this version displays — `reversed` finds settled payouts the bank later returned. Requires Api-Version-Date 2026-08-21 or later.
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<ListPayoutsRequestSource>` — Filter by how the payout was created. Payouts created before source tracking or through internal tooling carry no source and never match.
    
</dd>
</dl>

<dl>
<dd>

**payout_method_id:** `Option<String>` — Filter to payouts sent to one saved payout method (a pytk_ identifier). An unknown id matches nothing.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only payouts created before this ISO 8601 time (exclusive).
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only payouts created at or after this ISO 8601 time (inclusive).
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of payouts to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of payouts to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to fetch the page before (from page_info.start_cursor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payouts.<a href="/src/api/resources/payouts/client.rs">create</a>(request: CreatePayoutsRequestBody) -> Result&lt;CreatePayoutsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Sends money from an account or user balance to a saved payout method for that owner.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payouts
        .create(
            &CreatePayoutsRequestBody::Unknown(serde_json::json!({"key":"value"})),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payouts.<a href="/src/api/resources/payouts/client.rs">create_quote</a>(request: CreateQuotePayoutsRequest) -> Result&lt;CreateQuotePayoutsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a short-lived, provider-backed quote for a payout. No funds move until the returned quote_token is submitted to POST /payouts. An Idempotency-Key header is required.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payouts
        .create_quote(
            &CreateQuotePayoutsRequest {
                amount: 6762.41,
                payout_method_id: "potk_xxxxxxxxxxxxxx".to_string(),
                account_id: None,
                currency: None,
                platform_covers_fees: None,
                speed: None,
                user_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account to pay out from, prefixed `biz_`. Provide exactly one of `account_id` or `user_id`.
    
</dd>
</dl>

<dl>
<dd>

**amount:** `String` — The amount to pay out in the specified currency.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — The balance currency to pay out.
    
</dd>
</dl>

<dl>
<dd>

**payout_method_id:** `String` — The saved payout method to quote (a potk_ identifier).
    
</dd>
</dl>

<dl>
<dd>

**platform_covers_fees:** `Option<bool>` — Whether the parent platform covers the payout fee instead of the account being paid out.
    
</dd>
</dl>

<dl>
<dd>

**speed:** `Option<CreateQuotePayoutsRequestSpeed>` — How fast the funds should arrive.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — User to pay out from, prefixed `user_`. Provide exactly one of `account_id` or `user_id`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payouts.<a href="/src/api/resources/payouts/client.rs">retrieve</a>(id: String, account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;RetrievePayoutsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetches one payout by its `wdrl_` ID, or by the `cofr_` conversion request ID a stablecoin payout carries as `payout_request_id` — both ids answer with the same payout object.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payouts
        .retrieve(
            &"id".to_string(),
            &PayoutsRetrieveQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Payout ID, prefixed `wdrl_` for a payout returned by `GET /payouts` or `cofr_` for the payout request returned by `POST /payouts`.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — Owning account ID, prefixed `biz_`. Provide exactly one of `account_id` or `user_id`.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Owning user ID, prefixed `user_`. Provide exactly one of `account_id` or `user_id`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payouts.<a href="/src/api/resources/payouts/client.rs">cancel</a>(id: String, user_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;CancelPayoutsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancels a payout that is still in review and returns the funds, fees included, to the balance. A payout can be canceled while its status is `in_review`. A `requested` payout is still being prepared (its funds may be converting) and answers 409 until it reaches review; from `processing` on, the money is on its way and the answer is 409 with error type `not_cancelable`. Canceling a payout that is already canceled succeeds and returns it unchanged.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payouts
        .cancel(
            &"id".to_string(),
            &CancelQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Payout ID, prefixed `wdrl_`, or the `cofr_` payout request ID returned by `POST /payouts` — both cancel the same payout.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Owning user ID, prefixed `user_`. Provide exactly one of `account_id` or `user_id`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## People
<details><summary><code>client.people.<a href="/src/api/resources/people/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, attribution_model: Option&lt;Option&lt;ListPeopleRequestAttributionModel&gt;&gt;, custom_event: Option&lt;Option&lt;String&gt;&gt;, event_from: Option&lt;Option&lt;String&gt;&gt;, event_to: Option&lt;Option&lt;String&gt;&gt;, audience_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, email: Option&lt;Option&lt;String&gt;&gt;, phone: Option&lt;Option&lt;String&gt;&gt;, country: Option&lt;Option&lt;String&gt;&gt;, has_purchased: Option&lt;Option&lt;bool&gt;&gt;, contactable: Option&lt;Option&lt;bool&gt;&gt;, first_seen_within_days: Option&lt;Option&lt;i64&gt;&gt;, last_seen_within_days: Option&lt;Option&lt;i64&gt;&gt;, first_seen_after: Option&lt;Option&lt;String&gt;&gt;, first_seen_before: Option&lt;Option&lt;String&gt;&gt;, last_seen_after: Option&lt;Option&lt;String&gt;&gt;, last_seen_before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListPeopleRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListPeopleRequestDirection&gt;&gt;) -> Result&lt;ListPeopleResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the people (visitors and customers) of an account: the identity-linked person profiles aggregated from every pixel, payment, and platform event — identities, purchases and LTV, geo/device profile, traffic sources, and first/last marketing touches.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .people
        .list(
            &PeopleListQueryRequest {
                source: vec![Some("direct".to_string())],
                event_name: vec![Some("payment.completed".to_string())],
                account_id: None,
                query: None,
                attribution_model: None,
                custom_event: None,
                event_from: None,
                event_to: None,
                audience_id: None,
                user_id: None,
                email: None,
                phone: None,
                country: None,
                has_purchased: None,
                contactable: None,
                first_seen_within_days: None,
                last_seen_within_days: None,
                first_seen_after: None,
                first_seen_before: None,
                last_seen_after: None,
                last_seen_before: None,
                first: None,
                after: None,
                before: None,
                order: None,
                direction: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account ID, prefixed `biz_`. Optional for account API keys; required for credentials that can access multiple accounts.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Search people by name, email, phone, or whop user ID (case-insensitive substring match).
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<String>` — Only include people acquired from any of these sources — canonical paths (whop:<campaign>:<group>:<ad>, ext:<platform>:..., referrer:<domain>, direct, other), exact or with a trailing :* prefix. The same vocabulary the events / people metrics use.
    
</dd>
</dl>

<dl>
<dd>

**attribution_model:** `Option<ListPeopleRequestAttributionModel>` — Attribution model the source filter matches against (defaults to last_touch).
    
</dd>
</dl>

<dl>
<dd>

**event_name:** `Option<String>` — Only include people who fired any of these events, e.g. payment.completed or page.checkout.view.
    
</dd>
</dl>

<dl>
<dd>

**custom_event:** `Option<String>` — Only include people who fired this custom pixel event.
    
</dd>
</dl>

<dl>
<dd>

**event_from:** `Option<String>` — With event_to plus an event or source filter, switches to exact-population mode: person ids are resolved and paginated on the events side within this window (the same query the people metric counts), then hydrated per page.
    
</dd>
</dl>

<dl>
<dd>

**event_to:** `Option<String>` — The inclusive end of the event window for exact-population mode.
    
</dd>
</dl>

<dl>
<dd>

**audience_id:** `Option<String>` — Only include people in this audience. An audience that keeps itself up to date resolves to the People filters that define it, so this always reflects who matches now; uploaded lists and point-in-time snapshots match their recorded members.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Only include the person linked to this whop user ID.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Only include the person linked to this email address.
    
</dd>
</dl>

<dl>
<dd>

**phone:** `Option<String>` — Only include the person linked to this phone number.
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — Only include people whose most recent visit came from this ISO 3166-1 alpha-2 country code.
    
</dd>
</dl>

<dl>
<dd>

**has_purchased:** `Option<bool>` — true for customers only, false for people who have never purchased.
    
</dd>
</dl>

<dl>
<dd>

**contactable:** `Option<bool>` — true for people who have an email address or phone number — the ones an ad platform can match.
    
</dd>
</dl>

<dl>
<dd>

**first_seen_within_days:** `Option<i64>` — Only include people first seen within this many days, as a rolling window.
    
</dd>
</dl>

<dl>
<dd>

**last_seen_within_days:** `Option<i64>` — Only include people last seen within this many days, as a rolling window.
    
</dd>
</dl>

<dl>
<dd>

**first_seen_after:** `Option<String>` — Only include people first seen at or after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**first_seen_before:** `Option<String>` — Only include people first seen before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**last_seen_after:** `Option<String>` — Only include people last seen at or after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**last_seen_before:** `Option<String>` — Only include people last seen before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of people to return (default 100, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor for fetching people after a previous page.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor for fetching people before a later page.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListPeopleRequestOrder>` — Column to sort by. Defaults to last_seen_at.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListPeopleRequestDirection>` — Sort direction. Defaults to desc.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.people.<a href="/src/api/resources/people/client.rs">retrieve</a>(id: String, account_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;RetrievePeopleResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves one person for an account. The identifier can be a person ID (prefixed `prsn_`), a user ID (prefixed `user_`), an email address, or a phone number — merged people resolve to the surviving profile.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .people
        .retrieve(
            &"id".to_string(),
            &PeopleRetrieveQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The person ID, user ID, email address, or phone number to look up.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — Account ID, prefixed `biz_`. Optional for account API keys; required for credentials that can access multiple accounts.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Permissions
<details><summary><code>client.permissions.<a href="/src/api/resources/permissions/client.rs">list</a>(resource_id: Option&lt;String&gt;, actions: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListPermissionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists permission actions and whether the calling credential is granted each one for a resource. Answers for whichever identity authenticated the request — a user session, an OAuth token, or an account or app API key — so it never describes who else can reach the resource.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .permissions
        .list(
            &PermissionsListQueryRequest {
                resource_id: "resource_id".to_string(),
                actions: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**resource_id:** `String` — Tag of the resource to check against: an account (`biz_`), product (`prod_`), experience (`exp_`), or app (`app_`). A resource the credential cannot see is reported as granted nothing rather than as an error.
    
</dd>
</dl>

<dl>
<dd>

**actions:** `Option<String>` — Comma-separated permission actions to check, for example `stats:read,payment:basic:read`. Every action is returned when omitted.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Plans
<details><summary><code>client.plans.<a href="/src/api/resources/plans/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, direction: Option&lt;Option&lt;ListPlansRequestDirection&gt;&gt;, order: Option&lt;Option&lt;ListPlansRequestOrder&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListPlansResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of plans. Omit `account_id` and pass `product_ids` to list a product's public buyable plans.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .plans
        .list(
            &PlansListQueryRequest {
                release_methods: vec![Some("buy_now".to_string())],
                visibilities: vec![Some("visible".to_string())],
                plan_types: vec![Some("renewal".to_string())],
                product_ids: vec![Some("prod_xxxxxxxxxxxxxx".to_string())],
                account_id: None,
                direction: None,
                order: None,
                created_before: None,
                created_after: None,
                first: None,
                after: None,
                last: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The unique identifier of the account to list plans for. Required unless `product_ids` is provided for a public product-plan read.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListPlansRequestDirection>` — The sort direction for results. Defaults to descending.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListPlansRequestOrder>` — The field to sort results by. Defaults to created_at.
    
</dd>
</dl>

<dl>
<dd>

**release_methods:** `Option<String>` — Filter to only plans matching these release methods.
    
</dd>
</dl>

<dl>
<dd>

**visibilities:** `Option<String>` — Filter to only plans matching these visibility states.
    
</dd>
</dl>

<dl>
<dd>

**plan_types:** `Option<String>` — Filter to only plans matching these billing types.
    
</dd>
</dl>

<dl>
<dd>

**product_ids:** `Option<String>` — Filter to only plans belonging to these product identifiers. When `account_id` is omitted, this is required and the response is publicly readable: only visible, non-invoice plans are returned.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return plans created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return plans created after this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of plans to return (default and max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns plans after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of plans to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns plans before this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.plans.<a href="/src/api/resources/plans/client.rs">create</a>(request: CreatePlansRequest) -> Result&lt;Plan, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new pricing plan for a product. The plan defines the billing interval, price, and availability for customers.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .plans
        .create(
            &CreatePlansRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The unique identifier of the account to create this plan for. Defaults to the caller's account.
    
</dd>
</dl>

<dl>
<dd>

**adaptive_pricing_enabled:** `Option<Option<bool>>` — Whether this plan accepts local currency payments via adaptive pricing.
    
</dd>
</dl>

<dl>
<dd>

**billing_period:** `Option<Option<i64>>` — Recurring billing interval in days, such as 30 for monthly or 365 for annual.
    
</dd>
</dl>

<dl>
<dd>

**checkout_styling:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Checkout styling overrides for this plan.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — The three-letter ISO currency code for the plan's pricing. Defaults to USD.
    
</dd>
</dl>

<dl>
<dd>

**custom_fields:** `Option<Option<Vec<CreatePlansRequestCustomFieldsItem>>>` — An array of custom field definitions to collect from customers at checkout. Omitting this field clears existing custom fields.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — A text description of the plan displayed to customers on the product page.
    
</dd>
</dl>

<dl>
<dd>

**expiration_days:** `Option<Option<i64>>` — Access duration in days before the membership expires.
    
</dd>
</dl>

<dl>
<dd>

**image:** `Option<Option<CreatePlansRequestImage>>` — An image displayed on the product page to represent this plan.
    
</dd>
</dl>

<dl>
<dd>

**initial_price:** `Option<Option<f64>>` — Initial amount charged in the plan's currency, e.g. 10.43 for $10.43.
    
</dd>
</dl>

<dl>
<dd>

**internal_notes:** `Option<Option<String>>` — Private notes visible only to the account owner. Not shown to customers.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Custom key-value pairs to store on the plan. Included in webhook payloads for payment and membership events. Max 50 keys, 100 chars per key, 500 chars per string value. The reserved keys `custom_cta` (a checkout call-to-action button label — one of the product custom CTA values, e.g. `subscribe`, `get_offer`) and `custom_cta_url` (a URL the button links to; web or `tel:`) override the product's call to action for this plan and are validated on save.
    
</dd>
</dl>

<dl>
<dd>

**override_tax_type:** `Option<String>` — Override the default tax classification for this specific plan.
    
</dd>
</dl>

<dl>
<dd>

**payment_method_configuration:** `Option<Option<CreatePlansRequestPaymentMethodConfiguration>>` — Explicit payment method configuration for the plan. When not provided, the account's defaults apply.
    
</dd>
</dl>

<dl>
<dd>

**plan_type:** `Option<String>` — Plan billing type, such as `one_time` or `renewal`.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<String>` — The unique identifier of the product to attach this plan to.
    
</dd>
</dl>

<dl>
<dd>

**release_method:** `Option<String>` — Sales method for this plan.
    
</dd>
</dl>

<dl>
<dd>

**renewal_price:** `Option<Option<f64>>` — The amount charged each billing period for recurring plans, in the plan's currency.
    
</dd>
</dl>

<dl>
<dd>

**split_pay_required_payments:** `Option<Option<i64>>` — Installment payments required before the subscription pauses.
    
</dd>
</dl>

<dl>
<dd>

**stock:** `Option<Option<i64>>` — The maximum number of units available for purchase. Ignored when unlimited_stock is true.
    
</dd>
</dl>

<dl>
<dd>

**three_ds_level:** `Option<Option<CreatePlansRequestThreeDsLevel>>` — 3D Secure behavior for this plan. Send `null` to inherit the account default.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The display name of the plan shown to customers on the product page.
    
</dd>
</dl>

<dl>
<dd>

**trial_period_days:** `Option<Option<i64>>` — Free trial duration before the first recurring charge.
    
</dd>
</dl>

<dl>
<dd>

**unlimited_stock:** `Option<Option<bool>>` — Whether the plan has unlimited stock. When true, the stock field is ignored.
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<String>` — Whether the plan is visible to customers or hidden from public view.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.plans.<a href="/src/api/resources/plans/client.rs">retrieve</a>(id: String) -> Result&lt;Plan, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing plan.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.plans.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Plan ID, prefixed `plan_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.plans.<a href="/src/api/resources/plans/client.rs">delete</a>(id: String) -> Result&lt;DeletePlansResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently delete a plan from a product. Existing memberships on this plan will not be affected.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.plans.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Plan ID, prefixed `plan_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.plans.<a href="/src/api/resources/plans/client.rs">update</a>(id: String, request: UpdatePlansRequest) -> Result&lt;Plan, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a plan's pricing, billing interval, visibility, stock, and other settings.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .plans
        .update(
            &"id".to_string(),
            &UpdatePlansRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Plan ID, prefixed `plan_`.
    
</dd>
</dl>

<dl>
<dd>

**adaptive_pricing_enabled:** `Option<Option<bool>>` — Whether this plan accepts local currency payments via adaptive pricing.
    
</dd>
</dl>

<dl>
<dd>

**billing_period:** `Option<Option<i64>>` — Recurring billing interval in days, such as 30 for monthly or 365 for annual.
    
</dd>
</dl>

<dl>
<dd>

**cancel_discount_intervals:** `Option<Option<i64>>` — How many renewals the retention discount applies to. Required when `offer_cancel_discount` is true.
    
</dd>
</dl>

<dl>
<dd>

**cancel_discount_percentage:** `Option<Option<i64>>` — Percentage taken off each discounted renewal. Required when `offer_cancel_discount` is true.
    
</dd>
</dl>

<dl>
<dd>

**checkout_styling:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Checkout styling overrides for this plan.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — The three-letter ISO currency code for the plan's pricing. Defaults to USD.
    
</dd>
</dl>

<dl>
<dd>

**custom_fields:** `Option<Option<Vec<UpdatePlansRequestCustomFieldsItem>>>` — An array of custom field definitions to collect from customers at checkout. Omitting this field clears existing custom fields.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — A text description of the plan displayed to customers on the product page.
    
</dd>
</dl>

<dl>
<dd>

**expiration_days:** `Option<Option<i64>>` — Access duration in days before the membership expires.
    
</dd>
</dl>

<dl>
<dd>

**image:** `Option<Option<UpdatePlansRequestImage>>` — An image displayed on the product page to represent this plan.
    
</dd>
</dl>

<dl>
<dd>

**initial_price:** `Option<Option<f64>>` — Initial amount charged in the plan's currency, e.g. 10.43 for $10.43.
    
</dd>
</dl>

<dl>
<dd>

**internal_notes:** `Option<Option<String>>` — Private notes visible only to the account owner. Not shown to customers.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Custom key-value pairs to store on the plan. Included in webhook payloads for payment and membership events. Max 50 keys, 100 chars per key, 500 chars per string value. The reserved keys `custom_cta` (a checkout call-to-action button label — one of the product custom CTA values, e.g. `subscribe`, `get_offer`) and `custom_cta_url` (a URL the button links to; web or `tel:`) override the product's call to action for this plan and are validated on save.
    
</dd>
</dl>

<dl>
<dd>

**offer_cancel_discount:** `Option<Option<bool>>` — Whether to offer a retention discount when a customer attempts to cancel.
    
</dd>
</dl>

<dl>
<dd>

**override_tax_type:** `Option<String>` — Override the default tax classification for this specific plan.
    
</dd>
</dl>

<dl>
<dd>

**payment_method_configuration:** `Option<Option<UpdatePlansRequestPaymentMethodConfiguration>>` — Explicit payment method configuration for the plan. When not provided, the account's defaults apply.
    
</dd>
</dl>

<dl>
<dd>

**release_method:** `Option<String>` — Sales method for this plan.
    
</dd>
</dl>

<dl>
<dd>

**renewal_price:** `Option<Option<f64>>` — The amount charged each billing period for recurring plans, in the plan's currency.
    
</dd>
</dl>

<dl>
<dd>

**stock:** `Option<Option<i64>>` — The maximum number of units available for purchase. Ignored when unlimited_stock is true.
    
</dd>
</dl>

<dl>
<dd>

**strike_through_initial_price:** `Option<Option<f64>>` — A comparison price displayed with a strikethrough for the initial price.
    
</dd>
</dl>

<dl>
<dd>

**strike_through_renewal_price:** `Option<Option<f64>>` — A comparison price displayed with a strikethrough for the renewal price.
    
</dd>
</dl>

<dl>
<dd>

**three_ds_level:** `Option<Option<UpdatePlansRequestThreeDsLevel>>` — 3D Secure behavior for this plan. Send `null` to inherit the account default.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<Option<String>>` — The display name of the plan shown to customers on the product page.
    
</dd>
</dl>

<dl>
<dd>

**trial_period_days:** `Option<Option<i64>>` — Free trial duration before the first recurring charge.
    
</dd>
</dl>

<dl>
<dd>

**unlimited_stock:** `Option<Option<bool>>` — Whether the plan has unlimited stock. When true, the stock field is ignored.
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<String>` — Whether the plan is visible to customers or hidden from public view.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.plans.<a href="/src/api/resources/plans/client.rs">calculate_tax</a>(id: String, request: CalculateTaxPlansRequest) -> Result&lt;CalculateTaxPlansResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Previews tax for a plan before checkout, based on the buyer's location.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .plans
        .calculate_tax(
            &"id".to_string(),
            &CalculateTaxPlansRequest {
                address: Some(CalculateTaxPlansRequestAddress {
                    country: "DE".to_string(),
                    postal_code: Some("10115".to_string()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Plan ID, prefixed `plan_`.
    
</dd>
</dl>

<dl>
<dd>

**address:** `Option<Option<CalculateTaxPlansRequestAddress>>` — Buyer billing address used for tax calculation. Provide either `address.country` or `ip_address`; include state and postal code when available for more accurate results.
    
</dd>
</dl>

<dl>
<dd>

**ip_address:** `Option<String>` — Buyer IP address used to infer location when no billing address is provided.
    
</dd>
</dl>

<dl>
<dd>

**tax_ids:** `Option<Option<Vec<CalculateTaxPlansRequestTaxIdsItem>>>` — Optional buyer tax ID for B2B exemptions. At most one entry is supported.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Products
<details><summary><code>client.products.<a href="/src/api/resources/products/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, query: Option&lt;Option&lt;String&gt;&gt;, marketplace_category_route: Option&lt;Option&lt;String&gt;&gt;, price_minimum: Option&lt;Option&lt;f64&gt;&gt;, price_maximum: Option&lt;Option&lt;f64&gt;&gt;, direction: Option&lt;Option&lt;ListProductsRequestDirection&gt;&gt;, order: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListProductsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of products. Omit `account_id` to search the public marketplace.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .products
        .list(
            &ProductsListQueryRequest {
                visibilities: vec![Some("visible".to_string())],
                access_pass_types: vec![Some("regular".to_string())],
                account_id: None,
                query: None,
                marketplace_category_route: None,
                plan_types: vec![],
                price_minimum: None,
                price_maximum: None,
                labels: vec![],
                direction: None,
                order: None,
                first: None,
                after: None,
                last: None,
                before: None,
                created_after: None,
                created_before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The unique identifier of the account to list products for. Omit to search the public marketplace.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Ranked search against product title and headline. Omit to browse by recency.
    
</dd>
</dl>

<dl>
<dd>

**marketplace_category_route:** `Option<String>` — Only return marketplace products assigned to this category route, such as `trading`.
    
</dd>
</dl>

<dl>
<dd>

**plan_types:** `Option<ListProductsRequestPlanTypesItem>` — Filter to products with a buyable plan of these billing models, such as `one_time` or `renewal`.
    
</dd>
</dl>

<dl>
<dd>

**price_minimum:** `Option<f64>` — Only return products whose advertised buyable plan has a displayed price of at least this amount. Recurring plans use renewal price.
    
</dd>
</dl>

<dl>
<dd>

**price_maximum:** `Option<f64>` — Only return products whose advertised buyable plan has a displayed price of at most this amount. Recurring plans use renewal price.
    
</dd>
</dl>

<dl>
<dd>

**visibilities:** `Option<String>` — Filter to only products matching these visibility states. Ignored on the public marketplace list, which only returns visible products.
    
</dd>
</dl>

<dl>
<dd>

**access_pass_types:** `Option<String>` — Filter to only products matching these types.
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<String>` — Filter to only products carrying all of these labels. Labels are matched lowercased.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListProductsRequestDirection>` — The sort direction for results. Defaults to descending.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<String>` — The field to sort results by. Account lists default to `created_at`. Marketplace lists default to `discoverable_at` and accept `created_at` or `discoverable_at`. Cannot be combined with `query`.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of products to return (default and max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns products after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of products to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns products before this position.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return products created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return products created before this ISO 8601 timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.products.<a href="/src/api/resources/products/client.rs">create</a>(request: CreateProductsRequest) -> Result&lt;Product, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new product for an account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .products
        .create(
            &CreateProductsRequest {
                title: "Interior Deep Clean".to_string(),
                account_id: None,
                collect_shipping_address: None,
                custom_cta: None,
                custom_cta_url: None,
                custom_statement_descriptor: None,
                description: None,
                global_affiliate_percentage: None,
                global_affiliate_status: None,
                headline: None,
                labels: None,
                member_affiliate_percentage: None,
                member_affiliate_status: None,
                metadata: None,
                product_tax_code_id: None,
                redirect_purchase_url: None,
                route: None,
                send_welcome_message: None,
                visibility: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The unique identifier of the account to create this product for.
    
</dd>
</dl>

<dl>
<dd>

**collect_shipping_address:** `Option<Option<bool>>` — Whether to collect a shipping address at checkout.
    
</dd>
</dl>

<dl>
<dd>

**custom_cta:** `Option<Option<CreateProductsRequestCustomCta>>` — The call-to-action button label.
    
</dd>
</dl>

<dl>
<dd>

**custom_cta_url:** `Option<Option<String>>` — A URL the call-to-action button links to.
    
</dd>
</dl>

<dl>
<dd>

**custom_statement_descriptor:** `Option<Option<String>>` — Custom bank statement descriptor. Must start with WHOP*.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — A written description displayed on the product page.
    
</dd>
</dl>

<dl>
<dd>

**global_affiliate_percentage:** `Option<Option<f64>>` — The commission rate affiliates earn.
    
</dd>
</dl>

<dl>
<dd>

**global_affiliate_status:** `Option<CreateProductsRequestGlobalAffiliateStatus>` — The enrollment status in the global affiliate program.
    
</dd>
</dl>

<dl>
<dd>

**headline:** `Option<Option<String>>` — A short marketing headline for the product page.
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<Option<Vec<String>>>` — Labels used to group products into collections. Stored lowercased and de-duplicated. Maximum 20 labels, 50 characters each.
    
</dd>
</dl>

<dl>
<dd>

**member_affiliate_percentage:** `Option<Option<f64>>` — The commission rate members earn.
    
</dd>
</dl>

<dl>
<dd>

**member_affiliate_status:** `Option<CreateProductsRequestMemberAffiliateStatus>` — The enrollment status in the member affiliate program.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Custom key-value pairs to store on the product.
    
</dd>
</dl>

<dl>
<dd>

**product_tax_code_id:** `Option<Option<String>>` — The unique identifier of the tax classification code. See the available [product categories](https://docs.numeral.com/essentials/product-categories).
    
</dd>
</dl>

<dl>
<dd>

**redirect_purchase_url:** `Option<Option<String>>` — A URL to redirect the customer to after purchase.
    
</dd>
</dl>

<dl>
<dd>

**route:** `Option<Option<String>>` — The URL slug for the product's public link.
    
</dd>
</dl>

<dl>
<dd>

**send_welcome_message:** `Option<Option<bool>>` — Whether to send an automated welcome message via support chat when a user joins this product. Defaults to true.
    
</dd>
</dl>

<dl>
<dd>

**title:** `String` — The display name of the product. Maximum 80 characters.
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<String>` — Whether the product is visible to customers.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.products.<a href="/src/api/resources/products/client.rs">retrieve</a>(id: String) -> Result&lt;Product, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a product. Public — no credentials.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.products.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the product.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.products.<a href="/src/api/resources/products/client.rs">delete</a>(id: String) -> Result&lt;DeleteProductsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a product. Only products with no memberships, entries, reviews, or invoices can be deleted.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.products.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the product.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.products.<a href="/src/api/resources/products/client.rs">update</a>(id: String, request: UpdateProductsRequest) -> Result&lt;Product, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an existing product.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .products
        .update(
            &"id".to_string(),
            &UpdateProductsRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the product.
    
</dd>
</dl>

<dl>
<dd>

**banner_image:** `Option<Option<UpdateProductsRequestBannerImage>>` — A wide image for the product, shown on the product page and on listing cards. Pass `{ id }` for an existing attachment or `{ direct_upload_id }` for a completed direct upload; `null` removes it.
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<Option<String>>` — A written description displayed on the product page.
    
</dd>
</dl>

<dl>
<dd>

**headline:** `Option<Option<String>>` — A short marketing headline for the product page.
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<Option<Vec<String>>>` — Labels used to group products into collections. Replaces the existing labels. Send an empty array to clear them.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Custom key-value pairs to store on the product.
    
</dd>
</dl>

<dl>
<dd>

**product_tax_code_id:** `Option<Option<String>>` — The unique identifier of the tax classification code. See the available [product categories](https://docs.numeral.com/essentials/product-categories).
    
</dd>
</dl>

<dl>
<dd>

**send_welcome_message:** `Option<Option<bool>>` — Whether to send an automated welcome message via support chat when a user joins this product.
    
</dd>
</dl>

<dl>
<dd>

**title:** `Option<String>` — The display name of the product.
    
</dd>
</dl>

<dl>
<dd>

**visibility:** `Option<String>` — Whether the product is visible to customers.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.products.<a href="/src/api/resources/products/client.rs">publish</a>(id: String) -> Result&lt;Product, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Submits a product to the whop.com marketplace for review. The product moves to `pending_review`; a Whop reviewer approves it before it goes live.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.products.publish(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the product, prefixed `prod_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.products.<a href="/src/api/resources/products/client.rs">unpublish</a>(id: String) -> Result&lt;Product, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Removes a product from the whop.com marketplace. The product moves to `not_available`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.products.unpublish(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the product, prefixed `prod_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Promo Codes
<details><summary><code>client.promo_codes.<a href="/src/api/resources/promo_codes/client.rs">list</a>(account_id: Option&lt;String&gt;, status: Option&lt;Option&lt;ListPromoCodesRequestStatus&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListPromoCodesRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListPromoCodesRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListPromoCodesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists promo codes for an account with cursor pagination, filters, and sorting.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .promo_codes
        .list(
            &PromoCodesListQueryRequest {
                account_id: "account_id".to_string(),
                product_ids: vec![Some("prod_xxxxxxxxxxxxxx".to_string())],
                plan_ids: vec![Some("plan_xxxxxxxxxxxxxx".to_string())],
                status: None,
                created_before: None,
                created_after: None,
                order: None,
                direction: None,
                first: None,
                after: None,
                last: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Account whose promo codes are listed (`biz_` tag).
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListPromoCodesRequestStatus>` — Promo-code status. `expired` groups inactive and archived codes.
    
</dd>
</dl>

<dl>
<dd>

**product_ids:** `Option<String>` — Only promo codes scoped to these product IDs.
    
</dd>
</dl>

<dl>
<dd>

**plan_ids:** `Option<String>` — Only promo codes scoped to these plan IDs.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only promo codes created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only promo codes created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListPromoCodesRequestOrder>` — Sort field.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListPromoCodesRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of promo codes to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to paginate forwards from.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of promo codes to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to paginate backwards from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.promo_codes.<a href="/src/api/resources/promo_codes/client.rs">create</a>(request: CreatePromoCodesRequest) -> Result&lt;PromoCode, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a promo code for an account. First-party sessions may attach an affiliate.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .promo_codes
        .create(
            &CreatePromoCodesRequest {
                account_id: "biz_xxxxxxxxxxxxxx".to_string(),
                amount_off: 25.0,
                base_currency: CreatePromoCodesRequestBaseCurrency::Usd,
                code: "AFFILIATE25".to_string(),
                new_users_only: true,
                promo_duration_months: 3,
                promo_type: CreatePromoCodesRequestPromoType::Percentage,
                churned_users_only: None,
                existing_memberships_only: None,
                expires_at: None,
                one_per_customer: None,
                plan_ids: None,
                product_id: None,
                stock: None,
                unlimited_stock: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**amount_off:** `f64` 
    
</dd>
</dl>

<dl>
<dd>

**base_currency:** `CreatePromoCodesRequestBaseCurrency` 
    
</dd>
</dl>

<dl>
<dd>

**churned_users_only:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**existing_memberships_only:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**expires_at:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**new_users_only:** `bool` 
    
</dd>
</dl>

<dl>
<dd>

**one_per_customer:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**plan_ids:** `Option<Vec<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**promo_duration_months:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**promo_type:** `CreatePromoCodesRequestPromoType` 
    
</dd>
</dl>

<dl>
<dd>

**stock:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**unlimited_stock:** `Option<bool>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.promo_codes.<a href="/src/api/resources/promo_codes/client.rs">retrieve</a>(id: String) -> Result&lt;PromoCode, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a promo code by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.promo_codes.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Promo code ID (`promo_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.promo_codes.<a href="/src/api/resources/promo_codes/client.rs">delete</a>(id: String) -> Result&lt;DeletePromoCodesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Archives a promo code so it cannot be used in future checkouts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.promo_codes.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Promo code ID (`promo_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.promo_codes.<a href="/src/api/resources/promo_codes/client.rs">activate</a>(id: String) -> Result&lt;PromoCode, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Turns an inactive promo code back on so it can be redeemed at checkout.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.promo_codes.activate(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Promo code ID (`promo_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.promo_codes.<a href="/src/api/resources/promo_codes/client.rs">deactivate</a>(id: String) -> Result&lt;PromoCode, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Turns off an active promo code so it can no longer be redeemed at checkout.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.promo_codes.deactivate(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Promo code ID (`promo_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Reactions
<details><summary><code>client.reactions.<a href="/src/api/resources/reactions/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, resource_id: Option&lt;String&gt;) -> Result&lt;ListReactionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of emoji reactions on a specific message or forum post, sorted by most recent.

Required permissions (one of):
 - `chat:read`
 - `dms:read`
 - `forum:read`
 - `livestream:chat:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .reactions
        .list(
            &ReactionsListQueryRequest {
                first: Some(42),
                last: Some(42),
                resource_id: "resource_id".to_string(),
                after: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**resource_id:** `String` — The unique identifier of the message or forum post to list reactions for.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reactions.<a href="/src/api/resources/reactions/client.rs">create</a>(request: CreateReactionsRequest) -> Result&lt;Reaction, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add an emoji reaction or poll vote to a message or forum post. In forums, the reaction is always a like.

Required permissions (one of):
 - `chat:read`
 - `dms:read`
 - `forum:read`
 - `livestream:chat:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .reactions
        .create(
            &CreateReactionsRequest {
                resource_id: "resource_id".to_string(),
                emoji: None,
                poll_option_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**emoji:** `Option<Option<String>>` — The emoji to react with, in shortcode or unicode format. For example, ':heart:' or a unicode emoji. Ignored in forums where reactions are always likes.
    
</dd>
</dl>

<dl>
<dd>

**poll_option_id:** `Option<Option<String>>` — The unique identifier of a poll option to vote for. Only valid when the target message or post contains a poll.
    
</dd>
</dl>

<dl>
<dd>

**resource_id:** `String` — The unique identifier of the message or forum post to react to.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reactions.<a href="/src/api/resources/reactions/client.rs">retrieve</a>(id: String) -> Result&lt;Reaction, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing reaction.

Required permissions (one of):
 - `chat:read`
 - `dms:read`
 - `forum:read`
 - `livestream:chat:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .reactions
        .retrieve(&"reac_xxxxxxxxxxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the reaction to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reactions.<a href="/src/api/resources/reactions/client.rs">delete</a>(id: String, emoji: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove an emoji reaction from a message or forum post. Only the reaction author or a channel admin can remove a reaction.

Required permissions (one of):
 - `chat:read`
 - `dms:read`
 - `forum:read`
 - `livestream:chat:read`
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .reactions
        .delete(
            &"reac_xxxxxxxxxxxxxxxxxxxxxx".to_string(),
            &ReactionsDeleteQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the reaction to remove, or the identifier of the message or forum post to remove a reaction from. When passing a message or post ID, you must also provide the emoji argument.
    
</dd>
</dl>

<dl>
<dd>

**emoji:** `Option<String>` — The emoji to remove, in shortcode or unicode format. For example, ':heart:' or a unicode emoji. Required when the id refers to a message or post instead of a reaction.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Recommended Actions
<details><summary><code>client.recommended_actions.<a href="/src/api/resources/recommended_actions/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListRecommendedActionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the recommended action chains for an account — short sequences of actions (create a product, price it, publish it) the account should run next, gated on what it already has.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .recommended_actions
        .list(
            &RecommendedActionsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Account ID, prefixed `biz_`. Defaults to the API key's own account.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.recommended_actions.<a href="/src/api/resources/recommended_actions/client.rs">retrieve</a>(id: String, account_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;AccountRecommendedActionChain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a recommended action chain by id, including chains that have already been run. Seeded chains are reconstructed from their hard-coded chain; generated chains are read from the account's stored chain, with each step's filled-in input.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .recommended_actions
        .retrieve(
            &"id".to_string(),
            &RecommendedActionsRetrieveQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Chain ID from the list endpoint, e.g. `rac_seed_start_selling_9f2c1a7b04`.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — Account ID, prefixed `biz_`. Defaults to the API key's own account.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.recommended_actions.<a href="/src/api/resources/recommended_actions/client.rs">run</a>(id: String, account_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;RunRecommendedActionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Records that the caller ran a recommended action chain. Nothing is executed server-side yet — the client follows the chain's step CTAs itself; this writes the `recommended_action_chain.executed` analytics event.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .recommended_actions
        .run(
            &"id".to_string(),
            &RunQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Chain ID from the list endpoint, e.g. `rac_seed_start_selling_9f2c1a7b04`.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — Account ID, prefixed `biz_`. Defaults to the API key's own account.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.recommended_actions.<a href="/src/api/resources/recommended_actions/client.rs">list_executions</a>(id: String, account_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListExecutionsRecommendedActionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the per-step record of a recommended action chain the server ran — one entry per step in position order, each carrying its current status and, once the step completed, the API response it produced. A chain that was never run server-side returns an empty list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .recommended_actions
        .list_executions(
            &"id".to_string(),
            &ListExecutionsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Chain ID from the list endpoint.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — Account ID, prefixed `biz_`. Defaults to the API key's own account.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Refunds
<details><summary><code>client.refunds.<a href="/src/api/resources/refunds/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, payment_id: Option&lt;Option&lt;String&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, direction: Option&lt;Option&lt;Direction&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListRefundsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of refunds, with optional filtering by payment, company, user, and creation date.

Required permissions:
 - `payment:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .refunds
        .list(
            &RefundsListQueryRequest {
                first: Some(42),
                last: Some(42),
                payment_id: Some("pay_xxxxxxxxxxxxxx".to_string()),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                user_id: Some("user_xxxxxxxxxxxxx".to_string()),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**payment_id:** `Option<String>` — Filter refunds to only those associated with this specific payment.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — Filter refunds to only those belonging to this company.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Filter refunds to only those associated with this specific user.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<Direction>` 
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return refunds created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return refunds created after this timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.refunds.<a href="/src/api/resources/refunds/client.rs">retrieve</a>(id: String) -> Result&lt;Refund, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing refund.

Required permissions:
 - `payment:basic:read`
 - `plan:basic:read`
 - `access_pass:basic:read`
 - `member:email:read`
 - `member:basic:read`
 - `member:phone:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .refunds
        .retrieve(&"rf_xxxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the refund.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Resolution Center Cases
<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListResolutionCenterCasesRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListResolutionCenterCasesRequestDirection&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListResolutionCenterCasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists resolution center cases. Without `account_id` you get every case you can read — the ones you opened as a buyer and every account you are a team member of; the filters narrow that list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .list(
            &ResolutionCenterCasesListQueryRequest {
                account_id: None,
                user_id: None,
                first: None,
                after: None,
                last: None,
                before: None,
                order: None,
                direction: None,
                status: vec![],
                reason: vec![],
                outcome: vec![],
                created_before: None,
                created_after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Only cases filed against this account (`biz_` tag). With read access to the account this lists its whole queue; without, only the cases you opened against it.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Only cases opened by this customer — a `user_` tag, or `me` for the calling user. It narrows what you can already read, so `me` lists the cases you opened without the ones on accounts you are a team member of.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of cases to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns cases after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of cases to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns cases before this position.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListResolutionCenterCasesRequestOrder>` — The field to sort cases by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListResolutionCenterCasesRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListResolutionCenterCasesRequestStatusItem>` — Only cases in these statuses. Repeat the parameter to pass several — one paginated list covers all of them.
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<ListResolutionCenterCasesRequestReasonItem>` — Only cases opened for these reasons. Repeat the parameter to pass several.
    
</dd>
</dl>

<dl>
<dd>

**outcome:** `Option<ListResolutionCenterCasesRequestOutcomeItem>` — Only closed cases that ended these ways. Repeat the parameter to pass several.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only cases created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only cases created after this ISO 8601 timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">create</a>(request: CreateResolutionCenterCasesRequest) -> Result&lt;ResolutionCenterCase, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Opens a case, as the customer, against one of your own payments. Provide the payment (`receipt_id`), the `reason`, and a `message`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .create(
            &CreateResolutionCenterCasesRequest {
                message: "The mobile detailer never showed up for the Ceramic Coating appointment."
                    .to_string(),
                reason: CreateResolutionCenterCasesRequestReason::Fraudulent,
                receipt_id: "pay_xxxxxxxxxxxxxx".to_string(),
                attachments: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attachments:** `Option<Vec<CreateResolutionCenterCasesRequestAttachmentsItem>>` 
    
</dd>
</dl>

<dl>
<dd>

**message:** `String` — The customer's explanation.
    
</dd>
</dl>

<dl>
<dd>

**reason:** `CreateResolutionCenterCasesRequestReason` — What went wrong. Uses the same vocabulary as `/disputes`.
    
</dd>
</dl>

<dl>
<dd>

**receipt_id:** `String` — The payment to open the case against (`pay_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">summary</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;SummaryResolutionCenterCasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Aggregates the same cases `GET /resolution_center_cases` lists, using the same filters. Use it to build status tabs and issue filters without paging the whole list.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .summary(
            &ResolutionCenterCasesSummaryQueryRequest {
                groups: vec![],
                account_id: None,
                user_id: None,
                status: vec![],
                reason: vec![],
                outcome: vec![],
                created_before: None,
                created_after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**groups:** `Option<SummaryResolutionCenterCasesRequestGroupsItem>` — Which breakdowns to return, keyed by these names under `groups`. Repeat the parameter to ask for several; omit it for all of them.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — The account to summarize cases for (`biz_` tag).
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Only cases opened by this customer — a `user_` tag, or `me` for the calling user.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<SummaryResolutionCenterCasesRequestStatusItem>` — Only cases in these statuses.
    
</dd>
</dl>

<dl>
<dd>

**reason:** `Option<SummaryResolutionCenterCasesRequestReasonItem>` — Only cases opened for these reasons.
    
</dd>
</dl>

<dl>
<dd>

**outcome:** `Option<SummaryResolutionCenterCasesRequestOutcomeItem>` — Only closed cases that ended these ways.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only count cases created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only count cases created after this ISO 8601 timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">retrieve</a>(id: String) -> Result&lt;ResolutionCenterCase, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single resolution center case with its full event timeline.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .retrieve(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The resolution center case ID (`reso_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">accept</a>(id: String, request: AcceptResolutionCenterCasesRequest) -> Result&lt;ResolutionCenterCase, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Accepts the case in the customer's favor, as the merchant: refunds the payment in full and closes the case.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .accept(
            &"id".to_string(),
            &AcceptResolutionCenterCasesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The resolution center case ID (`reso_` tag).
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Vec<AcceptResolutionCenterCasesRequestAttachmentsItem>>` — Up to 3 evidence files, by existing file `id` or `direct_upload_id`.
    
</dd>
</dl>

<dl>
<dd>

**message:** `Option<String>` — An optional note to the customer, recorded on the case timeline.
    
</dd>
</dl>

<dl>
<dd>

**terminate_membership:** `Option<bool>` — Whether to also terminate the customer's membership.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">appeal</a>(id: String, request: AppealResolutionCenterCasesRequest) -> Result&lt;ResolutionCenterCase, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Appeals a decision, as the customer, on a case that closed in the merchant's favor. Escalates the case to Whop for platform review. A case can be appealed once.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .appeal(
            &"id".to_string(),
            &AppealResolutionCenterCasesRequest {
                message: "The coating is already flaking on the hood two weeks later.".to_string(),
                attachments: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The resolution center case ID (`reso_` tag).
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Vec<AppealResolutionCenterCasesRequestAttachmentsItem>>` — Up to 3 evidence files, by existing file `id` or `direct_upload_id`.
    
</dd>
</dl>

<dl>
<dd>

**message:** `String` — Why you are appealing the decision.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">deny</a>(id: String, request: DenyResolutionCenterCasesRequest) -> Result&lt;ResolutionCenterCase, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Denies the case, as the merchant: rejects the claim and closes the case with no refund.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .deny(
            &"id".to_string(),
            &DenyResolutionCenterCasesRequest {
                message:
                    "The ceramic coating was applied and the vehicle was collected on 2026-01-05."
                        .to_string(),
                attachments: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The resolution center case ID (`reso_` tag).
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Vec<DenyResolutionCenterCasesRequestAttachmentsItem>>` — Up to 3 evidence files, by existing file `id` or `direct_upload_id`.
    
</dd>
</dl>

<dl>
<dd>

**message:** `String` — Why the claim is being denied. Shown to the customer.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">events</a>(id: String, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;EventsResolutionCenterCasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the case timeline, newest first. Events the viewer is not allowed to see are omitted — a customer reads the customer-visible timeline, the merchant reads the full one.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .events(
            &"id".to_string(),
            &EventsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The resolution center case ID (`reso_` tag).
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of events to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns events after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of events to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns events before this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">reply</a>(id: String, request: ReplyResolutionCenterCasesRequest) -> Result&lt;ResolutionCenterCase, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Replies to an open request for information on the case. As the merchant this answers Whop's request (valid while the case awaits your information); as the customer it provides the information requested from you. The actor is resolved from the credential.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .reply(
            &"id".to_string(),
            &ReplyResolutionCenterCasesRequest {
                message: "Here are the before and after photos from the Burnet Rd bay.".to_string(),
                attachments: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The resolution center case ID (`reso_` tag).
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Vec<ReplyResolutionCenterCasesRequestAttachmentsItem>>` — Up to 3 evidence files, by existing file `id` or `direct_upload_id`.
    
</dd>
</dl>

<dl>
<dd>

**message:** `String` — The reply to add to the case.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">request_info</a>(id: String, request: RequestInfoResolutionCenterCasesRequest) -> Result&lt;ResolutionCenterCase, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Asks the customer for more information, as the merchant. Allowed up to 3 times per case before you must accept or deny it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .request_info(
            &"id".to_string(),
            &RequestInfoResolutionCenterCasesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The resolution center case ID (`reso_` tag).
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Vec<RequestInfoResolutionCenterCasesRequestAttachmentsItem>>` — Up to 3 evidence files, by existing file `id` or `direct_upload_id`.
    
</dd>
</dl>

<dl>
<dd>

**message:** `Option<String>` — What you need from the customer.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.resolution_center_cases.<a href="/src/api/resources/resolution_center_cases/client.rs">withdraw</a>(id: String) -> Result&lt;ResolutionCenterCase, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Withdraws (cancels) a case you opened, as the customer. Only possible while the case is still open.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .resolution_center_cases
        .withdraw(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The resolution center case ID (`reso_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Reviews
<details><summary><code>client.reviews.<a href="/src/api/resources/reviews/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, product_id: Option&lt;String&gt;, min_stars: Option&lt;Option&lt;i64&gt;&gt;, max_stars: Option&lt;Option&lt;i64&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListReviewsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of customer reviews for a specific product, with optional filtering by star rating and creation date.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .reviews
        .list(
            &ReviewsListQueryRequest {
                first: Some(42),
                last: Some(42),
                product_id: "prod_xxxxxxxxxxxxx".to_string(),
                min_stars: Some(42),
                max_stars: Some(42),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                after: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**product_id:** `String` — The unique identifier of the product to list reviews for.
    
</dd>
</dl>

<dl>
<dd>

**min_stars:** `Option<i64>` — The minimum star rating to include in results, from 1 to 5 inclusive.
    
</dd>
</dl>

<dl>
<dd>

**max_stars:** `Option<i64>` — The maximum star rating to include in results, from 1 to 5 inclusive.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return reviews created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return reviews created after this timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.reviews.<a href="/src/api/resources/reviews/client.rs">retrieve</a>(id: String) -> Result&lt;Review, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing review.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .reviews
        .retrieve(&"rev_xxxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the review to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Setup Intents
<details><summary><code>client.setup_intents.<a href="/src/api/resources/setup_intents/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;String&gt;, direction: Option&lt;Option&lt;Direction&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListSetupIntentsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of setup intents for a company, with optional filtering by creation date. A setup intent securely collects and stores a member's payment method for future use without charging them immediately.

Required permissions:
 - `payment:setup_intent:read`
 - `member:basic:read`
 - `member:email:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .setup_intents
        .list(
            &SetupIntentsListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                after: None,
                before: None,
                direction: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to list setup intents for.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<Direction>` 
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return setup intents created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return setup intents created after this timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.setup_intents.<a href="/src/api/resources/setup_intents/client.rs">create</a>(request: CreateSetupIntentsRequestBody) -> Result&lt;CreateSetupIntentsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Save a buyer's payment method for later without charging it. Provide a confirmation token for a method the buyer just supplied, or an existing payment method to re-verify. The buyer may still have a step to complete — 3D Secure, a hosted enrollment, linking a bank account — so poll the setup intent's status endpoint for what to do next.

Required permissions:
 - `payment:charge`
 - `member:basic:read`
 - `member:email:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .setup_intents
        .create(
            &CreateSetupIntentsRequestBody::CreateSetupIntentsRequestBodyConfirmationToken(
                CreateSetupIntentsRequestBodyConfirmationToken {
                    company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                    confirmation_token: "ctok_xxxxxxxxxxxxxx".to_string(),
                    ..Default::default()
                },
            ),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.setup_intents.<a href="/src/api/resources/setup_intents/client.rs">retrieve</a>(id: String) -> Result&lt;SetupIntent, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing setup intent.

Required permissions:
 - `payment:setup_intent:read`
 - `member:basic:read`
 - `member:email:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .setup_intents
        .retrieve(&"sint_xxxxxxxxxxxxx".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the setup intent.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.setup_intents.<a href="/src/api/resources/setup_intents/client.rs">update_return_url</a>(setup_intent_id: String, request: UpdateReturnUrlSetupIntentsRequest) -> Result&lt;SetupStatus, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Changes where the buyer lands after completing an off-site step, up until they return. Accepts either a secret key or the setup's own `client_secret`, so the surface that knows the final destination can set it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .setup_intents
        .update_return_url(
            &"setup_intent_id".to_string(),
            &UpdateReturnURLSetupIntentsRequest {
                return_url: "https://shinetime.example/checkout/thanks".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**setup_intent_id:** `String` — The unique identifier of the setup intent.
    
</dd>
</dl>

<dl>
<dd>

**return_url:** `String` — Where the buyer continues after completing an off-site step. Must be an absolute https URL without credentials (http is allowed for localhost), at most 2,048 characters.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.setup_intents.<a href="/src/api/resources/setup_intents/client.rs">retrieve_status</a>(setup_intent_id: String) -> Result&lt;SetupStatus, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves how far a setup has got and what the buyer must do next, if anything. Collection runs in the background, so poll this rather than reading the create response. Accepts either a secret key or the setup's own `client_secret`, so the surface collecting the payment method can poll it directly.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .setup_intents
        .retrieve_status(&"setup_intent_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**setup_intent_id:** `String` — The unique identifier of the setup intent.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Shipments
<details><summary><code>client.shipments.<a href="/src/api/resources/shipments/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListShipmentsRequestStatus&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListShipmentsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListShipmentsRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListShipmentsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of shipments for an account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .shipments
        .list(
            &ShipmentsListQueryRequest {
                payment_id: vec![Some("pay_xxxxxxxxxxxxxx".to_string())],
                account_id: None,
                status: None,
                created_before: None,
                created_after: None,
                order: None,
                direction: None,
                first: None,
                after: None,
                last: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The account to list shipments for. Defaults to the acting account.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListShipmentsRequestStatus>` — Filter to shipments with this delivery status.
    
</dd>
</dl>

<dl>
<dd>

**payment_id:** `Option<String>` — Only shipments fulfilling these payments, each prefixed `pay_`. Repeat the parameter to pass several, up to 100 per request — one paginated list covers all of them.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Return shipments created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Return shipments created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListShipmentsRequestOrder>` — The field to sort by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListShipmentsRequestDirection>` — The sort direction.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of shipments to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns shipments after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of shipments to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns shipments before this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.shipments.<a href="/src/api/resources/shipments/client.rs">create</a>(request: CreateShipmentsRequest) -> Result&lt;Shipment, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Attaches a carrier tracking number to a payment and begins tracking it.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .shipments
        .create(
            &CreateShipmentsRequest {
                payment_id: "pay_xxxxxxxxxxxxxx".to_string(),
                tracking_number: "1Z999AA10123456784".to_string(),
                account_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The unique identifier of the account, prefixed `biz_`.
    
</dd>
</dl>

<dl>
<dd>

**payment_id:** `String` — The payment to attach the shipment to, prefixed `pay_`.
    
</dd>
</dl>

<dl>
<dd>

**tracking_number:** `String` — The carrier-assigned tracking number.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.shipments.<a href="/src/api/resources/shipments/client.rs">retrieve</a>(id: String) -> Result&lt;Shipment, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a shipment by its id, or by the payment id it fulfills.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.shipments.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The shipment id (`ship_`), or the payment id (`pay_`) it fulfills.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.shipments.<a href="/src/api/resources/shipments/client.rs">update</a>(id: String, request: UpdateShipmentsRequest) -> Result&lt;Shipment, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a shipment's tracking number and re-tracks it with the carrier.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .shipments
        .update(
            &"id".to_string(),
            &UpdateShipmentsRequest {
                tracking_number: "9400111899223456789012".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The shipment id (`ship_`), or the payment id (`pay_`) it fulfills.
    
</dd>
</dl>

<dl>
<dd>

**tracking_number:** `String` — The new carrier-assigned tracking number.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Social Accounts
<details><summary><code>client.social_accounts.<a href="/src/api/resources/social_accounts/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, platform: Option&lt;Option&lt;ListSocialAccountsRequestPlatform&gt;&gt;, verified: Option&lt;Option&lt;bool&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListSocialAccountsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListSocialAccountsRequestDirection&gt;&gt;) -> Result&lt;ListSocialAccountsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the social accounts linked to an account or user.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .social_accounts
        .list(
            &SocialAccountsListQueryRequest {
                account_id: None,
                user_id: None,
                platform: None,
                verified: None,
                scopes: vec![],
                first: None,
                after: None,
                last: None,
                before: None,
                order: None,
                direction: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The Account that the social accounts are connected to. Provide either this or user_id.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The User that the social accounts are connected to. Provide either this or account_id.
    
</dd>
</dl>

<dl>
<dd>

**platform:** `Option<ListSocialAccountsRequestPlatform>` — Only return social accounts for the platform that is specified.
    
</dd>
</dl>

<dl>
<dd>

**verified:** `Option<bool>` — Only return social accounts that are verified on the platform.
    
</dd>
</dl>

<dl>
<dd>

**scopes:** `Option<ListSocialAccountsRequestScopesItem>` — Only return social accounts that have these scopes.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of social accounts to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of social accounts to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to fetch the page before (from page_info.start_cursor).
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListSocialAccountsRequestOrder>` — The field to sort social accounts by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListSocialAccountsRequestDirection>` — Sort direction.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.social_accounts.<a href="/src/api/resources/social_accounts/client.rs">create</a>(request: CreateSocialAccountsRequest) -> Result&lt;SocialAccount, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates or returns a Whop-managed Facebook page for an account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .social_accounts
        .create(
            &CreateSocialAccountsRequest {
                platform: CreateSocialAccountsRequestPlatform::Facebook,
                account_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The Account (biz_ identifier) to create the social account for. An account-scoped API key may omit this to default to its own account. Account API keys cannot update their own account's branding through Update Account; use a user-authenticated path.
    
</dd>
</dl>

<dl>
<dd>

**platform:** `CreateSocialAccountsRequestPlatform` — The platform to create the social account on. `facebook` requires the account's `banner_image`, `logo`, and `description`; configure them with [Update Account](/api-reference/beta/accounts/update-account).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.social_accounts.<a href="/src/api/resources/social_accounts/client.rs">connect</a>(request: ConnectSocialAccountsRequest) -> Result&lt;ConnectSocialAccountsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Starts an OAuth connection flow and returns an authorize_url where the user can connect a social account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .social_accounts
        .connect(
            &ConnectSocialAccountsRequest {
                platform: ConnectSocialAccountsRequestPlatform::MetaBusiness,
                account_id: None,
                redirect_url: None,
                scopes: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The Account (biz_ identifier) to connect the social account for. An account-scoped API key may omit this to default to its own account.
    
</dd>
</dl>

<dl>
<dd>

**platform:** `ConnectSocialAccountsRequestPlatform` — The platform to connect the social account on. Supported options are `meta_business` and `tiktok`.
    
</dd>
</dl>

<dl>
<dd>

**redirect_url:** `Option<String>` — The Whop URL to redirect the user to after they finish connecting.
    
</dd>
</dl>

<dl>
<dd>

**scopes:** `Option<Vec<ConnectSocialAccountsRequestScopesItem>>` — Capabilities to grant for the connected social account. Use `advertise` when connecting a Meta Business or TikTok account for ads.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.social_accounts.<a href="/src/api/resources/social_accounts/client.rs">delete</a>(id: String, account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;DeleteSocialAccountsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Disconnects a social account from an account or user without deleting the underlying platform account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .social_accounts
        .delete(
            &"id".to_string(),
            &SocialAccountsDeleteQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of the social account to disconnect.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — The Account that the social account is connected to. Provide either this or user_id.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The User that the social account is connected to. Provide either this or account_id.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.social_accounts.<a href="/src/api/resources/social_accounts/client.rs">lead_forms</a>(id: String, account_id: Option&lt;String&gt;) -> Result&lt;LeadFormsSocialAccountsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the active lead (instant) forms that already exist on a connected Facebook page, so an ad can reuse one as its `lead_gen_form_id` instead of authoring a new form. Every active form comes back in a single response — the list is not paginated.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .social_accounts
        .lead_forms(
            &"id".to_string(),
            &LeadFormsQueryRequest {
                account_id: "account_id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The social account (a sacc_ identifier) whose lead forms to list.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `String` — The Account (a biz_ identifier) the social account is connected to.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.social_accounts.<a href="/src/api/resources/social_accounts/client.rs">posts</a>(id: String, account_id: Option&lt;String&gt;, post_id: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;PostsSocialAccountsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the existing posts of a connected Facebook page, Instagram account, or TikTok account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .social_accounts
        .posts(
            &"id".to_string(),
            &PostsQueryRequest {
                account_id: "account_id".to_string(),
                post_id: None,
                first: None,
                after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The social account (a sacc_ identifier) whose posts to list.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `String` — The Account (a biz_ identifier) the social account is connected to.
    
</dd>
</dl>

<dl>
<dd>

**post_id:** `Option<String>` — Return only the single post with this platform id, instead of the full list.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of posts to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Stats
<details><summary><code>client.stats.<a href="/src/api/resources/stats/client.rs">list</a>() -> Result&lt;ListStatsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists every metric you can query, with its unit and the properties you can filter or break it down by.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.stats.list(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stats.<a href="/src/api/resources/stats/client.rs">describe_stats</a>(resource: Option&lt;Option&lt;String&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;DescribeStatsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Describe available stats schema. Without resource returns root nodes and metrics. With resource returns node columns, associations, and available metrics.

Required permissions:
 - `stats:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .stats
        .describe_stats(
            &DescribeStatsQueryRequest {
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                user_id: Some("user_xxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**resource:** `Option<String>` — Resource path using : as separator (e.g., 'receipts', 'payments:membership', 'receipts:gross_revenue').
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — Scope query to a specific company.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Scope query to a specific user.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stats.<a href="/src/api/resources/stats/client.rs">metric_stats</a>(resource: Option&lt;String&gt;, granularity: Option&lt;Option&lt;String&gt;&gt;, filters: Option&lt;Option&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;, from: Option&lt;Option&lt;String&gt;&gt;, to: Option&lt;Option&lt;String&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;MetricStatsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Query an aggregated metric. Returns data grouped by period with optional breakdowns.

Required permissions:
 - `stats:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .stats
        .metric_stats(
            &MetricStatsQueryRequest {
                resource: "resource".to_string(),
                from: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                to: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                user_id: Some("user_xxxxxxxxxxxxx".to_string()),
                granularity: None,
                breakdowns: vec![],
                filters: None,
                time_zone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**resource:** `String` — Metric resource using : as separator (e.g., 'receipts:gross_revenue', 'members:new_users').
    
</dd>
</dl>

<dl>
<dd>

**granularity:** `Option<String>` — Time granularity (daily, weekly, monthly).
    
</dd>
</dl>

<dl>
<dd>

**breakdowns:** `Option<String>` — Columns to break down the metric by.
    
</dd>
</dl>

<dl>
<dd>

**filters:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Key-value pairs to filter the data.
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — IANA timezone for period bucketing (e.g. 'America/New_York'). Defaults to UTC. Only applies to ClickHouse metrics.
    
</dd>
</dl>

<dl>
<dd>

**from:** `Option<String>` — Start of time range (unix timestamp).
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<String>` — End of time range (unix timestamp).
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — Scope query to a specific company.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Scope query to a specific user.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stats.<a href="/src/api/resources/stats/client.rs">raw_stats</a>(resource: Option&lt;String&gt;, from: Option&lt;Option&lt;String&gt;&gt;, to: Option&lt;Option&lt;String&gt;&gt;, limit: Option&lt;Option&lt;i64&gt;&gt;, cursor: Option&lt;Option&lt;String&gt;&gt;, sort: Option&lt;Option&lt;String&gt;&gt;, sort_direction: Option&lt;Option&lt;Direction&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;RawStatsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Query raw data from a resource. Returns paginated rows with all columns.

Required permissions:
 - `stats:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .stats
        .raw_stats(
            &RawStatsQueryRequest {
                resource: "resource".to_string(),
                from: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                to: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
                limit: Some(42),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                user_id: Some("user_xxxxxxxxxxxxx".to_string()),
                cursor: None,
                sort: None,
                sort_direction: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**resource:** `String` — Resource path using : as separator (e.g., 'members', 'payments:membership').
    
</dd>
</dl>

<dl>
<dd>

**from:** `Option<String>` — Start of time range (unix timestamp).
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<String>` — End of time range (unix timestamp).
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` — Number of records to return (max 10000).
    
</dd>
</dl>

<dl>
<dd>

**cursor:** `Option<String>` — Pagination cursor for next page.
    
</dd>
</dl>

<dl>
<dd>

**sort:** `Option<String>` — Column to sort by.
    
</dd>
</dl>

<dl>
<dd>

**sort_direction:** `Option<Direction>` 
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — Scope query to a specific company.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Scope query to a specific user.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stats.<a href="/src/api/resources/stats/client.rs">retrieve</a>(metric: String, account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, from: Option&lt;String&gt;, to: Option&lt;String&gt;, interval: Option&lt;Option&lt;RetrieveStatsRequestInterval&gt;&gt;, breakdown_by: Option&lt;Option&lt;String&gt;&gt;, convert_to: Option&lt;Option&lt;String&gt;&gt;, currency: Option&lt;Option&lt;String&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;, payment_method: Option&lt;Option&lt;String&gt;&gt;, card_network: Option&lt;Option&lt;String&gt;&gt;, dispute_reason: Option&lt;Option&lt;String&gt;&gt;, source: Option&lt;Option&lt;String&gt;&gt;, hostname: Option&lt;Option&lt;String&gt;&gt;, page: Option&lt;Option&lt;String&gt;&gt;, device_type: Option&lt;Option&lt;String&gt;&gt;, country_code: Option&lt;Option&lt;String&gt;&gt;, event_name: Option&lt;Option&lt;String&gt;&gt;, event_type: Option&lt;Option&lt;RetrieveStatsRequestEventType&gt;&gt;, custom_name: Option&lt;Option&lt;String&gt;&gt;, segment: Option&lt;Option&lt;String&gt;&gt;, category: Option&lt;Option&lt;String&gt;&gt;, merchant: Option&lt;Option&lt;String&gt;&gt;, fee_type: Option&lt;Option&lt;String&gt;&gt;, product: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;String&gt;&gt;, access_level: Option&lt;Option&lt;String&gt;&gt;, most_recent_action: Option&lt;Option&lt;String&gt;&gt;, referred_user_id: Option&lt;Option&lt;String&gt;&gt;, snapshot_window: Option&lt;Option&lt;RetrieveStatsRequestSnapshotWindow&gt;&gt;, event: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;RetrieveStatsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a metric as a time series of points for an account or user over a time range. The `market_prices` metric is public and requires no authentication.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .stats
        .retrieve(
            &"metric".to_string(),
            &StatsRetrieveQueryRequest {
                from: "from".to_string(),
                to: "to".to_string(),
                ad_campaign_ids: vec![Some("adcamp_xxxxxxxxxxxxxx".to_string())],
                ad_group_ids: vec![Some("adgrp_xxxxxxxxxxxxxx".to_string())],
                ad_ids: vec![Some("ad_xxxxxxxxxxxxxx".to_string())],
                account_id: None,
                user_id: None,
                interval: None,
                breakdown_by: None,
                convert_to: None,
                currency: None,
                time_zone: None,
                payment_method: None,
                card_network: None,
                dispute_reason: None,
                source: None,
                hostname: None,
                page: None,
                device_type: None,
                country_code: None,
                event_name: None,
                event_type: None,
                custom_name: None,
                segment: None,
                category: None,
                merchant: None,
                fee_type: None,
                product: None,
                status: None,
                access_level: None,
                most_recent_action: None,
                referred_user_id: None,
                snapshot_window: None,
                event: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**metric:** `String` — The metric to retrieve, for example net_revenue. Use GET /stats to see every metric key. The metric sets the unit and the properties you can filter or break down by.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — The account this query concerns, for example biz_AbC123.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The user this query concerns, for example user_AbC123. Available on metrics that support user subjects, such as account_balance.
    
</dd>
</dl>

<dl>
<dd>

**from:** `String` — Start of the range — a date (YYYY-MM-DD), expanded to the start of that day, or an ISO 8601 timestamp (for example 2026-07-16T16:37:00Z), used exactly.
    
</dd>
</dl>

<dl>
<dd>

**to:** `String` — End of the range — a date (YYYY-MM-DD), expanded to the end of that day, or an ISO 8601 timestamp (for example 2026-07-17T16:37:00Z), used exactly.
    
</dd>
</dl>

<dl>
<dd>

**interval:** `Option<RetrieveStatsRequestInterval>` — How wide each point is. Defaults to day. Snapshot metrics are day-only.
    
</dd>
</dl>

<dl>
<dd>

**breakdown_by:** `Option<String>` — Split the metric out by one of its properties — each point gets a breakdown array. For example breakdown_by=currency returns an entry for usd, an entry for eur, and so on.
    
</dd>
</dl>

<dl>
<dd>

**convert_to:** `Option<String>` — Display currency for money metrics — every amount is converted into this ISO currency using the exchange rate on each period's date. Defaults to usd. For the ads metrics (ad_spend, ad_delivery), pass the account's ads reporting currency to match the ad entity endpoints. On transaction metrics, it is ignored when you filter or break down by currency (those report the original transaction currency, unconverted).
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — Select the source currency or asset on metrics that list currency. For transaction metrics, for example currency=eur, values are reported without conversion. For market_prices, use btc or xaut and convert_to=usd. Pair with breakdown_by=currency to split a metric by currency.
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — IANA time zone to bucket the series in, for example America/New_York. Defaults to UTC. Not accepted by snapshot metrics, which are UTC only.
    
</dd>
</dl>

<dl>
<dd>

**payment_method:** `Option<String>` — Filter to a single payment method, for example card or crypto. Available on metrics that list payment_method.
    
</dd>
</dl>

<dl>
<dd>

**card_network:** `Option<String>` — Filter to a single card brand, for example visa. A refinement of payment_method=card. Available on metrics that list card_network.
    
</dd>
</dl>

<dl>
<dd>

**dispute_reason:** `Option<String>` — Filter disputes to a normalized reason, for example product_not_received. Pair with breakdown_by=dispute_reason to split dispute counts by reason.
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<String>` — Filter to a single GMV source, for example payments — or, on the traffic metrics, a visit source (whop_ads, direct, or a utm_source value). Pair with breakdown_by=source to split by source. Available on metrics that list source.
    
</dd>
</dl>

<dl>
<dd>

**hostname:** `Option<String>` — Filter traffic metrics to one website hostname, for example shop.example.com. Pair with breakdown_by=hostname to split by website.
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<String>` — Filter traffic metrics to one page — a hostname plus normalized path, for example shop.example.com/pricing. Pair with breakdown_by=page to split by page.
    
</dd>
</dl>

<dl>
<dd>

**device_type:** `Option<String>` — Filter traffic metrics to one device type: desktop, mobile, tablet, or unknown. Pair with breakdown_by=device_type to split by device.
    
</dd>
</dl>

<dl>
<dd>

**country_code:** `Option<String>` — Filter traffic metrics to one visitor country (uppercase ISO 3166-1 alpha-2, for example US). Pair with breakdown_by=country_code to split by country.
    
</dd>
</dl>

<dl>
<dd>

**event_name:** `Option<String>` — Filter the events metric to one tracked event name, for example pixel.page or pixel.custom. Pair with breakdown_by=event_name to split by event.
    
</dd>
</dl>

<dl>
<dd>

**event_type:** `Option<RetrieveStatsRequestEventType>` — Filter the events metric to a canonical group of events: page_view (pixel page views plus whop.com store views), checkout_start (hosted and embedded checkout views), or other. Pair with breakdown_by=event_type to split by group.
    
</dd>
</dl>

<dl>
<dd>

**custom_name:** `Option<String>` — Filter the events metric to one merchant-defined custom event name. Only valid alongside event_name=pixel.custom. Pair with breakdown_by=custom_name to split custom events by name.
    
</dd>
</dl>

<dl>
<dd>

**segment:** `Option<String>` — Filter to a single wallet-balance segment, for example available. Pair with breakdown_by=segment to split the balance. Available on metrics that list segment.
    
</dd>
</dl>

<dl>
<dd>

**category:** `Option<String>` — Filter to a single balance-activity category, for example payments. Pair with breakdown_by=category to split the activity. Available on metrics that list category.
    
</dd>
</dl>

<dl>
<dd>

**merchant:** `Option<String>` — Filter to a single cashback merchant bucket, for example whop-ads. Pair with breakdown_by=merchant to split cashback by merchant. Available on metrics that list merchant.
    
</dd>
</dl>

<dl>
<dd>

**fee_type:** `Option<String>` — Filter to a single fee type. Pair with breakdown_by=fee_type to split fees by type. Available on metrics that list fee_type.
    
</dd>
</dl>

<dl>
<dd>

**product:** `Option<String>` — Filter to a single product (access pass id), for example prod_AbC123. Pair with breakdown_by=product. Available on metrics that list product.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<String>` — Filter to a single membership status. Pair with breakdown_by=status. Available on metrics that list status.
    
</dd>
</dl>

<dl>
<dd>

**access_level:** `Option<String>` — Filter to a single access level. Pair with breakdown_by=access_level. Available on metrics that list access_level.
    
</dd>
</dl>

<dl>
<dd>

**most_recent_action:** `Option<String>` — Filter to a single most-recent member action. Pair with breakdown_by=most_recent_action. Available on metrics that list most_recent_action.
    
</dd>
</dl>

<dl>
<dd>

**referred_user_id:** `Option<String>` — Filter a referral metric to the businesses attributed to one person you referred, for example user_AbC123. Available on metrics that list referred_user_id.
    
</dd>
</dl>

<dl>
<dd>

**ad_campaign_ids:** `Option<String>` — Ad campaign ids (adcamp_...) to scope the report to; stats are summed across them. Available on metrics that list ad_campaign_ids.
    
</dd>
</dl>

<dl>
<dd>

**ad_group_ids:** `Option<String>` — Ad group ids (adgrp_...) to scope the report to; stats are summed across them. Available on metrics that list ad_group_ids.
    
</dd>
</dl>

<dl>
<dd>

**ad_ids:** `Option<String>` — Ad ids (ad_...) to scope the report to; stats are summed across them. Available on metrics that list ad_ids.
    
</dd>
</dl>

<dl>
<dd>

**snapshot_window:** `Option<RetrieveStatsRequestSnapshotWindow>` — Window used by a snapshot metric. Ordinary snapshots accept 30d as their trailing activity window. Cohorted dispute metrics accept 7d or 28d as the sales-transaction pool; their attribution window is fixed in the metric name. Each metric lists its accepted values in the catalog.
    
</dd>
</dl>

<dl>
<dd>

**event:** `Option<String>` — Filter the events metric to one or more full event names, for example payment.completed or pixel.lead. Comma-separate several to break the metric down by each event. Available on metrics that list event.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## SupportChannels
<details><summary><code>client.support_channels.<a href="/src/api/resources/support_channels/client.rs">list</a>(after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, company_id: Option&lt;Option&lt;String&gt;&gt;, view: Option&lt;Option&lt;SupportChannelView&gt;&gt;, open: Option&lt;Option&lt;bool&gt;&gt;, direction: Option&lt;Option&lt;Direction&gt;&gt;, order: Option&lt;Option&lt;MessageChannelOrder&gt;&gt;) -> Result&lt;ListSupportChannelsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of support channels for a specific company, with optional filtering by resolution status and custom sorting.

Required permissions:
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .support_channels
        .list(
            &SupportChannelsListQueryRequest {
                first: Some(42),
                last: Some(42),
                company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `Option<String>` — The unique identifier of the company to list support channels for. Includes channels of child companies. When omitted, returns support channels across all companies the user has access to.
    
</dd>
</dl>

<dl>
<dd>

**view:** `Option<SupportChannelView>` 
    
</dd>
</dl>

<dl>
<dd>

**open:** `Option<bool>` — Whether to filter by open or resolved support channels. Set to true to only return channels awaiting a response, or false for resolved channels.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<Direction>` 
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<MessageChannelOrder>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.support_channels.<a href="/src/api/resources/support_channels/client.rs">create</a>(request: CreateSupportChannelsRequest) -> Result&lt;SupportChannel, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Open a new support channel between a company team member and a customer. Returns the existing channel if one already exists for that user.

Required permissions:
 - `support_chat:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .support_channels
        .create(
            &CreateSupportChannelsRequest {
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                user_id: "user_xxxxxxxxxxxxx".to_string(),
                custom_name: None,
                notifications_enabled: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to create the support channel in.
    
</dd>
</dl>

<dl>
<dd>

**custom_name:** `Option<Option<String>>` — Optional custom display name for the support channel.
    
</dd>
</dl>

<dl>
<dd>

**notifications_enabled:** `Option<Option<bool>>` — Whether Whop app notifications are enabled for this support channel. Webhooks still fire.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `String` — The user ID (e.g. 'user_xxxxx') or username of the customer to open a support channel for.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.support_channels.<a href="/src/api/resources/support_channels/client.rs">retrieve</a>(id: String) -> Result&lt;SupportChannel, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing support channel.

Required permissions:
 - `support_chat:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .support_channels
        .retrieve(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The unique identifier of the support channel to retrieve.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Swaps
<details><summary><code>client.swaps.<a href="/src/api/resources/swaps/client.rs">list</a>(account_id: Option&lt;String&gt;) -> Result&lt;ListSwapsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the account's completed or pending swaps — currently just the latest one.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .swaps
        .list(
            &SwapsListQueryRequest {
                account_id: "account_id".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Business or user account ID (biz_* / user_*).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.swaps.<a href="/src/api/resources/swaps/client.rs">create</a>(request: CreateSwapsRequest) -> Result&lt;CreateSwapsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Swaps one token for another from the account's wallet, or converts between fiat currencies in the account's ledger at the mid-market rate. Crypto swaps finish in the background — check the swap for its status.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .swaps
        .create(
            &CreateSwapsRequest {
                account_id: "biz_xxxxxxxxxxxxxx".to_string(),
                from_token: "usd".to_string(),
                to_token: "cad".to_string(),
                amount: None,
                from_chain: None,
                slippage_bps: None,
                to_amount: None,
                to_chain: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Business or user account ID (biz_* / user_*).
    
</dd>
</dl>

<dl>
<dd>

**amount:** `Option<Option<String>>` — Source token amount. Required for crypto swaps. For fiat pairs: the amount of from_token to convert at the mid-market rate; omit (along with to_amount) to repay the full negative to_token balance instead.
    
</dd>
</dl>

<dl>
<dd>

**from_chain:** `Option<Option<CreateSwapsRequestFromChain>>` — Source chain name or chain ID. Defaults to the source token's chain when omitted.
    
</dd>
</dl>

<dl>
<dd>

**from_token:** `String` — Source token contract address or ticker symbol, such as "USDT".
    
</dd>
</dl>

<dl>
<dd>

**slippage_bps:** `Option<Option<i64>>` — Maximum slippage tolerance in basis points.
    
</dd>
</dl>

<dl>
<dd>

**to_amount:** `Option<Option<String>>` — Fiat pairs only: sizes a partial repayment of the negative to_token balance, denominated in to_token. Must not exceed the debt. Mutually exclusive with amount.
    
</dd>
</dl>

<dl>
<dd>

**to_chain:** `Option<Option<CreateSwapsRequestToChain>>` — Destination chain name or chain ID. Defaults to the destination token's chain when omitted.
    
</dd>
</dl>

<dl>
<dd>

**to_token:** `String` — Destination token contract address or ticker symbol, such as "XAUT".
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.swaps.<a href="/src/api/resources/swaps/client.rs">create_quote</a>(request: CreateQuoteSwapsRequest) -> Result&lt;CreateQuoteSwapsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Previews the price of a swap. Fiat pairs quote the in-ledger mid-market conversion — the same rate creating the swap fills at. No funds move and nothing is saved.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .swaps
        .create_quote(
            &CreateQuoteSwapsRequest {
                amount: "100".to_string(),
                from_token: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
                to_token: "0x1b64b9025eebb9a6239575df9ea4b9ac46d4d193".to_string(),
                from_address: None,
                from_chain: None,
                metadata: None,
                slippage_bps: None,
                to_address: None,
                to_chain: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**amount:** `String` — Source token amount.
    
</dd>
</dl>

<dl>
<dd>

**from_address:** `Option<Option<String>>` — Source wallet address used for the quote.
    
</dd>
</dl>

<dl>
<dd>

**from_chain:** `Option<Option<CreateQuoteSwapsRequestFromChain>>` — Source chain name or chain ID. Defaults to the source token's chain when omitted.
    
</dd>
</dl>

<dl>
<dd>

**from_token:** `String` — Source token contract address or ticker symbol, such as "USDT".
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Metadata to include with the quote response.
    
</dd>
</dl>

<dl>
<dd>

**slippage_bps:** `Option<Option<i64>>` — Maximum slippage tolerance in basis points.
    
</dd>
</dl>

<dl>
<dd>

**to_address:** `Option<Option<String>>` — Destination wallet address used for the quote.
    
</dd>
</dl>

<dl>
<dd>

**to_chain:** `Option<Option<CreateQuoteSwapsRequestToChain>>` — Destination chain name or chain ID. Defaults to the destination token's chain when omitted.
    
</dd>
</dl>

<dl>
<dd>

**to_token:** `String` — Destination token contract address or ticker symbol, such as "XAUT".
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.swaps.<a href="/src/api/resources/swaps/client.rs">retrieve</a>(id: String) -> Result&lt;RetrieveSwapsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single swap and its status.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.swaps.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Swap ID returned from POST /swaps.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Team Members
<details><summary><code>client.team_members.<a href="/src/api/resources/team_members/client.rs">list</a>(account_id: Option&lt;String&gt;, status: Option&lt;Option&lt;ListTeamMembersRequestStatus&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, role: Option&lt;Option&lt;ListTeamMembersRequestRole&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListTeamMembersRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListTeamMembersRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListTeamMembersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists an account's team members, including pending invites (`status: "pending"`, `ausri_` ids; `user` is `null` for invites sent to an email with no Whop account yet). For accepted members, `email` requires the `company:authorized_user:email:read` scope and is `null` otherwise. Listing `role=workforce` is also allowed with the `bounty:create` scope.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .team_members
        .list(
            &TeamMembersListQueryRequest {
                account_id: "account_id".to_string(),
                status: None,
                user_id: None,
                role: None,
                created_before: None,
                created_after: None,
                order: None,
                direction: None,
                first: None,
                after: None,
                last: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Account ID, prefixed `biz_`.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListTeamMembersRequestStatus>` — Only return members with this status: `joined` (accepted members) or `pending` (pending invites). Both are returned by default.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — Only return the membership for this user ID, prefixed `user_`.
    
</dd>
</dl>

<dl>
<dd>

**role:** `Option<ListTeamMembersRequestRole>` — Only return members with this role. `custom` matches members on a dashboard-managed custom role.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return members added before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return members added after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListTeamMembersRequestOrder>` — Field used to sort members.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListTeamMembersRequestDirection>` — Sort direction. Defaults to `desc`.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of members to return. Defaults to 20; maximum 100.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor for the next page of members.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of members to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to paginate backwards from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team_members.<a href="/src/api/resources/team_members/client.rs">create</a>(request: CreateTeamMembersRequest) -> Result&lt;TeamMember, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Adds a member to an account's team with a system role. Identify them by exactly one of `user_id` or `email`. If the person has not yet accepted — or the email does not belong to a Whop account yet — an invitation is sent instead and the response is `202` with `{ "object": "team_member_invite", "invitation_sent": true }`. If they already have a pending invite, the request fails with a `400`. Custom roles cannot be granted via the API. Granting the `workforce` role is also allowed with the `bounty:create` scope.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .team_members
        .create(
            &CreateTeamMembersRequest {
                account_id: "biz_xxxxxxxxxxxxxx".to_string(),
                role: CreateTeamMembersRequestRole::Owner,
                email: None,
                user_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Account ID, prefixed `biz_`.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Email address to invite. Mutually exclusive with `user_id`. If the email already belongs to a Whop account it is treated the same as passing that account's `user_id`; otherwise a pending invite is created for the email.
    
</dd>
</dl>

<dl>
<dd>

**role:** `CreateTeamMembersRequestRole` — The system role to grant.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The user to add to the team, prefixed `user_`. Mutually exclusive with `email`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team_members.<a href="/src/api/resources/team_members/client.rs">retrieve</a>(id: String) -> Result&lt;TeamMember, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a team member by ID. `email` requires the `company:authorized_user:email:read` scope and is `null` otherwise.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.team_members.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Team member ID — `ausr_` for accepted members, `ausri_` for pending invites.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team_members.<a href="/src/api/resources/team_members/client.rs">delete</a>(id: String) -> Result&lt;DeleteTeamMembersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Removes a team member from the account, or revokes a pending invite when given an `ausri_` ID. A user session may delete its own membership to leave the team without the delete scope. Removing a member on the `workforce` role is also allowed with the `bounty:create` scope. The account owner cannot be removed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.team_members.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Team member ID — `ausr_` for accepted members, `ausri_` for pending invites.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.team_members.<a href="/src/api/resources/team_members/client.rs">update</a>(id: String, request: UpdateTeamMembersRequest) -> Result&lt;TeamMember, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Changes a team member's system role. Requires a user session — account API keys cannot change member roles. The account owner's role cannot be changed, and you cannot change your own role.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .team_members
        .update(
            &"id".to_string(),
            &UpdateTeamMembersRequest {
                role: UpdateTeamMembersRequestRole::Owner,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Team member ID — `ausr_` for accepted members, `ausri_` for pending invites.
    
</dd>
</dl>

<dl>
<dd>

**role:** `UpdateTeamMembersRequestRole` — The system role to grant.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Topups
<details><summary><code>client.topups.<a href="/src/api/resources/topups/client.rs">create</a>(request: CreateTopupsRequest) -> Result&lt;Topup, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add funds to a company's platform balance by charging a stored payment method. Top-ups have no fees or taxes and do not count as revenue.

Required permissions:
 - `payment:charge`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .topups
        .create(
            &CreateTopupsRequest {
                amount: 6.9,
                company_id: "biz_xxxxxxxxxxxxxx".to_string(),
                currency: Currencies::Usd,
                payment_method_id: "pmt_xxxxxxxxxxxxxx".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**amount:** `f64` — The amount to add to the balance in the specified currency. For example, 50.00 for $50.00 USD.
    
</dd>
</dl>

<dl>
<dd>

**company_id:** `String` — The unique identifier of the company to add funds to, starting with 'biz_'.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Currencies` — The currency for the top-up amount, such as 'usd'.
    
</dd>
</dl>

<dl>
<dd>

**payment_method_id:** `String` — The unique identifier of the stored payment method to charge for the top-up.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Transfers
<details><summary><code>client.transfers.<a href="/src/api/resources/transfers/client.rs">list</a>(origin_id: Option&lt;Option&lt;String&gt;&gt;, destination_id: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListTransfersRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListTransfersRequestDirection&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListTransfersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists an account's transfers.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .transfers
        .list(
            &TransfersListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**origin_id:** `Option<String>` — Filter to transfers sent from this account. Provide this or destination_id.
    
</dd>
</dl>

<dl>
<dd>

**destination_id:** `Option<String>` — Filter to transfers received by this account. Provide this or origin_id.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListTransfersRequestOrder>` — Sort column. Defaults to created_at.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListTransfersRequestDirection>` — Sort direction. Defaults to desc.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only transfers created strictly before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only transfers created strictly after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of transfers to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of transfers to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to fetch the page before (from page_info.start_cursor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transfers.<a href="/src/api/resources/transfers/client.rs">create</a>(request: CreateTransfersRequest) -> Result&lt;CreateTransfersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Moves money between accounts, or into a claim link anyone with the URL can redeem.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .transfers
        .create(
            &CreateTransfersRequest {
                amount: 25.0,
                origin_id: "biz_xxxxxxxxxxxxxx".to_string(),
                currency: None,
                destination_id: None,
                expires_at: None,
                idempotence_key: None,
                metadata: None,
                notes: None,
                redeemable_count: None,
                r#type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**amount:** `f64` — The amount to move, in the transfer currency. For example 25.00.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — Currency, such as `usd`. Required for ledger transfers.
    
</dd>
</dl>

<dl>
<dd>

**destination_id:** `Option<String>` — The recipient. Required for ledger and wallet_send (a user_/biz_/ldgr_ ID, or — for sends — an email). Omit for claim_link.
    
</dd>
</dl>

<dl>
<dd>

**expires_at:** `Option<Option<String>>` — claim_link only. Link expiry as an ISO 8601 timestamp. Defaults to 24 hours from creation.
    
</dd>
</dl>

<dl>
<dd>

**idempotence_key:** `Option<Option<String>>` — Ledger transfers and wallet sends. A unique key that makes retries safe. Retrying with the same key returns the original transfer, or attaches to the original wallet send, instead of moving money twice.
    
</dd>
</dl>

<dl>
<dd>

**metadata:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Ledger transfers only. Custom key-value pairs attached to the transfer. Max 50 keys, 100 chars per key, 500 chars per string value.
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<Option<String>>` — Ledger transfers only. A short note describing the transfer.
    
</dd>
</dl>

<dl>
<dd>

**origin_id:** `String` — The account sending the funds. A user ID (user_xxx), account ID (biz_xxx), or ledger account ID (ldgr_xxx).
    
</dd>
</dl>

<dl>
<dd>

**redeemable_count:** `Option<i64>` — claim_link only. How many different users can claim the link. Defaults to 1.
    
</dd>
</dl>

<dl>
<dd>

**type_:** `Option<CreateTransfersRequestType>` — The kind of money movement, which decides what comes back. Defaults to ledger. `ledger` moves credit between two Whop balances and returns a `transfer`; `wallet_send` sends USDT from the origin account's Ethereum wallet and returns a `send`; `claim_link` funds a shareable link anyone with the URL can redeem and returns a `claim_link`. A `ledger` transfer from a stablecoin-rails account settles on-chain when covered, and still returns a `transfer`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transfers.<a href="/src/api/resources/transfers/client.rs">list_recipients</a>(origin_id: Option&lt;String&gt;, query: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListRecipientsTransfersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the people and accounts you can send money to.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .transfers
        .list_recipients(
            &ListRecipientsQueryRequest {
                origin_id: "origin_id".to_string(),
                query: None,
                first: None,
                after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**origin_id:** `String` — The account sending the money: a company account ID (`biz_`), or a user ID (`user_`) for that user's own personal balance.
    
</dd>
</dl>

<dl>
<dd>

**query:** `Option<String>` — Search anyone on Whop by name or username, plus your own accounts by name or ID. Omit it to get the team around the balance, the people you follow, and your own accounts. The list is the same whether the balance belongs to a company or to you. Searching from a `biz_` origin additionally requires the member:basic:read scope. A credential scoped to a single company is the exception to the search itself: it only ever sees that company's own people. Complete email addresses return no matches.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of recipients per page. Search queries preserve the dashboard's 20-result maximum.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transfers.<a href="/src/api/resources/transfers/client.rs">retrieve</a>(id: String) -> Result&lt;RetrieveTransfersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single transfer.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.transfers.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The transfer ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Users
<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">list</a>(query: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListUsersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Search for users by name or username, ranked by social proximity to the authenticated user. Returns the user's most recently followed users when no query is given.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .list(
            &UsersListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**query:** `Option<String>` — A search term to filter users by name or username.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of users to return (max 50).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns users after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of users to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns users before this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">me</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, include_balance_history: Option&lt;Option&lt;bool&gt;&gt;, from: Option&lt;Option&lt;String&gt;&gt;, to: Option&lt;Option&lt;String&gt;&gt;, interval: Option&lt;Option&lt;MeUsersRequestInterval&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;User, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the authenticated user — the self view of the user object. Same shape as `GET /users/{id}`, with the self-only fields populated: `email` (email-read scope), `staff` (Whop staff only, staff-read scope), `balance` and `earnings_usd` (balance-read scope), the opt-in `balance_history`, and every linked social account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .me(
            &MeQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — When set, returns your account-specific profile overrides for this account.
    
</dd>
</dl>

<dl>
<dd>

**include_balance_history:** `Option<bool>` — Also compute your balance history (opt-in; runs a heavier query). Ignored for callers without balance-read scope.
    
</dd>
</dl>

<dl>
<dd>

**from:** `Option<String>` — Balance-history window start, ISO 8601 date or datetime. Defaults to 30 days ago. Only used with `include_balance_history`.
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<String>` — Balance-history window end, ISO 8601 date or datetime. Defaults to now. Only used with `include_balance_history`.
    
</dd>
</dl>

<dl>
<dd>

**interval:** `Option<MeUsersRequestInterval>` — Balance-history point granularity. Defaults to `day`. Only used with `include_balance_history`.
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — IANA time zone the balance-history points are bucketed in. Defaults to `UTC`. Only used with `include_balance_history`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">update_me</a>(request: UpdateMeUsersRequest, account_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;User, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates the authenticated user's global profile, or their profile override for an account when account_id is given. Not available to API keys.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .update_me(
            &UpdateMeUsersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**banner:** `Option<Option<UpdateMeUsersRequestBanner>>` 
    
</dd>
</dl>

<dl>
<dd>

**bio:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**profile_picture:** `Option<UpdateMeUsersRequestProfilePicture>` 
    
</dd>
</dl>

<dl>
<dd>

**username:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — When set, updates the authenticated user's profile override for this account instead of their global profile.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">retrieve</a>(id: String, account_id: Option&lt;Option&lt;String&gt;&gt;, include_balance_history: Option&lt;Option&lt;bool&gt;&gt;, from: Option&lt;Option&lt;String&gt;&gt;, to: Option&lt;Option&lt;String&gt;&gt;, interval: Option&lt;Option&lt;RetrieveUsersRequestInterval&gt;&gt;, time_zone: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;User, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a user by `user_` tag or username, or the authenticated user with the reserved id `me`. Profiles include linked social accounts — reading your own profile returns every linked account, other profiles only what is public on Whop (the primary Discord and the X account). The self-only fields are populated only when the id is `me`: `email` (email-read scope), `staff` (Whop staff only, staff-read scope), `balance` and `earnings_usd` (balance-read scope), and the opt-in `balance_history`. They are always `null` when addressing a user by tag or username.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .retrieve(
            &"id".to_string(),
            &UsersRetrieveQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — User ID (prefixed `user_`), username, or `me` for the authenticated user.
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — When set, returns the user's account-specific profile overrides for this account.
    
</dd>
</dl>

<dl>
<dd>

**include_balance_history:** `Option<bool>` — Also compute your balance history (opt-in; runs a heavier query). Only applies when the id is `me`; ignored for callers without balance-read scope.
    
</dd>
</dl>

<dl>
<dd>

**from:** `Option<String>` — Balance-history window start, ISO 8601 date or datetime. Defaults to 30 days ago. Only used with `include_balance_history`.
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<String>` — Balance-history window end, ISO 8601 date or datetime. Defaults to now. Only used with `include_balance_history`.
    
</dd>
</dl>

<dl>
<dd>

**interval:** `Option<RetrieveUsersRequestInterval>` — Balance-history point granularity. Defaults to `day`. Only used with `include_balance_history`.
    
</dd>
</dl>

<dl>
<dd>

**time_zone:** `Option<String>` — IANA time zone the balance-history points are bucketed in. Defaults to `UTC`. Only used with `include_balance_history`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">update</a>(id: String, request: UpdateUsersRequest, account_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;User, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a user, addressed by `user_` tag, username, or the reserved id `me` for the authenticated user. A user token updates their own global profile; an API key updates the user's account-specific profile override (account_id required).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .update(
            &"id".to_string(),
            &UpdateUsersRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — User ID (prefixed `user_`), username, or `me` for the authenticated user.
    
</dd>
</dl>

<dl>
<dd>

**banner:** `Option<Option<UpdateUsersRequestBanner>>` 
    
</dd>
</dl>

<dl>
<dd>

**bio:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**profile_picture:** `Option<UpdateUsersRequestProfilePicture>` 
    
</dd>
</dl>

<dl>
<dd>

**username:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — The account whose profile override to update. Required for API key callers.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">check_access</a>(id: String, resource_id: String) -> Result&lt;CheckAccessUsersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Checks whether a user has access to an account, product, or experience the caller can reach.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .check_access(&"id".to_string(), &"resource_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The user_ tag or username to check access for.
    
</dd>
</dl>

<dl>
<dd>

**resource_id:** `String` — An account (biz_), product (prod_), or experience (exp_) ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users.<a href="/src/api/resources/users/client.rs">recommend_actions</a>(id: String) -> Result&lt;RecommendActionsUsersResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the recommended actions computed for the user: personal suggestions (e.g. start a business or become an affiliate) pooled with the highest-impact actions across the accounts the user owns. Business actions are tagged with their `account_id`/`account_name`; personal actions leave those `null`. Self-only: `id` must be `me` or the authenticated user's own tag/username.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .recommend_actions(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — `me`, or the authenticated user's own `user_` tag or username.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Verifications
<details><summary><code>client.verifications.<a href="/src/api/resources/verifications/client.rs">list</a>(account_id: Option&lt;String&gt;, order: Option&lt;Option&lt;ListVerificationsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListVerificationsRequestDirection&gt;&gt;) -> Result&lt;ListVerificationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns verifications for an account, including their status and any required actions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .verifications
        .list(
            &VerificationsListQueryRequest {
                account_id: "account_id".to_string(),
                order: None,
                direction: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Account or user ID whose verifications you want to list. Use a `biz_` account ID, or the caller's `user_` ID for personal verifications.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListVerificationsRequestOrder>` — Field used to sort returned verifications.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListVerificationsRequestDirection>` — Sort direction for returned verifications.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.verifications.<a href="/src/api/resources/verifications/client.rs">create</a>(request: CreateVerificationsRequestBody) -> Result&lt;CreateVerificationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Starts a hosted verification session for an account or user, or returns the active session when one already exists. Any fields you include in the request body are used to prefill the session. Send `documents` (with `document_type`) to instead verify the person from identity documents included in this request — no hosted session involved. Send `share_token` to reuse a verification another Sumsub account has already completed for this person, instead of verifying them again. If the account already has an `approved` verification the request is rejected; unlink it first to start a new one.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .verifications
        .create(
            &CreateVerificationsRequestBody::Individual {
                data: CreateVerificationsRequestBodyIndividual {
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.verifications.<a href="/src/api/resources/verifications/client.rs">retrieve</a>(id: String) -> Result&lt;RetrieveVerificationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns verifications for an account, including their status and any required actions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.verifications.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Verification profile ID, prefixed `idpf_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.verifications.<a href="/src/api/resources/verifications/client.rs">update</a>(id: String, request: UpdateVerificationsRequestBody) -> Result&lt;UpdateVerificationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates editable profile details or submits answers for items returned in `requested_information`. Once a verification is `approved` its profile details are locked and can no longer be edited.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .verifications
        .update(
            &"id".to_string(),
            &UpdateVerificationsRequestBody::UpdateVerificationsRequestBodyPersonalAddress(
                UpdateVerificationsRequestBodyPersonalAddress {
                    ..Default::default()
                },
            ),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Verification profile ID, prefixed `idpf_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Webhooks
<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">list</a>(account_id: Option&lt;String&gt;, app_id: Option&lt;Option&lt;String&gt;&gt;, include_app_webhooks: Option&lt;Option&lt;bool&gt;&gt;, has_failures: Option&lt;Option&lt;bool&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of webhook endpoints configured for an account, ordered by most recently created.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .webhooks
        .list(
            &WebhooksListQueryRequest {
                account_id: "account_id".to_string(),
                app_id: None,
                include_app_webhooks: None,
                has_failures: None,
                first: None,
                after: None,
                last: None,
                before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — The unique identifier of the account to list webhooks for.
    
</dd>
</dl>

<dl>
<dd>

**app_id:** `Option<String>` — Only return webhooks attached to this app. Omit to list the account's own webhooks.
    
</dd>
</dl>

<dl>
<dd>

**include_app_webhooks:** `Option<bool>` — Also return webhooks attached to the account's apps, not just the account's own. Cannot be combined with `app_id`.
    
</dd>
</dl>

<dl>
<dd>

**has_failures:** `Option<bool>` — Only return webhooks whose endpoint is currently failing — every delivery since the current failure streak began has been rejected. Clears as soon as a delivery succeeds.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of webhooks to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns webhooks after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of webhooks to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns webhooks before this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">create</a>(request: CreateWebhooksRequest) -> Result&lt;Webhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a webhook endpoint that receives event notifications via HTTP POST.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .webhooks
        .create(
            &CreateWebhooksRequest {
                url: "https://example.com/hooks".to_string(),
                api_version_date: None,
                child_resource_events: None,
                enabled: None,
                events: None,
                resource_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_version_date:** `Option<Option<String>>` — The dated API version (Api-Version-Date) to pin this webhook's payloads to. Omit to leave the webhook unpinned, tracking the current payload shape.
    
</dd>
</dl>

<dl>
<dd>

**child_resource_events:** `Option<bool>` — Whether to send events for child resources. For example, if the webhook is created for an account, enabling this sends events only from its connected accounts.
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `Option<bool>` — Whether or not the webhook is enabled. Defaults to `true`.
    
</dd>
</dl>

<dl>
<dd>

**events:** `Option<Vec<CreateWebhooksRequestEventsItem>>` — The events to send the webhook for, in dot form (for example `payment.succeeded`).
    
</dd>
</dl>

<dl>
<dd>

**resource_id:** `Option<Option<String>>` — The account or app to create the webhook for. Defaults to the current account.
    
</dd>
</dl>

<dl>
<dd>

**url:** `String` — The URL to send the webhook to.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">retrieve</a>(id: String) -> Result&lt;Webhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of an existing webhook.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.webhooks.retrieve(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Webhook ID, prefixed `hook_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">delete</a>(id: String) -> Result&lt;DeleteWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently deletes a webhook endpoint. Returns `true` on success, matching the legacy proxy response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.webhooks.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Webhook ID, prefixed `hook_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">update</a>(id: String, request: UpdateWebhooksRequest) -> Result&lt;Webhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a webhook endpoint's URL, subscribed events, pinned payload version, or enabled state.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .webhooks
        .update(
            &"id".to_string(),
            &UpdateWebhooksRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Webhook ID, prefixed `hook_`.
    
</dd>
</dl>

<dl>
<dd>

**api_version_date:** `Option<Option<String>>` — The dated API version (Api-Version-Date) to pin this webhook's payloads to. Only valid for `v1` webhooks. Omit to leave the current pin unchanged, or pass `null` to unpin and track the current payload shape.
    
</dd>
</dl>

<dl>
<dd>

**child_resource_events:** `Option<bool>` — Whether or not to send events for child resources.
    
</dd>
</dl>

<dl>
<dd>

**enabled:** `Option<bool>` — Whether or not the webhook is enabled.
    
</dd>
</dl>

<dl>
<dd>

**events:** `Option<Vec<UpdateWebhooksRequestEventsItem>>` — The events to send the webhook for, in dot form (for example `payment.succeeded`).
    
</dd>
</dl>

<dl>
<dd>

**url:** `Option<String>` — The URL to send the webhook to.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">list_deliveries</a>(id: String, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListDeliveriesWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of delivery attempts for a webhook, ordered by most recent first. Includes the request payload, response body, response code, and timing for each attempt.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .webhooks
        .list_deliveries(
            &"id".to_string(),
            &ListDeliveriesQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Webhook ID, prefixed `hook_`.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of deliveries to return (default 50, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns deliveries after this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">replay_delivery</a>(id: String, delivery_id: String, request: ReplayDeliveryWebhooksRequest) -> Result&lt;ReplayDeliveryWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Re-sends the exact payload of a past delivery to the webhook's current URL and returns the delivery result. By default the replay keeps the original `webhook-id`, so consumers that deduplicate on it can drop events they already processed. Pass `regenerate_id` to re-send under a freshly generated `webhook-id` instead, so a deduplicating consumer processes the replay as a new message. Only available for enabled webhooks on API version v1; deliveries are retained for 30 days.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .webhooks
        .replay_delivery(
            &"id".to_string(),
            &"delivery_id".to_string(),
            &ReplayDeliveryWebhooksRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Webhook ID, prefixed `hook_`.
    
</dd>
</dl>

<dl>
<dd>

**delivery_id:** `String` — Delivery ID, prefixed `whdel_`, from the List Deliveries endpoint.
    
</dd>
</dl>

<dl>
<dd>

**regenerate_id:** `Option<bool>` — Re-send the delivery under a freshly generated `webhook-id` (in both the envelope and the signed headers) instead of the original one. Defaults to false. Use this when your endpoint deduplicates on `webhook-id` and you want it to process the replay as a new message.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">replay</a>(id: String, request: ReplayWebhooksRequest) -> Result&lt;ReplayWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Re-sends the webhook's past deliveries within a time window, optionally limited to specific events or to messages whose most recent delivery attempt failed. Fire and forget: nothing about the replay is stored, and each re-send appears as a new entry in the webhook's delivery log. Each matching message is re-sent once, by default with its original `webhook-id`, so consumers that deduplicate are unaffected; pass `regenerate_ids` to re-send under freshly generated ids instead. Only available for enabled webhooks on API version v1; deliveries are retained for 30 days.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .webhooks
        .replay(
            &"id".to_string(),
            &ReplayWebhooksRequest {
                sent_after: "2026-01-01T12:00:00.000Z".to_string(),
                events: None,
                failed_only: None,
                regenerate_ids: None,
                sent_before: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Webhook ID, prefixed `hook_`.
    
</dd>
</dl>

<dl>
<dd>

**events:** `Option<Vec<String>>` — Only replay these event types, in dot form (for example `payment.succeeded`). Omit to include every event.
    
</dd>
</dl>

<dl>
<dd>

**failed_only:** `Option<bool>` — Only replay messages whose most recent delivery attempt in the window failed. Defaults to false. Best-effort: a message whose attempts span processing batches can still be re-sent — replays keep the original `webhook-id` by default, so consumers that deduplicate are unaffected.
    
</dd>
</dl>

<dl>
<dd>

**regenerate_ids:** `Option<bool>` — Re-send each replayed message under a freshly generated `webhook-id` (in both the envelope and the signed headers) instead of its original one. Defaults to false. Use this when your endpoint deduplicates on `webhook-id` and you want it to process the replays as new messages.
    
</dd>
</dl>

<dl>
<dd>

**sent_after:** `String` — Start of the delivery window to replay, as an ISO 8601 timestamp. Clamped to the 30-day delivery retention.
    
</dd>
</dl>

<dl>
<dd>

**sent_before:** `Option<String>` — End of the delivery window to replay, as an ISO 8601 timestamp. Defaults to now.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">test</a>(id: String, request: TestWebhooksRequest) -> Result&lt;TestWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Sends a sample payload for the given event to the webhook's URL and returns the delivery result.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .webhooks
        .test(
            &"id".to_string(),
            &TestWebhooksRequest {
                event: "payment.succeeded".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Webhook ID, prefixed `hook_`.
    
</dd>
</dl>

<dl>
<dd>

**event:** `String` — The event to test the webhook for, in dot form (for example `payment.succeeded`).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">deliveries_webhook</a>(webhook_id: String, after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;) -> Result&lt;DeliveriesWebhookResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of delivery attempts for a webhook, ordered by most recent first. Includes the request payload, response body, response code, and timing for each attempt.

Required permissions:
 - `developer:manage_webhook`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .webhooks
        .deliveries_webhook(
            &"webhook_id".to_string(),
            &DeliveriesWebhookQueryRequest {
                first: Some(42),
                last: Some(42),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `String` — The unique identifier of the webhook to list deliveries for.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Accounts Preferences
<details><summary><code>client.accounts().preferences.<a href="/src/api/resources/accounts/preferences/client.rs">retrieve</a>(account_id: String) -> Result&lt;RetrievePreferencesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the account's preferences: a singleton settings document keyed by preference name.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .accounts
        .preferences
        .retrieve(&"account_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Account ID, prefixed `biz_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.accounts().preferences.<a href="/src/api/resources/accounts/preferences/client.rs">update</a>(account_id: String, request: UpdatePreferencesRequest) -> Result&lt;UpdatePreferencesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates the account's preferences. Each top-level key present in the body is replaced as a whole; omitted keys are left untouched. `ads_triple_whale_integration` takes the Data-In API key to connect with, or `null` to disconnect. `ads_payment_methods` always requires a `primary` entry. `backup` is optional and any pairing is allowed — two cards, `card`+`platform_balance`, or a single method — so a card-only advertiser can fund ads without a platform balance. The `primary` and `backup` must be different sources. A `platform_balance` entry may omit `id` to use the account's default Whop balance. Configuring a `card` requires a user token; account API keys can set up platform-balance billing only.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .accounts
        .preferences
        .update(
            &"account_id".to_string(),
            &UpdatePreferencesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Account ID, prefixed `biz_`.
    
</dd>
</dl>

<dl>
<dd>

**ads_payment_methods:** `Option<UpdatePreferencesRequestAdsPaymentMethods>` — How the account pays for Whop Ads spend. `primary` is charged first; `backup` covers the charge when the primary fails.
    
</dd>
</dl>

<dl>
<dd>

**ads_reporting_currency:** `Option<String>` — Lowercase ISO currency code, such as `usd` or `eur`, used to display ad spend and stats. Defaults to `usd`.
    
</dd>
</dl>

<dl>
<dd>

**ads_scheduling_timezone:** `Option<String>` — IANA timezone (e.g. `America/New_York`) used to interpret campaign start/end times and to bucket reports. Cannot be cleared once set — pass a new value to change it.
    
</dd>
</dl>

<dl>
<dd>

**ads_triple_whale_integration:** `Option<UpdatePreferencesRequestAdsTripleWhaleIntegration>` — Connects or disconnects the Triple Whale integration. Requires a connected Shopify store, since Triple Whale keys spend records by Shopify shop.
    
</dd>
</dl>

<dl>
<dd>

**cards_auto_top_up:** `Option<bool>` — Whether incoming funds are automatically moved to the account's cards balance. Requires a cards balance on the account.
    
</dd>
</dl>

<dl>
<dd>

**dispute_fighter_enabled:** `Option<bool>` — Whether Whop assembles and files the evidence response when this account's payments are disputed. Off by default; enabling it also opts the account into the success fee charged only on disputes it wins.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Accounts Reserves
<details><summary><code>client.accounts().reserves.<a href="/src/api/resources/accounts/reserves/client.rs">list</a>(account_id: String) -> Result&lt;ListReservesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists what the account's held balance is made of, one entry per currency: the total held, why each part is held, and the days it unlocks.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .accounts
        .reserves
        .list(&"account_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `String` — Account ID, prefixed `biz_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Affiliates Overrides
<details><summary><code>client.affiliates().overrides.<a href="/src/api/resources/affiliates/overrides/client.rs">list</a>(id: String, after: Option&lt;Option&lt;String&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, override_type: Option&lt;Option&lt;AffiliateOverrideRoles&gt;&gt;) -> Result&lt;ListOverridesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a paginated list of overrides for an affiliate.

Required permissions:
 - `affiliate:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .affiliates
        .overrides
        .list(
            &"aff_xxxxxxxxxxxxxx".to_string(),
            &AffiliatesOverridesListQueryRequest {
                first: Some(42),
                last: Some(42),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The affiliate ID.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Returns the elements in the list that come after the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Returns the elements in the list that come before the specified cursor.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Returns the first _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Returns the last _n_ elements from the list.
    
</dd>
</dl>

<dl>
<dd>

**override_type:** `Option<AffiliateOverrideRoles>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.affiliates().overrides.<a href="/src/api/resources/affiliates/overrides/client.rs">create</a>(id: String, request: CreateOverridesRequestBody) -> Result&lt;CreateOverridesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a commission override for an affiliate.

Required permissions:
 - `affiliate:create`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .affiliates
        .overrides
        .create(
            &"aff_xxxxxxxxxxxxxx".to_string(),
            &CreateOverridesRequestBody::Standard {
                data: CreateOverridesRequestBodyStandard {
                    commission_value: 6.9,
                    id: "id".to_string(),
                    plan_id: "plan_xxxxxxxxxxxxx".to_string(),
                    ..Default::default()
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The affiliate ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.affiliates().overrides.<a href="/src/api/resources/affiliates/overrides/client.rs">retrieve</a>(id: String, override_id: String) -> Result&lt;RetrieveOverridesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details of a specific affiliate override.

Required permissions:
 - `affiliate:basic:read`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .affiliates
        .overrides
        .retrieve(
            &"aff_xxxxxxxxxxxxxx".to_string(),
            &"override_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The affiliate ID.
    
</dd>
</dl>

<dl>
<dd>

**override_id:** `String` — The override ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.affiliates().overrides.<a href="/src/api/resources/affiliates/overrides/client.rs">delete</a>(id: String, override_id: String) -> Result&lt;bool, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an affiliate override.

Required permissions:
 - `affiliate:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .affiliates
        .overrides
        .delete(
            &"aff_xxxxxxxxxxxxxx".to_string(),
            &"override_id".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The affiliate ID.
    
</dd>
</dl>

<dl>
<dd>

**override_id:** `String` — The override ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.affiliates().overrides.<a href="/src/api/resources/affiliates/overrides/client.rs">update</a>(id: String, override_id: String, request: UpdateOverridesRequest) -> Result&lt;UpdateOverridesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an existing affiliate override.

Required permissions:
 - `affiliate:update`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .affiliates
        .overrides
        .update(
            &"aff_xxxxxxxxxxxxxx".to_string(),
            &"override_id".to_string(),
            &UpdateOverridesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The affiliate ID.
    
</dd>
</dl>

<dl>
<dd>

**override_id:** `String` — The override ID.
    
</dd>
</dl>

<dl>
<dd>

**applies_to_payments:** `Option<Option<AffiliateAppliesToPayments>>` — Whether commission applies to first payment or all payments (standard only).
    
</dd>
</dl>

<dl>
<dd>

**commission_type:** `Option<Option<AffiliatePayoutTypes>>` — The commission type (percentage or flat_fee).
    
</dd>
</dl>

<dl>
<dd>

**commission_value:** `Option<Option<f64>>` — The commission value (percentage 1-100 or flat fee in dollars).
    
</dd>
</dl>

<dl>
<dd>

**revenue_basis:** `Option<Option<AffiliateRevenueBases>>` — The revenue calculation basis (rev-share only).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Bounties Submissions
<details><summary><code>client.bounties().submissions.<a href="/src/api/resources/bounties/submissions/client.rs">list</a>(bounty_id: String, status: Option&lt;Option&lt;ListSubmissionsRequestStatus&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListSubmissionsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListSubmissionsRequestDirection&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListSubmissionsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists a bounty's publicly visible work — submitted, approved, and denied submissions in the reduced public shape. Authentication is optional; a bounty that is not publicly visible returns `404`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .bounties
        .submissions
        .list(
            &"bounty_id".to_string(),
            &BountiesSubmissionsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bounty_id:** `String` — The bounty whose public submissions to list (`bnty_` tag).
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListSubmissionsRequestStatus>` — Filter by lifecycle state.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only submissions created after this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only submissions created before this ISO 8601 timestamp.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListSubmissionsRequestOrder>` — Sort field.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListSubmissionsRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of submissions to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to paginate forwards from.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of submissions to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to paginate backwards from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bounties().submissions.<a href="/src/api/resources/bounties/submissions/client.rs">retrieve</a>(bounty_id: String, id: String) -> Result&lt;PublicBountySubmission, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves one of a bounty's publicly visible submissions in the reduced public shape — the read behind a shared proof link, whose submission is usually outside the bounty page's capped preview. Authentication is optional; a bounty that is not publicly visible, and a submission that is not publicly visible work on it, both return `404`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .bounties
        .submissions
        .retrieve(&"bounty_id".to_string(), &"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bounty_id:** `String` — The bounty the submission belongs to (`bnty_` tag).
    
</dd>
</dl>

<dl>
<dd>

**id:** `String` — The submission to retrieve (`btys_` tag).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Members Logs
<details><summary><code>client.members().logs.<a href="/src/api/resources/members/logs/client.rs">list</a>(id: String, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListLogsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists activity for a member and all of their non-drafted memberships, most recent first.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .members
        .logs
        .list(
            &"id".to_string(),
            &MembersLogsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Member ID (`mber_` tag).
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of log entries to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to paginate forwards from.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of log entries to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to paginate backwards from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Notifications Topics
<details><summary><code>client.notifications().topics.<a href="/src/api/resources/notifications/topics/client.rs">list</a>(topic_type: Option&lt;Option&lt;ListTopicsRequestTopicType&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListTopicsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the platform's visible notification topics — the categories users can set notification preferences on. App-created topics are internal and not returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .notifications
        .topics
        .list(
            &NotificationsTopicsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**topic_type:** `Option<ListTopicsRequestTopicType>` — Only return topics of this scope: `user` (member notifications) or `account_team` (team notifications).
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of topics to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns topics after this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Partners Businesses
<details><summary><code>client.partners().businesses.<a href="/src/api/resources/partners/businesses/client.rs">list</a>(status: Option&lt;Option&lt;ListBusinessesRequestStatus&gt;&gt;, has_earnings: Option&lt;Option&lt;bool&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListBusinessesRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListBusinessesRequestDirection&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;, referred_user_id: Option&lt;Option&lt;String&gt;&gt;, referred_username: Option&lt;Option&lt;String&gt;&gt;, tier: Option&lt;Option&lt;ListBusinessesRequestTier&gt;&gt;) -> Result&lt;ListBusinessesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the businesses the authenticated user referred onto Whop, most recent first.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .partners
        .businesses
        .list(
            &PartnersBusinessesListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**status:** `Option<ListBusinessesRequestStatus>` — Filter by referral status.
    
</dd>
</dl>

<dl>
<dd>

**has_earnings:** `Option<bool>` — When true, only businesses with pending or completed earnings paid to the caller.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of partner businesses to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of partner businesses to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to fetch the page before (from page_info.start_cursor).
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListBusinessesRequestOrder>` — The field to sort partner businesses by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListBusinessesRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return partner businesses created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return partner businesses created after this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**referred_user_id:** `Option<String>` — Filter to referrals attributed to this user. For first-tier referrals, this is the referred account owner; for second-tier referrals, this is the partner you recruited.
    
</dd>
</dl>

<dl>
<dd>

**referred_username:** `Option<String>` — Filter by the referred user's exact username. Ignored when `referred_user_id` is present.
    
</dd>
</dl>

<dl>
<dd>

**tier:** `Option<ListBusinessesRequestTier>` — Filter to referrals from a single tier: first, second, or blueprint.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.partners().businesses.<a href="/src/api/resources/partners/businesses/client.rs">retrieve</a>(id: String) -> Result&lt;RetrieveBusinessesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single referred business and its referral terms.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .partners
        .businesses
        .retrieve(&"id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The partner business ID (a coma_ identifier).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Partners Businesses Earnings
<details><summary><code>client.partners().businesses().earnings.<a href="/src/api/resources/partners/businesses/earnings/client.rs">list</a>(id: String, status: Option&lt;Option&lt;ListEarningsRequestStatus&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListEarningsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListEarningsRequestDirection&gt;&gt;, created_before: Option&lt;Option&lt;String&gt;&gt;, created_after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListEarningsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the earnings Whop pays out for one referred business's activity, most recent first.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .partners
        .businesses
        .earnings
        .list(
            &"id".to_string(),
            &PartnersBusinessesEarningsListQueryRequest {
                status: None,
                income_source: vec![],
                first: None,
                after: None,
                last: None,
                before: None,
                order: None,
                direction: None,
                created_before: None,
                created_after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The partner business ID (a coma_ identifier).
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListEarningsRequestStatus>` — Filter by earning status.
    
</dd>
</dl>

<dl>
<dd>

**income_source:** `Option<ListEarningsRequestIncomeSourceItem>` — Filter to earnings from these income sources. Repeat the parameter for each one (income_source=sales&income_source=ad_spend).
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListEarningsRequestOrder>` — The field to sort earnings by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListEarningsRequestDirection>` — Sort direction.
    
</dd>
</dl>

<dl>
<dd>

**created_before:** `Option<String>` — Only return earnings created before this timestamp.
    
</dd>
</dl>

<dl>
<dd>

**created_after:** `Option<String>` — Only return earnings created after this timestamp.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Payouts Methods
<details><summary><code>client.payouts().methods.<a href="/src/api/resources/payouts/methods/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, status: Option&lt;Option&lt;ListMethodsRequestStatus&gt;&gt;, amount: Option&lt;Option&lt;f64&gt;&gt;, currency: Option&lt;Option&lt;String&gt;&gt;, include_limits: Option&lt;Option&lt;bool&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListMethodsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the bank accounts, wallets, and crypto addresses an account or user can pay out to, newest first.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payouts
        .methods
        .list(
            &PayoutsMethodsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The owning account ID (a biz_ identifier). Provide this or user_id.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The owning user ID (a user_ identifier). Provide this or account_id.
    
</dd>
</dl>

<dl>
<dd>

**status:** `Option<ListMethodsRequestStatus>` — Optional status filter. `created` means saved but unused, `active` means a payout through it succeeded, `broken` means the last payout failed and the method needs fixing.
    
</dd>
</dl>

<dl>
<dd>

**amount:** `Option<f64>` — Optional payout amount in whole currency units, for example `250.00`. When provided, each method includes a quote with the estimated fee, amount received, and delivery date for that amount.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — Currency code of the amount, for example `usd`. Only meaningful with amount or include_limits.
    
</dd>
</dl>

<dl>
<dd>

**include_limits:** `Option<bool>` — When true, the response also carries limits — the live per-speed payout caps the account's payout requests are validated against, in the requested currency. Requires the payout:withdrawal:read scope.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of payout methods to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of payout methods to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to fetch the page before (from page_info.start_cursor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payouts().methods.<a href="/src/api/resources/payouts/methods/client.rs">create</a>(request: CreateMethodsRequest) -> Result&lt;CreateMethodsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Saves a new place an account or user can pay out to. Sensitive details are vaulted in transit and never stored raw.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payouts
        .methods
        .create(
            &CreateMethodsRequest {
                supported_payout_method_id: "podst_xxxxxxxxxxxxxx".to_string(),
                account_id: None,
                destination_currency: None,
                fields: None,
                is_default: None,
                nickname: None,
                user_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The account to add the payout method for, prefixed `biz_`. Provide this or `user_id`.
    
</dd>
</dl>

<dl>
<dd>

**destination_currency:** `Option<String>` — Currency the supported payout method delivers payouts in.
    
</dd>
</dl>

<dl>
<dd>

**fields:** `Option<std::collections::HashMap<String, String>>` — The supported payout method's required field values, keyed by field id — list them with `GET /payouts/supported_methods?supported_payout_method_id=...`. Field ids are stable `fld_` identifiers you may hardcode; they never change for a given field. A Basis Theory token id may be passed in place of a raw value. For a U.S. bank routing-number field, a raw nine-digit value must also pass the ABA checksum. A validation failure returns the method's full required_fields schema alongside the error. Required whenever the account details are supplied directly.
    
</dd>
</dl>

<dl>
<dd>

**is_default:** `Option<bool>` — Whether to make this the account's default payout method.
    
</dd>
</dl>

<dl>
<dd>

**nickname:** `Option<String>` — A label for the payout method, unique per destination.
    
</dd>
</dl>

<dl>
<dd>

**supported_payout_method_id:** `String` — The supported payout method to save (a podst_ identifier from a previous listing).
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The user to add the payout method for, prefixed `user_`. Provide this or `account_id`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payouts().methods.<a href="/src/api/resources/payouts/methods/client.rs">delete</a>(id: String) -> Result&lt;DeleteMethodsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a saved payout method so it can no longer receive payouts.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.payouts.methods.delete(&"id".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Payout method ID, prefixed `potk_`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payouts().methods.<a href="/src/api/resources/payouts/methods/client.rs">update</a>(id: String, request: UpdateMethodsRequest) -> Result&lt;UpdateMethodsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Changes the label used to identify a saved payout method.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payouts
        .methods
        .update(
            &"id".to_string(),
            &UpdateMethodsRequest {
                nickname: "Primary checking".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Payout method ID, prefixed `potk_`.
    
</dd>
</dl>

<dl>
<dd>

**nickname:** `String` — New label for the payout method, with at least one non-whitespace character and a maximum of 100 characters.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Payouts SupportedMethods
<details><summary><code>client.payouts().supported_methods.<a href="/src/api/resources/payouts/supported_methods/client.rs">list</a>(account_id: Option&lt;Option&lt;String&gt;&gt;, user_id: Option&lt;Option&lt;String&gt;&gt;, country: Option&lt;Option&lt;String&gt;&gt;, amount: Option&lt;Option&lt;f64&gt;&gt;, currency: Option&lt;Option&lt;String&gt;&gt;, supported_payout_method_id: Option&lt;Option&lt;String&gt;&gt;, destination_currency: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListSupportedMethodsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the payout methods an account or user is eligible to add.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .payouts
        .supported_methods
        .list(
            &PayoutsSupportedMethodsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — The owning account ID (a biz_ identifier). Provide this or user_id.
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` — The owning user ID (a user_ identifier). Provide this or account_id.
    
</dd>
</dl>

<dl>
<dd>

**country:** `Option<String>` — ISO 3166-1 alpha-2 country code for the bank account or wallet, such as `US`. Defaults to the country of supported_payout_method_id when one is given, otherwise the payout account's country.
    
</dd>
</dl>

<dl>
<dd>

**amount:** `Option<f64>` — Optional payout amount in whole currency units, for example `250.00`. When provided, each destination includes per-currency fee and delivery quotes.
    
</dd>
</dl>

<dl>
<dd>

**currency:** `Option<String>` — Currency code of the amount, for example `usd`. Only meaningful with amount.
    
</dd>
</dl>

<dl>
<dd>

**supported_payout_method_id:** `Option<String>` — Narrows the list to one supported payout method (a podst_ identifier) and includes the required_fields needed to save it as a payout method.
    
</dd>
</dl>

<dl>
<dd>

**destination_currency:** `Option<String>` — Currency the supported payout method would deliver payouts in. Only meaningful with supported_payout_method_id; required fields vary by destination currency.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — Number of supported payout methods to return from the start of the window.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — Cursor to fetch the page after (from page_info.end_cursor).
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — Number of supported payout methods to return from the end of the window.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — Cursor to fetch the page before (from page_info.start_cursor).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Users OauthGrants
<details><summary><code>client.users().oauth_grants.<a href="/src/api/resources/users/oauth_grants/client.rs">list</a>(app_id: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListOauthGrantsRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListOauthGrantsRequestDirection&gt;&gt;) -> Result&lt;ListOauthGrantsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the authenticated user's own OAuth grants — one per app they have authorized, per account they authorized it for. The list is always the caller's own; there is no parameter for reading another user's grants. Requires a user session: an API key or an OAuth token is refused, so an app can never enumerate the other apps a user has authorized.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .oauth_grants
        .list(
            &UsersOauthGrantsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**app_id:** `Option<String>` — Only return grants for this app, prefixed `app_`. An app the user has never authorized returns an empty list.
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of grants to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns grants after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of grants to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns grants before this position.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListOauthGrantsRequestOrder>` — The field to sort grants by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListOauthGrantsRequestDirection>` — Sort direction.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users().oauth_grants.<a href="/src/api/resources/users/oauth_grants/client.rs">create</a>(request: CreateOauthGrantsRequest) -> Result&lt;OauthGrant, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Completes the OAuth authorization step for the authenticated user: records their consent for the scopes an app asked for and mints the authorization code to hand back to it. Returns the grant, plus a `redirect_url` carrying that code — the one and only time it is returned. Exchange the code at `POST /oauth/token` with the verifier for `code_challenge`. Requires a user session, because consent has to come from the account holder: an API key or an OAuth token is refused, so an app can never authorize itself. Send an `Idempotency-Key` to make a retry safe — a replay returns the original `redirect_url` and its code rather than issuing a second one.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .oauth_grants
        .create(
            &CreateOauthGrantsRequest {
                client_id: "app_xxxxxxxxxxxxxx".to_string(),
                code_challenge: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
                code_challenge_method: CreateOauthGrantsRequestCodeChallengeMethod::S256,
                redirect_uri: "https://Booking.Shinetime.example:8443/oauth/Callback/".to_string(),
                requested_scopes: vec!["profile".to_string()],
                account_id: None,
                consent_shown: None,
                nonce: None,
                response_type: None,
                state: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<String>` — Authorize the app for one of the user's accounts rather than for the user alone, prefixed `biz_`. The user must have access to it.
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `String` — The app being authorized, prefixed `app_`.
    
</dd>
</dl>

<dl>
<dd>

**code_challenge:** `String` — The PKCE code challenge: the base64url-encoded SHA-256 of your code verifier, without padding.
    
</dd>
</dl>

<dl>
<dd>

**code_challenge_method:** `CreateOauthGrantsRequestCodeChallengeMethod` — How `code_challenge` was derived. Only `S256` is accepted.
    
</dd>
</dl>

<dl>
<dd>

**consent_shown:** `Option<bool>` — Whether the consent UI listed these scopes for the user. Sending `false` succeeds only when the user has already granted every scope requested.
    
</dd>
</dl>

<dl>
<dd>

**nonce:** `Option<String>` — OIDC nonce, echoed into the resulting ID token. Required when `requested_scopes` includes `openid`.
    
</dd>
</dl>

<dl>
<dd>

**redirect_uri:** `String` — Where to send the user once they have consented. Must match one of the app's registered redirect URIs exactly — it is compared as a string, not normalized.
    
</dd>
</dl>

<dl>
<dd>

**requested_scopes:** `Vec<String>` — The permissions the app is asking for, for example `member:basic:read`. `GET /api_keys/permissions` names and describes each one. Granting adds to whatever the user already granted this app rather than replacing it.
    
</dd>
</dl>

<dl>
<dd>

**response_type:** `Option<CreateOauthGrantsRequestResponseType>` — The OAuth response type. Only `code` is accepted; defaults to `code`.
    
</dd>
</dl>

<dl>
<dd>

**state:** `Option<String>` — Opaque value appended to `redirect_url` unchanged, for the client to correlate the response with its request.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Users Passkeys
<details><summary><code>client.users().passkeys.<a href="/src/api/resources/users/passkeys/client.rs">list</a>(first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;, last: Option&lt;Option&lt;i64&gt;&gt;, before: Option&lt;Option&lt;String&gt;&gt;, order: Option&lt;Option&lt;ListPasskeysRequestOrder&gt;&gt;, direction: Option&lt;Option&lt;ListPasskeysRequestDirection&gt;&gt;) -> Result&lt;ListPasskeysResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the authenticated user's own passkeys, newest first. The list is always the caller's own; there is no parameter for reading another user's passkeys. Requires a user session: an API key or an OAuth token is refused, because a passkey confirms the account holder before a sensitive action and no app may enumerate one.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .passkeys
        .list(
            &UsersPasskeysListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**first:** `Option<i64>` — The number of passkeys to return (default 20, max 100).
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns passkeys after this position.
    
</dd>
</dl>

<dl>
<dd>

**last:** `Option<i64>` — The number of passkeys to return from the end of the range.
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<String>` — A cursor; returns passkeys before this position.
    
</dd>
</dl>

<dl>
<dd>

**order:** `Option<ListPasskeysRequestOrder>` — The field to sort passkeys by.
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Option<ListPasskeysRequestDirection>` — Sort direction.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users().passkeys.<a href="/src/api/resources/users/passkeys/client.rs">create</a>(request: CreatePasskeysRequest) -> Result&lt;Passkey, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Registers a passkey for the authenticated user from the attestation a browser produced for a `registration` challenge. Mint that challenge first with `POST /users/me/passkeys/challenge`; it is single-use and expires 5 minutes after it is issued. Requires a user session.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .passkeys
        .create(
            &CreatePasskeysRequest {
                attestation_object: "YXR0ZXN0YXRpb24".to_string(),
                client_data_json: "Y2xpZW50LWRhdGE".to_string(),
                credential_id: "bmV3LWNyZWRlbnRpYWw".to_string(),
                nickname: "Work laptop".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**attestation_object:** `String` — The `attestationObject` from the WebAuthn attestation response, base64url-encoded.
    
</dd>
</dl>

<dl>
<dd>

**client_data_json:** `String` — The `clientDataJSON` from the WebAuthn attestation response, base64url-encoded.
    
</dd>
</dl>

<dl>
<dd>

**credential_id:** `String` — The WebAuthn credential ID the authenticator returned, base64url-encoded.
    
</dd>
</dl>

<dl>
<dd>

**nickname:** `String` — A name for this passkey, usually the device it lives on. 255 characters or fewer.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users().passkeys.<a href="/src/api/resources/users/passkeys/client.rs">challenge</a>(request: ChallengePasskeysRequest) -> Result&lt;ChallengePasskeysResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Mints the challenge a browser needs to run a WebAuthn ceremony against the authenticated user's own passkeys. A `registration` challenge enrolls a new passkey; a `deletion` challenge is bound to the one passkey named by `passkey_id` and proves the user still holds it. Challenges are single-use and expire 5 minutes after they are issued, so send a fresh `Idempotency-Key` per ceremony — a replayed key returns the original challenge, which may already have expired. Requires a user session.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .passkeys
        .challenge(
            &ChallengePasskeysRequest {
                challenge_type: ChallengePasskeysRequestChallengeType::Registration,
                passkey_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**challenge_type:** `ChallengePasskeysRequestChallengeType` — The ceremony this challenge is for.
    
</dd>
</dl>

<dl>
<dd>

**passkey_id:** `Option<String>` — The passkey the ceremony targets, prefixed `wcred_`. Required when `challenge_type` is `deletion`, ignored otherwise.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users().passkeys.<a href="/src/api/resources/users/passkeys/client.rs">delete</a>(id: String, request: DeletePasskeysRequest) -> Result&lt;DeletePasskeysResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes one of the authenticated user's own passkeys. The request body carries a WebAuthn assertion from the passkey being deleted, so possession of the credential is proven before it is removed: mint a `deletion` challenge for it first, run the ceremony with that passkey, and send the result here. Deleting the user's last passkey is allowed — their other step-up factors remain. Requires a user session.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .passkeys
        .delete(
            &"id".to_string(),
            &DeletePasskeysRequest {
                authenticator_data: "YXV0aGVudGljYXRvci1kYXRh".to_string(),
                client_data_json: "Y2xpZW50LWRhdGE".to_string(),
                signature: "c2lnbmF0dXJl".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Passkey ID, prefixed `wcred_`.
    
</dd>
</dl>

<dl>
<dd>

**authenticator_data:** `String` — The `authenticatorData` from the WebAuthn assertion, base64url-encoded.
    
</dd>
</dl>

<dl>
<dd>

**client_data_json:** `String` — The `clientDataJSON` from the WebAuthn assertion, base64url-encoded.
    
</dd>
</dl>

<dl>
<dd>

**signature:** `String` — The `signature` from the WebAuthn assertion, base64url-encoded.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Users Preferences
<details><summary><code>client.users().preferences.<a href="/src/api/resources/users/preferences/client.rs">retrieve</a>() -> Result&lt;UserPreferences, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the authenticated user's settings document. Addressed only as `me` — the document always belongs to the session user.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client.users.preferences.retrieve(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.users().preferences.<a href="/src/api/resources/users/preferences/client.rs">update</a>(request: UpdatePreferencesRequest) -> Result&lt;UserPreferences, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates the authenticated user's settings document. Replaces the top-level keys it is given and leaves the rest untouched.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .preferences
        .update(
            &UpdatePreferencesRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bounty_worker_onboarding_dismissed:** `Option<bool>` — Whether the user has dismissed the first-time bounty worker onboarding. Set to `false` to show it again.
    
</dd>
</dl>

<dl>
<dd>

**investigation_enabled:** `Option<bool>` — Whether investigation mode is enabled for the user. Only meaningful for staff users with investigation access.
    
</dd>
</dl>

<dl>
<dd>

**terms_accepted:** `Option<bool>` — Records the user's acceptance of Whop's terms and policies. Only `true` is accepted — the server stamps `terms_accepted_at` and acceptance cannot be withdrawn here.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Users Preferences Notifications
<details><summary><code>client.users().preferences().notifications.<a href="/src/api/resources/users/preferences/notifications/client.rs">set</a>(request: SetNotificationsRequest) -> Result&lt;SetNotificationsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Sets the authenticated user's notification preferences. Each preference is addressed by `scope`, not by id, so a scope read back from either list endpoint can be sent straight here.

A scope naming an experience with no topic sets that experience's level, and accepts all three levels. Any other scope sets a topic override, which is binary — `all` or `nothing` — and requires a `channel`.

`level: null` clears the preference. Preferences are stored as overrides, so clearing one means the scope inherits its default again rather than being switched off.

The batch is applied in one transaction: if any entry is rejected, none are written. Experience levels are applied before topic overrides, because setting a level replaces every topic preference for that experience — so an override sent alongside a level wins. The response reports what each scope now resolves to, in the order the entries were sent.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .preferences
        .notifications
        .set(
            &SetNotificationsRequest {
                preferences: vec![SetNotificationsRequestPreferencesItem {
                    scope: SetNotificationsRequestPreferencesItemScope {
                        ..Default::default()
                    },
                    ..Default::default()
                }],
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**preferences:** `Vec<SetNotificationsRequestPreferencesItem>` — The preferences to set, at most 100 per request.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Users Preferences Notifications Experiences
<details><summary><code>client.users().preferences().notifications().experiences.<a href="/src/api/resources/users/preferences/notifications/experiences/client.rs">list</a>(first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListExperiencesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the authenticated user's per-experience notification levels. Experiences the user never set a level for are omitted — their effective level is `all`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .preferences
        .notifications
        .experiences
        .list(
            &UsersPreferencesNotificationsExperiencesListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**first:** `Option<i64>` — The number of preferences to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns preferences after this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Users Preferences Notifications Topics
<details><summary><code>client.users().preferences().notifications().topics.<a href="/src/api/resources/users/preferences/notifications/topics/client.rs">list</a>(channel: Option&lt;Option&lt;ListTopicsRequestChannel&gt;&gt;, account_id: Option&lt;Option&lt;String&gt;&gt;, team_account_id: Option&lt;Option&lt;String&gt;&gt;, experience_id: Option&lt;Option&lt;String&gt;&gt;, topic_id: Option&lt;Option&lt;String&gt;&gt;, first: Option&lt;Option&lt;i64&gt;&gt;, after: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;ListTopicsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists the authenticated user's topic-scoped notification preferences, plus user-agnostic platform defaults. Each filter matches preferences scoped to its value or not narrowed on that dimension. Per-experience levels are listed separately, by `GET /users/me/preferences/notifications/experiences`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use whop_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = Whop::new(config).expect("Failed to build client");
    client
        .users
        .preferences
        .notifications
        .topics
        .list(
            &UsersPreferencesNotificationsTopicsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**channel:** `Option<ListTopicsRequestChannel>` — Only return preferences for this delivery channel (or not narrowed to a channel).
    
</dd>
</dl>

<dl>
<dd>

**account_id:** `Option<String>` — Only return preferences scoped to this account's member notifications (`biz_` tag).
    
</dd>
</dl>

<dl>
<dd>

**team_account_id:** `Option<String>` — Only return preferences scoped to this account's team notifications (`biz_` tag).
    
</dd>
</dl>

<dl>
<dd>

**experience_id:** `Option<String>` — Only return preferences scoped to this experience (`exp_` tag).
    
</dd>
</dl>

<dl>
<dd>

**topic_id:** `Option<String>` — Only return preferences scoped to this notification topic (`topic_` tag).
    
</dd>
</dl>

<dl>
<dd>

**first:** `Option<i64>` — The number of preferences to return.
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<String>` — A cursor; returns preferences after this position.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

