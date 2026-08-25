pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutSessionBreakdownDisplaySections {
    #[serde(default)]
    pub charges: Vec<CheckoutSessionBreakdownDisplayRow>,
    #[serde(default)]
    pub fee_breakdown: Vec<CheckoutSessionBreakdownDisplayRow>,
    #[serde(default)]
    pub line_items: Vec<CheckoutSessionBreakdownDisplayRow>,
    #[serde(default)]
    pub totals: Vec<CheckoutSessionBreakdownDisplayRow>,
}

impl CheckoutSessionBreakdownDisplaySections {
    pub fn builder() -> CheckoutSessionBreakdownDisplaySectionsBuilder {
        <CheckoutSessionBreakdownDisplaySectionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionBreakdownDisplaySectionsBuilder {
    charges: Option<Vec<CheckoutSessionBreakdownDisplayRow>>,
    fee_breakdown: Option<Vec<CheckoutSessionBreakdownDisplayRow>>,
    line_items: Option<Vec<CheckoutSessionBreakdownDisplayRow>>,
    totals: Option<Vec<CheckoutSessionBreakdownDisplayRow>>,
}

impl CheckoutSessionBreakdownDisplaySectionsBuilder {
    pub fn charges(mut self, value: Vec<CheckoutSessionBreakdownDisplayRow>) -> Self {
        self.charges = Some(value);
        self
    }

    pub fn fee_breakdown(mut self, value: Vec<CheckoutSessionBreakdownDisplayRow>) -> Self {
        self.fee_breakdown = Some(value);
        self
    }

    pub fn line_items(mut self, value: Vec<CheckoutSessionBreakdownDisplayRow>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn totals(mut self, value: Vec<CheckoutSessionBreakdownDisplayRow>) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionBreakdownDisplaySections`].
    /// This method will fail if any of the following fields are not set:
    /// - [`charges`](CheckoutSessionBreakdownDisplaySectionsBuilder::charges)
    /// - [`fee_breakdown`](CheckoutSessionBreakdownDisplaySectionsBuilder::fee_breakdown)
    /// - [`line_items`](CheckoutSessionBreakdownDisplaySectionsBuilder::line_items)
    /// - [`totals`](CheckoutSessionBreakdownDisplaySectionsBuilder::totals)
    pub fn build(self) -> Result<CheckoutSessionBreakdownDisplaySections, BuildError> {
        Ok(CheckoutSessionBreakdownDisplaySections {
            charges: self
                .charges
                .ok_or_else(|| BuildError::missing_field("charges"))?,
            fee_breakdown: self
                .fee_breakdown
                .ok_or_else(|| BuildError::missing_field("fee_breakdown"))?,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            totals: self
                .totals
                .ok_or_else(|| BuildError::missing_field("totals"))?,
        })
    }
}
