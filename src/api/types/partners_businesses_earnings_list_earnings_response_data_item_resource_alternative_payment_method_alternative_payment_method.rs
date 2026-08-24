pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default)]
    pub name: String,
}

impl ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethod {
    pub fn builder(
    ) -> ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethodBuilder
    {
        <ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethodBuilder
{
    image_url: Option<String>,
    name: Option<String>,
}

impl ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethodBuilder {
    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethodBuilder::name)
    pub fn build(
        self,
    ) -> Result<
        ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethod,
        BuildError,
    > {
        Ok(
            ListEarningsResponseDataItemResourceAlternativePaymentMethodAlternativePaymentMethod {
                image_url: self.image_url,
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            },
        )
    }
}
