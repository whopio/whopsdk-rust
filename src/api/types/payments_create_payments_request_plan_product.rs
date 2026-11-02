pub use crate::prelude::*;

/// Find or create a product by external identifier. Mutually exclusive with product_id.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePaymentsRequestPlanProduct {
    /// Whether to collect a shipping address at checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect_shipping_address: Option<bool>,
    /// Custom card statement descriptor for the product, starting with WHOP*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_statement_descriptor: Option<String>,
    /// Product description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Your unique identifier for the product.
    #[serde(default)]
    pub external_identifier: String,
    /// Percentage of revenue paid to global affiliates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_affiliate_percentage: Option<f64>,
    /// Global affiliate program status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_affiliate_status: Option<CreatePaymentsRequestPlanProductGlobalAffiliateStatus>,
    /// Product headline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// Product tax code identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_tax_code_id: Option<String>,
    /// Where to redirect the buyer after purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_purchase_url: Option<String>,
    /// Product route.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Product title.
    #[serde(default)]
    pub title: String,
    /// Product visibility. Defaults to hidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<CreatePaymentsRequestPlanProductVisibility>,
}

impl CreatePaymentsRequestPlanProduct {
    pub fn builder() -> CreatePaymentsRequestPlanProductBuilder {
        <CreatePaymentsRequestPlanProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestPlanProductBuilder {
    collect_shipping_address: Option<bool>,
    custom_statement_descriptor: Option<String>,
    description: Option<String>,
    external_identifier: Option<String>,
    global_affiliate_percentage: Option<f64>,
    global_affiliate_status: Option<CreatePaymentsRequestPlanProductGlobalAffiliateStatus>,
    headline: Option<String>,
    product_tax_code_id: Option<String>,
    redirect_purchase_url: Option<String>,
    route: Option<String>,
    title: Option<String>,
    visibility: Option<CreatePaymentsRequestPlanProductVisibility>,
}

impl CreatePaymentsRequestPlanProductBuilder {
    pub fn collect_shipping_address(mut self, value: bool) -> Self {
        self.collect_shipping_address = Some(value);
        self
    }

    pub fn custom_statement_descriptor(mut self, value: impl Into<String>) -> Self {
        self.custom_statement_descriptor = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn external_identifier(mut self, value: impl Into<String>) -> Self {
        self.external_identifier = Some(value.into());
        self
    }

    pub fn global_affiliate_percentage(mut self, value: f64) -> Self {
        self.global_affiliate_percentage = Some(value);
        self
    }

    pub fn global_affiliate_status(
        mut self,
        value: CreatePaymentsRequestPlanProductGlobalAffiliateStatus,
    ) -> Self {
        self.global_affiliate_status = Some(value);
        self
    }

    pub fn headline(mut self, value: impl Into<String>) -> Self {
        self.headline = Some(value.into());
        self
    }

    pub fn product_tax_code_id(mut self, value: impl Into<String>) -> Self {
        self.product_tax_code_id = Some(value.into());
        self
    }

    pub fn redirect_purchase_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_purchase_url = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: CreatePaymentsRequestPlanProductVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequestPlanProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_identifier`](CreatePaymentsRequestPlanProductBuilder::external_identifier)
    /// - [`title`](CreatePaymentsRequestPlanProductBuilder::title)
    pub fn build(self) -> Result<CreatePaymentsRequestPlanProduct, BuildError> {
        Ok(CreatePaymentsRequestPlanProduct {
            collect_shipping_address: self.collect_shipping_address,
            custom_statement_descriptor: self.custom_statement_descriptor,
            description: self.description,
            external_identifier: self
                .external_identifier
                .ok_or_else(|| BuildError::missing_field("external_identifier"))?,
            global_affiliate_percentage: self.global_affiliate_percentage,
            global_affiliate_status: self.global_affiliate_status,
            headline: self.headline,
            product_tax_code_id: self.product_tax_code_id,
            redirect_purchase_url: self.redirect_purchase_url,
            route: self.route,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            visibility: self.visibility,
        })
    }
}
