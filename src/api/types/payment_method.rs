pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "typename")]
#[non_exhaustive]
pub enum PaymentMethod {
    #[non_exhaustive]
    BasePaymentMethod {
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        icons: PaymentMethodBasePaymentMethodIcons,
        #[serde(default)]
        id: String,
        payment_method_type: PaymentMethodTypes,
    },

    #[non_exhaustive]
    CardPaymentMethod {
        #[serde(default)]
        card: PaymentMethodCardPaymentMethodCard,
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        has_payer_document: bool,
        #[serde(default)]
        icons: PaymentMethodCardPaymentMethodIcons,
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
        icons: PaymentMethodUsBankAccountPaymentMethodIcons,
        #[serde(default)]
        id: String,
        payment_method_type: PaymentMethodTypes,
        #[serde(default)]
        us_bank_account: PaymentMethodUsBankAccountPaymentMethodUsBankAccount,
    },

    #[non_exhaustive]
    CashappPaymentMethod {
        #[serde(default)]
        cashapp: PaymentMethodCashappPaymentMethodCashapp,
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        icons: PaymentMethodCashappPaymentMethodIcons,
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
        icons: PaymentMethodIdealPaymentMethodIcons,
        #[serde(default)]
        id: String,
        #[serde(default)]
        ideal: PaymentMethodIdealPaymentMethodIdeal,
        payment_method_type: PaymentMethodTypes,
    },

    #[non_exhaustive]
    SepaDebitPaymentMethod {
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        icons: PaymentMethodSepaDebitPaymentMethodIcons,
        #[serde(default)]
        id: String,
        payment_method_type: PaymentMethodTypes,
        #[serde(default)]
        sepa_debit: PaymentMethodSepaDebitPaymentMethodSepaDebit,
    },

    #[non_exhaustive]
    PlatformBalancePaymentMethod {
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(default)]
        icons: PaymentMethodPlatformBalancePaymentMethodIcons,
        #[serde(default)]
        id: String,
        payment_method_type: PaymentMethodTypes,
        #[serde(default)]
        platform_balance: PaymentMethodPlatformBalancePaymentMethodPlatformBalance,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PaymentMethod {
    pub fn base_payment_method(
        created_at: DateTime<FixedOffset>,
        icons: PaymentMethodBasePaymentMethodIcons,
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
        card: PaymentMethodCardPaymentMethodCard,
        created_at: DateTime<FixedOffset>,
        has_payer_document: bool,
        icons: PaymentMethodCardPaymentMethodIcons,
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
        icons: PaymentMethodUsBankAccountPaymentMethodIcons,
        id: String,
        payment_method_type: PaymentMethodTypes,
        us_bank_account: PaymentMethodUsBankAccountPaymentMethodUsBankAccount,
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
        cashapp: PaymentMethodCashappPaymentMethodCashapp,
        created_at: DateTime<FixedOffset>,
        icons: PaymentMethodCashappPaymentMethodIcons,
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
        icons: PaymentMethodIdealPaymentMethodIcons,
        id: String,
        ideal: PaymentMethodIdealPaymentMethodIdeal,
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
        icons: PaymentMethodSepaDebitPaymentMethodIcons,
        id: String,
        payment_method_type: PaymentMethodTypes,
        sepa_debit: PaymentMethodSepaDebitPaymentMethodSepaDebit,
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
        icons: PaymentMethodPlatformBalancePaymentMethodIcons,
        id: String,
        payment_method_type: PaymentMethodTypes,
        platform_balance: PaymentMethodPlatformBalancePaymentMethodPlatformBalance,
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
