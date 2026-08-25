pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionEntry {
    /// The waitlist entry created by the confirm, prefixed `entry_`.
    #[serde(default)]
    pub id: String,
    /// Where the join stands. `succeeded` — the join stands (whether the seller accepts it is the entry resource's own story). `requires_action` — the card save has a step left, carried by `next_action`. `processing` — the save is being decided; hold. `failed` — the card save died: the buyer is NOT on the waitlist, and needs a fresh checkout to join.
    pub status: CheckoutSessionEntryStatus,
}

impl CheckoutSessionEntry {
    pub fn builder() -> CheckoutSessionEntryBuilder {
        <CheckoutSessionEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionEntryBuilder {
    id: Option<String>,
    status: Option<CheckoutSessionEntryStatus>,
}

impl CheckoutSessionEntryBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CheckoutSessionEntryStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CheckoutSessionEntryBuilder::id)
    /// - [`status`](CheckoutSessionEntryBuilder::status)
    pub fn build(self) -> Result<CheckoutSessionEntry, BuildError> {
        Ok(CheckoutSessionEntry {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
