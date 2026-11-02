pub use crate::prelude::*;

/// The customer-facing payment method family or standalone service.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveFinancialReportsResponsePaymentFeeBreakdownItemCategory {
    AppStorePayments,
    BankPayments,
    BuyNowPayLater,
    CardPayments,
    CryptoPayments,
    FraudPrevention,
    LocalPayments,
    OtherPaymentMethods,
    SubscriptionBilling,
    WalletPayments,
    WhopBalance,
    WhopOrchestration,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveFinancialReportsResponsePaymentFeeBreakdownItemCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AppStorePayments => serializer.serialize_str("app_store_payments"),
            Self::BankPayments => serializer.serialize_str("bank_payments"),
            Self::BuyNowPayLater => serializer.serialize_str("buy_now_pay_later"),
            Self::CardPayments => serializer.serialize_str("card_payments"),
            Self::CryptoPayments => serializer.serialize_str("crypto_payments"),
            Self::FraudPrevention => serializer.serialize_str("fraud_prevention"),
            Self::LocalPayments => serializer.serialize_str("local_payments"),
            Self::OtherPaymentMethods => serializer.serialize_str("other_payment_methods"),
            Self::SubscriptionBilling => serializer.serialize_str("subscription_billing"),
            Self::WalletPayments => serializer.serialize_str("wallet_payments"),
            Self::WhopBalance => serializer.serialize_str("whop_balance"),
            Self::WhopOrchestration => serializer.serialize_str("whop_orchestration"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveFinancialReportsResponsePaymentFeeBreakdownItemCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "app_store_payments" => Ok(Self::AppStorePayments),
            "bank_payments" => Ok(Self::BankPayments),
            "buy_now_pay_later" => Ok(Self::BuyNowPayLater),
            "card_payments" => Ok(Self::CardPayments),
            "crypto_payments" => Ok(Self::CryptoPayments),
            "fraud_prevention" => Ok(Self::FraudPrevention),
            "local_payments" => Ok(Self::LocalPayments),
            "other_payment_methods" => Ok(Self::OtherPaymentMethods),
            "subscription_billing" => Ok(Self::SubscriptionBilling),
            "wallet_payments" => Ok(Self::WalletPayments),
            "whop_balance" => Ok(Self::WhopBalance),
            "whop_orchestration" => Ok(Self::WhopOrchestration),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveFinancialReportsResponsePaymentFeeBreakdownItemCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AppStorePayments => write!(f, "app_store_payments"),
            Self::BankPayments => write!(f, "bank_payments"),
            Self::BuyNowPayLater => write!(f, "buy_now_pay_later"),
            Self::CardPayments => write!(f, "card_payments"),
            Self::CryptoPayments => write!(f, "crypto_payments"),
            Self::FraudPrevention => write!(f, "fraud_prevention"),
            Self::LocalPayments => write!(f, "local_payments"),
            Self::OtherPaymentMethods => write!(f, "other_payment_methods"),
            Self::SubscriptionBilling => write!(f, "subscription_billing"),
            Self::WalletPayments => write!(f, "wallet_payments"),
            Self::WhopBalance => write!(f, "whop_balance"),
            Self::WhopOrchestration => write!(f, "whop_orchestration"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
