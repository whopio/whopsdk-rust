pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FormCompanyAccountsRequestFoundersItem {
    /// Founder's personal address.
    #[serde(default)]
    pub address: FormCompanyAccountsRequestFoundersItemAddress,
    /// Formatted as `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub first_name: String,
    /// Marks the responsible party for the filing. Exactly one founder must be primary.
    #[serde(default)]
    pub is_primary: bool,
    #[serde(default)]
    pub last_name: String,
    /// The founder's ownership share: greater than `0`, at most `100`. Shares across founders must total `100`. Required when `entity_type` is `llc`; ignored for C-Corps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ownership_percentage: Option<f64>,
    /// Phone number in E.164 format, for example `+12125550100`.
    #[serde(default)]
    pub phone: String,
    /// Officer roles held by the member — one member can hold several. Required (at least one role) for every member when `entity_type` is `c_corp`; ignored for LLCs. Across all members every role must be covered; `president`, `secretary` and `treasurer` may each be held by only one member, while `director` may repeat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<FormCompanyAccountsRequestFoundersItemRolesItem>>,
    /// The founder's US Social Security Number. Leave empty if the founder is not a US resident. Non-US founders can request expedited EIN processing via the `expedite_ein` option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
}

impl FormCompanyAccountsRequestFoundersItem {
    pub fn builder() -> FormCompanyAccountsRequestFoundersItemBuilder {
        <FormCompanyAccountsRequestFoundersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FormCompanyAccountsRequestFoundersItemBuilder {
    address: Option<FormCompanyAccountsRequestFoundersItemAddress>,
    date_of_birth: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    is_primary: Option<bool>,
    last_name: Option<String>,
    ownership_percentage: Option<f64>,
    phone: Option<String>,
    roles: Option<Vec<FormCompanyAccountsRequestFoundersItemRolesItem>>,
    ssn: Option<String>,
}

impl FormCompanyAccountsRequestFoundersItemBuilder {
    pub fn address(mut self, value: FormCompanyAccountsRequestFoundersItemAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn is_primary(mut self, value: bool) -> Self {
        self.is_primary = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn ownership_percentage(mut self, value: f64) -> Self {
        self.ownership_percentage = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn roles(mut self, value: Vec<FormCompanyAccountsRequestFoundersItemRolesItem>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn ssn(mut self, value: impl Into<String>) -> Self {
        self.ssn = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FormCompanyAccountsRequestFoundersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`address`](FormCompanyAccountsRequestFoundersItemBuilder::address)
    /// - [`email`](FormCompanyAccountsRequestFoundersItemBuilder::email)
    /// - [`first_name`](FormCompanyAccountsRequestFoundersItemBuilder::first_name)
    /// - [`is_primary`](FormCompanyAccountsRequestFoundersItemBuilder::is_primary)
    /// - [`last_name`](FormCompanyAccountsRequestFoundersItemBuilder::last_name)
    /// - [`phone`](FormCompanyAccountsRequestFoundersItemBuilder::phone)
    pub fn build(self) -> Result<FormCompanyAccountsRequestFoundersItem, BuildError> {
        Ok(FormCompanyAccountsRequestFoundersItem {
            address: self
                .address
                .ok_or_else(|| BuildError::missing_field("address"))?,
            date_of_birth: self.date_of_birth,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            is_primary: self
                .is_primary
                .ok_or_else(|| BuildError::missing_field("is_primary"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            ownership_percentage: self.ownership_percentage,
            phone: self
                .phone
                .ok_or_else(|| BuildError::missing_field("phone"))?,
            roles: self.roles,
            ssn: self.ssn,
        })
    }
}
