pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListSupportedMethodsResponseDataItem {
    /// How funds are delivered, for example `bank_deposit`.
    pub delivery_type: ListSupportedMethodsResponseDataItemDeliveryType,
    /// Supported payout method icon URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Supported payout method ID.
    #[serde(default)]
    pub id: String,
    /// Supported payout method display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub object: ListSupportedMethodsResponseDataItemObject,
    /// Fee and delivery estimates for the requested amount, one per destination currency. Null unless an amount was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotes: Option<Vec<ListSupportedMethodsResponseDataItemQuotesItem>>,
    /// Fields to collect before saving this supported payout method. Present only when supported_payout_method_id narrows the request to one method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_fields: Option<Vec<ListSupportedMethodsResponseDataItemRequiredFieldsItem>>,
    #[serde(default)]
    pub supports_instant_delivery: bool,
    #[serde(default)]
    pub supports_standard_delivery: bool,
}

impl ListSupportedMethodsResponseDataItem {
    pub fn builder() -> ListSupportedMethodsResponseDataItemBuilder {
        <ListSupportedMethodsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSupportedMethodsResponseDataItemBuilder {
    delivery_type: Option<ListSupportedMethodsResponseDataItemDeliveryType>,
    icon_url: Option<String>,
    id: Option<String>,
    name: Option<String>,
    object: Option<ListSupportedMethodsResponseDataItemObject>,
    quotes: Option<Vec<ListSupportedMethodsResponseDataItemQuotesItem>>,
    required_fields: Option<Vec<ListSupportedMethodsResponseDataItemRequiredFieldsItem>>,
    supports_instant_delivery: Option<bool>,
    supports_standard_delivery: Option<bool>,
}

impl ListSupportedMethodsResponseDataItemBuilder {
    pub fn delivery_type(
        mut self,
        value: ListSupportedMethodsResponseDataItemDeliveryType,
    ) -> Self {
        self.delivery_type = Some(value);
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn object(mut self, value: ListSupportedMethodsResponseDataItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn quotes(mut self, value: Vec<ListSupportedMethodsResponseDataItemQuotesItem>) -> Self {
        self.quotes = Some(value);
        self
    }

    pub fn required_fields(
        mut self,
        value: Vec<ListSupportedMethodsResponseDataItemRequiredFieldsItem>,
    ) -> Self {
        self.required_fields = Some(value);
        self
    }

    pub fn supports_instant_delivery(mut self, value: bool) -> Self {
        self.supports_instant_delivery = Some(value);
        self
    }

    pub fn supports_standard_delivery(mut self, value: bool) -> Self {
        self.supports_standard_delivery = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSupportedMethodsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delivery_type`](ListSupportedMethodsResponseDataItemBuilder::delivery_type)
    /// - [`id`](ListSupportedMethodsResponseDataItemBuilder::id)
    /// - [`object`](ListSupportedMethodsResponseDataItemBuilder::object)
    /// - [`supports_instant_delivery`](ListSupportedMethodsResponseDataItemBuilder::supports_instant_delivery)
    /// - [`supports_standard_delivery`](ListSupportedMethodsResponseDataItemBuilder::supports_standard_delivery)
    pub fn build(self) -> Result<ListSupportedMethodsResponseDataItem, BuildError> {
        Ok(ListSupportedMethodsResponseDataItem {
            delivery_type: self
                .delivery_type
                .ok_or_else(|| BuildError::missing_field("delivery_type"))?,
            icon_url: self.icon_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            quotes: self.quotes,
            required_fields: self.required_fields,
            supports_instant_delivery: self
                .supports_instant_delivery
                .ok_or_else(|| BuildError::missing_field("supports_instant_delivery"))?,
            supports_standard_delivery: self
                .supports_standard_delivery
                .ok_or_else(|| BuildError::missing_field("supports_standard_delivery"))?,
        })
    }
}
