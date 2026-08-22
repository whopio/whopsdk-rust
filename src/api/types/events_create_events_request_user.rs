pub use crate::prelude::*;

/// User identity and profile data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEventsRequestUser {
    /// An anonymous identifier for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_id: Option<String>,
    /// Date of birth (YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// An external identifier for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Gender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<CreateEventsRequestUserGender>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// A second anonymous identifier to link to this user (e.g. captured across an iframe boundary).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_anonymous_id: Option<String>,
    /// A wuid from a linked frame, captured across an iframe boundary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_wuid: Option<String>,
    /// The Whop member ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The Whop membership ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
    /// Full display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The Whop user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl CreateEventsRequestUser {
    pub fn builder() -> CreateEventsRequestUserBuilder {
        <CreateEventsRequestUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEventsRequestUserBuilder {
    anonymous_id: Option<String>,
    birthdate: Option<String>,
    city: Option<String>,
    country: Option<String>,
    email: Option<String>,
    external_id: Option<String>,
    first_name: Option<String>,
    gender: Option<CreateEventsRequestUserGender>,
    last_name: Option<String>,
    linked_anonymous_id: Option<String>,
    linked_wuid: Option<String>,
    member_id: Option<String>,
    membership_id: Option<String>,
    name: Option<String>,
    phone: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
    user_id: Option<String>,
    username: Option<String>,
}

impl CreateEventsRequestUserBuilder {
    pub fn anonymous_id(mut self, value: impl Into<String>) -> Self {
        self.anonymous_id = Some(value.into());
        self
    }

    pub fn birthdate(mut self, value: impl Into<String>) -> Self {
        self.birthdate = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn gender(mut self, value: CreateEventsRequestUserGender) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn linked_anonymous_id(mut self, value: impl Into<String>) -> Self {
        self.linked_anonymous_id = Some(value.into());
        self
    }

    pub fn linked_wuid(mut self, value: impl Into<String>) -> Self {
        self.linked_wuid = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn membership_id(mut self, value: impl Into<String>) -> Self {
        self.membership_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateEventsRequestUser`].
    pub fn build(self) -> Result<CreateEventsRequestUser, BuildError> {
        Ok(CreateEventsRequestUser {
            anonymous_id: self.anonymous_id,
            birthdate: self.birthdate,
            city: self.city,
            country: self.country,
            email: self.email,
            external_id: self.external_id,
            first_name: self.first_name,
            gender: self.gender,
            last_name: self.last_name,
            linked_anonymous_id: self.linked_anonymous_id,
            linked_wuid: self.linked_wuid,
            member_id: self.member_id,
            membership_id: self.membership_id,
            name: self.name,
            phone: self.phone,
            postal_code: self.postal_code,
            state: self.state,
            user_id: self.user_id,
            username: self.username,
        })
    }
}
