pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEventsResponseDataItemRelatedPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ListEventsResponseDataItemRelatedPlan {
    pub fn builder() -> ListEventsResponseDataItemRelatedPlanBuilder {
        <ListEventsResponseDataItemRelatedPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemRelatedPlanBuilder {
    billing_period: Option<i64>,
    currency: Option<String>,
    id: Option<String>,
    initial_price: Option<f64>,
    renewal_price: Option<f64>,
    title: Option<String>,
}

impl ListEventsResponseDataItemRelatedPlanBuilder {
    pub fn billing_period(mut self, value: i64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn initial_price(mut self, value: f64) -> Self {
        self.initial_price = Some(value);
        self
    }

    pub fn renewal_price(mut self, value: f64) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemRelatedPlan`].
    pub fn build(self) -> Result<ListEventsResponseDataItemRelatedPlan, BuildError> {
        Ok(ListEventsResponseDataItemRelatedPlan {
            billing_period: self.billing_period,
            currency: self.currency,
            id: self.id,
            initial_price: self.initial_price,
            renewal_price: self.renewal_price,
            title: self.title,
        })
    }
}
