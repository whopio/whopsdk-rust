pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionRequirement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<CheckoutSessionCustomField>>,
    /// What to collect. `custom_password` — the plan is password-protected: send the buyer's answer through update, and the entry disappears once it is right; confirm refuses while it stands. `email` — the buyer's email address; it identifies who the purchase is for, and confirm has no other way to resolve them. `terms` — explicit acceptance, sent as the `tos_accepted` attestation on confirm; show the seller's documents from `account.terms`. `custom_fields` — the seller's own questions, published in `fields`; answer them through `custom_field_responses`. `shipping_address` — a postal address for physical goods, set through `shipping_address`. `phone_number` — the seller collects buyer phone numbers: set one through update and it is recorded against the order; a missing number never refuses the confirm. The list is closed; new entries are added deliberately, so an unrecognized type is safe to skip.
    pub r#type: CheckoutSessionRequirementType,
}

impl CheckoutSessionRequirement {
    pub fn builder() -> CheckoutSessionRequirementBuilder {
        <CheckoutSessionRequirementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionRequirementBuilder {
    fields: Option<Vec<CheckoutSessionCustomField>>,
    r#type: Option<CheckoutSessionRequirementType>,
}

impl CheckoutSessionRequirementBuilder {
    pub fn fields(mut self, value: Vec<CheckoutSessionCustomField>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn r#type(mut self, value: CheckoutSessionRequirementType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionRequirement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CheckoutSessionRequirementBuilder::r#type)
    pub fn build(self) -> Result<CheckoutSessionRequirement, BuildError> {
        Ok(CheckoutSessionRequirement {
            fields: self.fields,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
