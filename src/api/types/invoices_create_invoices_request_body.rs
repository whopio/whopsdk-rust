pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateInvoicesRequestBody {
    CreateInvoicesRequestBodyProduct(CreateInvoicesRequestBodyProduct),

    CreateInvoicesRequestBodyProductId(CreateInvoicesRequestBodyProductId),
}

impl CreateInvoicesRequestBody {
    pub fn is_create_invoices_request_body_product(&self) -> bool {
        matches!(self, Self::CreateInvoicesRequestBodyProduct(_))
    }

    pub fn is_create_invoices_request_body_product_id(&self) -> bool {
        matches!(self, Self::CreateInvoicesRequestBodyProductId(_))
    }

    pub fn as_create_invoices_request_body_product(
        &self,
    ) -> Option<&CreateInvoicesRequestBodyProduct> {
        match self {
            Self::CreateInvoicesRequestBodyProduct(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_invoices_request_body_product(
        self,
    ) -> Option<CreateInvoicesRequestBodyProduct> {
        match self {
            Self::CreateInvoicesRequestBodyProduct(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_invoices_request_body_product_id(
        &self,
    ) -> Option<&CreateInvoicesRequestBodyProductId> {
        match self {
            Self::CreateInvoicesRequestBodyProductId(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_invoices_request_body_product_id(
        self,
    ) -> Option<CreateInvoicesRequestBodyProductId> {
        match self {
            Self::CreateInvoicesRequestBodyProductId(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateInvoicesRequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateInvoicesRequestBodyProduct(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreateInvoicesRequestBodyProductId(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
