pub use crate::prelude::*;

/// The properties of the product to create for this invoice. Provide this to create a new product inline.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateInvoicesRequestBodyProductProduct {
    /// The ID of the product tax code to apply to this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_tax_code_id: Option<String>,
    /// The title of the product.
    #[serde(default)]
    pub title: String,
}

impl CreateInvoicesRequestBodyProductProduct {
    pub fn builder() -> CreateInvoicesRequestBodyProductProductBuilder {
        <CreateInvoicesRequestBodyProductProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvoicesRequestBodyProductProductBuilder {
    product_tax_code_id: Option<String>,
    title: Option<String>,
}

impl CreateInvoicesRequestBodyProductProductBuilder {
    pub fn product_tax_code_id(mut self, value: impl Into<String>) -> Self {
        self.product_tax_code_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateInvoicesRequestBodyProductProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](CreateInvoicesRequestBodyProductProductBuilder::title)
    pub fn build(self) -> Result<CreateInvoicesRequestBodyProductProduct, BuildError> {
        Ok(CreateInvoicesRequestBodyProductProduct {
            product_tax_code_id: self.product_tax_code_id,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
