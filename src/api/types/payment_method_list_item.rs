pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "typename")]
#[non_exhaustive]
pub enum PaymentMethodListItem {
    #[non_exhaustive]
    BasePaymentMethod {
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        icons: PaymentMethodListItemBasePaymentMethodIcons,
        #[serde(default)]
        id: String,
        payment_method_type: PaymentMethodTypes,
    },

    #[non_exhaustive]
    CardPaymentMethod {
        #[serde(default)]
        card: PaymentMethodListItemCardPaymentMethodCard,
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        has_payer_document: bool,
        #[serde(default)]
        icons: PaymentMethodListItemCardPaymentMethodIcons,
        #[serde(default)]
        id: String,
        payment_method_type: PaymentMethodTypes,
    },

    #[non_exhaustive]
    UsBankAccountPaymentMethod {
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        icons: PaymentMethodListItemUsBankAccountPaymentMethodIcons,
        #[serde(default)]
        id: String,
        payment_method_type: PaymentMethodTypes,
        #[serde(default)]
        us_bank_account: PaymentMethodListItemUsBankAccountPaymentMethodUsBankAccount,
    },

    #[non_exhaustive]
    CashappPaymentMethod {
        #[serde(default)]
        cashapp: PaymentMethodListItemCashappPaymentMethodCashapp,
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        icons: PaymentMethodListItemCashappPaymentMethodIcons,
        #[serde(default)]
        id: String,
        payment_method_type: PaymentMethodTypes,
    },

    #[non_exhaustive]
    IdealPaymentMethod {
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        icons: PaymentMethodListItemIdealPaymentMethodIcons,
        #[serde(default)]
        id: String,
        #[serde(default)]
        ideal: PaymentMethodListItemIdealPaymentMethodIdeal,
        payment_method_type: PaymentMethodTypes,
    },

    #[non_exhaustive]
    SepaDebitPaymentMethod {
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        icons: PaymentMethodListItemSepaDebitPaymentMethodIcons,
        #[serde(default)]
        id: String,
        payment_method_type: PaymentMethodTypes,
        #[serde(default)]
        sepa_debit: PaymentMethodListItemSepaDebitPaymentMethodSepaDebit,
    },

    #[non_exhaustive]
    PlatformBalancePaymentMethod {
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        icons: PaymentMethodListItemPlatformBalancePaymentMethodIcons,
        #[serde(default)]
        id: String,
        payment_method_type: PaymentMethodTypes,
        #[serde(default)]
        platform_balance: PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalance,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PaymentMethodListItem {
    pub fn base_payment_method(
        created_at: DateTime<FixedOffset>,
        icons: PaymentMethodListItemBasePaymentMethodIcons,
        id: String,
        payment_method_type: PaymentMethodTypes,
    ) -> Self {
        Self::BasePaymentMethod {
            created_at,
            icons,
            id,
            payment_method_type,
        }
    }

    pub fn card_payment_method(
        card: PaymentMethodListItemCardPaymentMethodCard,
        created_at: DateTime<FixedOffset>,
        has_payer_document: bool,
        icons: PaymentMethodListItemCardPaymentMethodIcons,
        id: String,
        payment_method_type: PaymentMethodTypes,
    ) -> Self {
        Self::CardPaymentMethod {
            card,
            created_at,
            has_payer_document,
            icons,
            id,
            payment_method_type,
        }
    }

    pub fn us_bank_account_payment_method(
        created_at: DateTime<FixedOffset>,
        icons: PaymentMethodListItemUsBankAccountPaymentMethodIcons,
        id: String,
        payment_method_type: PaymentMethodTypes,
        us_bank_account: PaymentMethodListItemUsBankAccountPaymentMethodUsBankAccount,
    ) -> Self {
        Self::UsBankAccountPaymentMethod {
            created_at,
            icons,
            id,
            payment_method_type,
            us_bank_account,
        }
    }

    pub fn cashapp_payment_method(
        cashapp: PaymentMethodListItemCashappPaymentMethodCashapp,
        created_at: DateTime<FixedOffset>,
        icons: PaymentMethodListItemCashappPaymentMethodIcons,
        id: String,
        payment_method_type: PaymentMethodTypes,
    ) -> Self {
        Self::CashappPaymentMethod {
            cashapp,
            created_at,
            icons,
            id,
            payment_method_type,
        }
    }

    pub fn ideal_payment_method(
        created_at: DateTime<FixedOffset>,
        icons: PaymentMethodListItemIdealPaymentMethodIcons,
        id: String,
        ideal: PaymentMethodListItemIdealPaymentMethodIdeal,
        payment_method_type: PaymentMethodTypes,
    ) -> Self {
        Self::IdealPaymentMethod {
            created_at,
            icons,
            id,
            ideal,
            payment_method_type,
        }
    }

    pub fn sepa_debit_payment_method(
        created_at: DateTime<FixedOffset>,
        icons: PaymentMethodListItemSepaDebitPaymentMethodIcons,
        id: String,
        payment_method_type: PaymentMethodTypes,
        sepa_debit: PaymentMethodListItemSepaDebitPaymentMethodSepaDebit,
    ) -> Self {
        Self::SepaDebitPaymentMethod {
            created_at,
            icons,
            id,
            payment_method_type,
            sepa_debit,
        }
    }

    pub fn platform_balance_payment_method(
        created_at: DateTime<FixedOffset>,
        icons: PaymentMethodListItemPlatformBalancePaymentMethodIcons,
        id: String,
        payment_method_type: PaymentMethodTypes,
        platform_balance: PaymentMethodListItemPlatformBalancePaymentMethodPlatformBalance,
    ) -> Self {
        Self::PlatformBalancePaymentMethod {
            created_at,
            icons,
            id,
            payment_method_type,
            platform_balance,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
