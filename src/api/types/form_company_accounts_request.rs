pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FormCompanyAccountsRequest {
    /// Company mailing address. Required unless `use_registered_agent` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_address: Option<FormCompanyAccountsRequestBusinessAddress>,
    /// Legal name for the new company.
    #[serde(default)]
    pub business_name: String,
    /// Business phone number in E.164 format, for example `+12125550100`. Required unless `use_registered_agent` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_phone: Option<String>,
    /// High-level business category, from the Whop business taxonomy. Valid values are listed on [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary).
    #[serde(default)]
    pub business_type: String,
    /// Company website URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_website: Option<String>,
    /// Legal entity ending appended to `business_name`. LLC formations accept `LLC`, `L.L.C`, `L.L.C.` or `Limited Liability Company` and default to `LLC`; C-Corp formations accept `Inc`, `Inc.`, `Incorporated`, `Corp.`, `Corporation`, `C Corp`, `C Corporation`, `CCorp` or `Company` and default to `Inc.`. Unrecognized values fall back to the default for the entity type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_suffix: Option<FormCompanyAccountsRequestEntitySuffix>,
    /// Legal entity type to form. Defaults to `llc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<FormCompanyAccountsRequestEntityType>,
    /// Request expedited EIN processing for an additional fee. Available only when no founder supplies an SSN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expedite_ein: Option<bool>,
    /// Two-letter code of the US state (or `DC`) to form the company in. We recommend `WY` because Wyoming formations are completed the same day.
    pub formation_state: FormCompanyAccountsRequestFormationState,
    /// The company's founders. Exactly one must be marked `is_primary` — the responsible party for the filing.
    #[serde(default)]
    pub founders: Vec<FormCompanyAccountsRequestFoundersItem>,
    /// Industry group, from the Whop business taxonomy. Valid values are listed on [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary).
    #[serde(default)]
    pub industry_group: String,
    /// Specific industry vertical, from the Whop business taxonomy. Valid values are listed on [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary).
    #[serde(default)]
    pub industry_type: String,
    /// Authorized share structure. Required when `entity_type` is `c_corp`; ignored for LLCs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_structure: Option<FormCompanyAccountsRequestShareStructure>,
    /// Use the registered agent's address as the company address instead of `business_address`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_registered_agent: Option<bool>,
}

impl FormCompanyAccountsRequest {
    pub fn builder() -> FormCompanyAccountsRequestBuilder {
        <FormCompanyAccountsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FormCompanyAccountsRequestBuilder {
    business_address: Option<FormCompanyAccountsRequestBusinessAddress>,
    business_name: Option<String>,
    business_phone: Option<String>,
    business_type: Option<String>,
    business_website: Option<String>,
    entity_suffix: Option<FormCompanyAccountsRequestEntitySuffix>,
    entity_type: Option<FormCompanyAccountsRequestEntityType>,
    expedite_ein: Option<bool>,
    formation_state: Option<FormCompanyAccountsRequestFormationState>,
    founders: Option<Vec<FormCompanyAccountsRequestFoundersItem>>,
    industry_group: Option<String>,
    industry_type: Option<String>,
    share_structure: Option<FormCompanyAccountsRequestShareStructure>,
    use_registered_agent: Option<bool>,
}

impl FormCompanyAccountsRequestBuilder {
    pub fn business_address(mut self, value: FormCompanyAccountsRequestBusinessAddress) -> Self {
        self.business_address = Some(value);
        self
    }

    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn business_phone(mut self, value: impl Into<String>) -> Self {
        self.business_phone = Some(value.into());
        self
    }

    pub fn business_type(mut self, value: impl Into<String>) -> Self {
        self.business_type = Some(value.into());
        self
    }

    pub fn business_website(mut self, value: impl Into<String>) -> Self {
        self.business_website = Some(value.into());
        self
    }

    pub fn entity_suffix(mut self, value: FormCompanyAccountsRequestEntitySuffix) -> Self {
        self.entity_suffix = Some(value);
        self
    }

    pub fn entity_type(mut self, value: FormCompanyAccountsRequestEntityType) -> Self {
        self.entity_type = Some(value);
        self
    }

    pub fn expedite_ein(mut self, value: bool) -> Self {
        self.expedite_ein = Some(value);
        self
    }

    pub fn formation_state(mut self, value: FormCompanyAccountsRequestFormationState) -> Self {
        self.formation_state = Some(value);
        self
    }

    pub fn founders(mut self, value: Vec<FormCompanyAccountsRequestFoundersItem>) -> Self {
        self.founders = Some(value);
        self
    }

    pub fn industry_group(mut self, value: impl Into<String>) -> Self {
        self.industry_group = Some(value.into());
        self
    }

    pub fn industry_type(mut self, value: impl Into<String>) -> Self {
        self.industry_type = Some(value.into());
        self
    }

    pub fn share_structure(mut self, value: FormCompanyAccountsRequestShareStructure) -> Self {
        self.share_structure = Some(value);
        self
    }

    pub fn use_registered_agent(mut self, value: bool) -> Self {
        self.use_registered_agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FormCompanyAccountsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_name`](FormCompanyAccountsRequestBuilder::business_name)
    /// - [`business_type`](FormCompanyAccountsRequestBuilder::business_type)
    /// - [`formation_state`](FormCompanyAccountsRequestBuilder::formation_state)
    /// - [`founders`](FormCompanyAccountsRequestBuilder::founders)
    /// - [`industry_group`](FormCompanyAccountsRequestBuilder::industry_group)
    /// - [`industry_type`](FormCompanyAccountsRequestBuilder::industry_type)
    pub fn build(self) -> Result<FormCompanyAccountsRequest, BuildError> {
        Ok(FormCompanyAccountsRequest {
            business_address: self.business_address,
            business_name: self
                .business_name
                .ok_or_else(|| BuildError::missing_field("business_name"))?,
            business_phone: self.business_phone,
            business_type: self
                .business_type
                .ok_or_else(|| BuildError::missing_field("business_type"))?,
            business_website: self.business_website,
            entity_suffix: self.entity_suffix,
            entity_type: self.entity_type,
            expedite_ein: self.expedite_ein,
            formation_state: self
                .formation_state
                .ok_or_else(|| BuildError::missing_field("formation_state"))?,
            founders: self
                .founders
                .ok_or_else(|| BuildError::missing_field("founders"))?,
            industry_group: self
                .industry_group
                .ok_or_else(|| BuildError::missing_field("industry_group"))?,
            industry_type: self
                .industry_type
                .ok_or_else(|| BuildError::missing_field("industry_type"))?,
            share_structure: self.share_structure,
            use_registered_agent: self.use_registered_agent,
        })
    }
}
