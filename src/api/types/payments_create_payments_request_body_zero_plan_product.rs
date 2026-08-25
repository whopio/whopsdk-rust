pub use crate::prelude::*;

/// Pass this object to create a new product for this plan. We will use the product external identifier to find or create an existing product.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePaymentsRequestBodyZeroPlanProduct {
    /// Whether or not to collect shipping information at checkout from the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collect_shipping_address: Option<bool>,
    /// The custom statement descriptor for the product i.e. WHOP*SPORTS, must be between 5 and 22 characters, contain at least one letter, and not contain any of the following characters: <, >, \, ', "
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_statement_descriptor: Option<String>,
    /// A written description of the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A unique ID used to find or create a product. When provided during creation, we will look for an existing product with this external identifier — if found, it will be updated; otherwise, a new product will be created.
    #[serde(default)]
    pub external_identifier: String,
    /// The percentage of the revenue that goes to the global affiliate program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_affiliate_percentage: Option<f64>,
    /// The status of the global affiliate program for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_affiliate_status: Option<GlobalAffiliateStatuses>,
    /// The headline of the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// The ID of the product tax code to apply to this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_tax_code_id: Option<String>,
    /// The URL to redirect the customer to after a purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_purchase_url: Option<String>,
    /// The route of the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// The title of the product.
    #[serde(default)]
    pub title: String,
    /// This product will/will not be displayed publicly - default hidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
}

impl CreatePaymentsRequestBodyZeroPlanProduct {
    pub fn builder() -> CreatePaymentsRequestBodyZeroPlanProductBuilder {
        <CreatePaymentsRequestBodyZeroPlanProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestBodyZeroPlanProductBuilder {
    collect_shipping_address: Option<bool>,
    custom_statement_descriptor: Option<String>,
    description: Option<String>,
    external_identifier: Option<String>,
    global_affiliate_percentage: Option<f64>,
    global_affiliate_status: Option<GlobalAffiliateStatuses>,
    headline: Option<String>,
    product_tax_code_id: Option<String>,
    redirect_purchase_url: Option<String>,
    route: Option<String>,
    title: Option<String>,
    visibility: Option<Visibility>,
}

impl CreatePaymentsRequestBodyZeroPlanProductBuilder {
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

    pub fn global_affiliate_status(mut self, value: GlobalAffiliateStatuses) -> Self {
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

    pub fn visibility(mut self, value: Visibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequestBodyZeroPlanProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_identifier`](CreatePaymentsRequestBodyZeroPlanProductBuilder::external_identifier)
    /// - [`title`](CreatePaymentsRequestBodyZeroPlanProductBuilder::title)
    pub fn build(self) -> Result<CreatePaymentsRequestBodyZeroPlanProduct, BuildError> {
        Ok(CreatePaymentsRequestBodyZeroPlanProduct {
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
