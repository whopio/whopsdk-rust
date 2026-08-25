pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionBreakdownDisplay {
    /// The hero: the figure (or trial length, or the word "Free") and the one-line context under it.
    pub headline: CheckoutSessionBreakdownDisplayHeadline,
    /// The itemized sections below — line items, charges, totals, and the collapsible fee breakdown. All empty on a free checkout.
    #[serde(default)]
    pub sections: CheckoutSessionBreakdownDisplaySections,
}

impl CheckoutSessionBreakdownDisplay {
    pub fn builder() -> CheckoutSessionBreakdownDisplayBuilder {
        <CheckoutSessionBreakdownDisplayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionBreakdownDisplayBuilder {
    headline: Option<CheckoutSessionBreakdownDisplayHeadline>,
    sections: Option<CheckoutSessionBreakdownDisplaySections>,
}

impl CheckoutSessionBreakdownDisplayBuilder {
    pub fn headline(mut self, value: CheckoutSessionBreakdownDisplayHeadline) -> Self {
        self.headline = Some(value);
        self
    }

    pub fn sections(mut self, value: CheckoutSessionBreakdownDisplaySections) -> Self {
        self.sections = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionBreakdownDisplay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`headline`](CheckoutSessionBreakdownDisplayBuilder::headline)
    /// - [`sections`](CheckoutSessionBreakdownDisplayBuilder::sections)
    pub fn build(self) -> Result<CheckoutSessionBreakdownDisplay, BuildError> {
        Ok(CheckoutSessionBreakdownDisplay {
            headline: self
                .headline
                .ok_or_else(|| BuildError::missing_field("headline"))?,
            sections: self
                .sections
                .ok_or_else(|| BuildError::missing_field("sections"))?,
        })
    }
}
