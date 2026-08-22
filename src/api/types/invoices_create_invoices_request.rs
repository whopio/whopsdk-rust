pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateInvoicesRequest {
    CreateInvoicesRequestProduct(CreateInvoicesRequestProduct),

    CreateInvoicesRequestProductId(CreateInvoicesRequestProductId),
}

impl CreateInvoicesRequest {
    pub fn is_create_invoices_request_product(&self) -> bool {
        matches!(self, Self::CreateInvoicesRequestProduct(_))
    }

    pub fn is_create_invoices_request_product_id(&self) -> bool {
        matches!(self, Self::CreateInvoicesRequestProductId(_))
    }

    pub fn as_create_invoices_request_product(&self) -> Option<&CreateInvoicesRequestProduct> {
        match self {
            Self::CreateInvoicesRequestProduct(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_invoices_request_product(self) -> Option<CreateInvoicesRequestProduct> {
        match self {
            Self::CreateInvoicesRequestProduct(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_invoices_request_product_id(&self) -> Option<&CreateInvoicesRequestProductId> {
        match self {
            Self::CreateInvoicesRequestProductId(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_invoices_request_product_id(self) -> Option<CreateInvoicesRequestProductId> {
        match self {
            Self::CreateInvoicesRequestProductId(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateInvoicesRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateInvoicesRequestProduct(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreateInvoicesRequestProductId(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
