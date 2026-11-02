pub use crate::prelude::*;

/// Whop Elements the app's production web build mounts, as `<namespace>.<element>` keys (sub-controller children take a third segment, e.g. `payments.cardFields.cardNumber`). A bare namespace means the build reaches that namespace but the individual elements could not be resolved. Empty when the build mounts none, when it has not been scanned yet, or when the app has no production web build.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppElementsUsedItem {
    Ads,
    AdsBillingSetup,
    AdsCampaignCreator,
    AdsReporting,
    AdsReportingChart,
    AdsReportingTable,
    Checkout,
    CheckoutCheckout,
    CheckoutExpressCheckout,
    Payments,
    PaymentsAddress,
    PaymentsBranding,
    PaymentsCard,
    PaymentsCardFields,
    PaymentsCardFieldsCardCvc,
    PaymentsCardFieldsCardExpiry,
    PaymentsCardFieldsCardNumber,
    PaymentsEmail,
    PaymentsPayment,
    PaymentsTaxId,
    Tracking,
    TrackingEvents,
    TrackingPeople,
    Wallet,
    WalletActivity,
    WalletBalances,
    WalletBalancesBalance,
    WalletBalancesList,
    WalletCards,
    WalletDeposit,
    WalletSend,
    WalletWithdraw,
    Websites,
    WebsitesPixelSetup,
    WebsitesWebsites,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AppElementsUsedItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ads => serializer.serialize_str("ads"),
            Self::AdsBillingSetup => serializer.serialize_str("ads.billing-setup"),
            Self::AdsCampaignCreator => serializer.serialize_str("ads.campaign-creator"),
            Self::AdsReporting => serializer.serialize_str("ads.reporting"),
            Self::AdsReportingChart => serializer.serialize_str("ads.reporting.chart"),
            Self::AdsReportingTable => serializer.serialize_str("ads.reporting.table"),
            Self::Checkout => serializer.serialize_str("checkout"),
            Self::CheckoutCheckout => serializer.serialize_str("checkout.checkout"),
            Self::CheckoutExpressCheckout => serializer.serialize_str("checkout.expressCheckout"),
            Self::Payments => serializer.serialize_str("payments"),
            Self::PaymentsAddress => serializer.serialize_str("payments.address"),
            Self::PaymentsBranding => serializer.serialize_str("payments.branding"),
            Self::PaymentsCard => serializer.serialize_str("payments.card"),
            Self::PaymentsCardFields => serializer.serialize_str("payments.cardFields"),
            Self::PaymentsCardFieldsCardCvc => {
                serializer.serialize_str("payments.cardFields.cardCvc")
            }
            Self::PaymentsCardFieldsCardExpiry => {
                serializer.serialize_str("payments.cardFields.cardExpiry")
            }
            Self::PaymentsCardFieldsCardNumber => {
                serializer.serialize_str("payments.cardFields.cardNumber")
            }
            Self::PaymentsEmail => serializer.serialize_str("payments.email"),
            Self::PaymentsPayment => serializer.serialize_str("payments.payment"),
            Self::PaymentsTaxId => serializer.serialize_str("payments.taxId"),
            Self::Tracking => serializer.serialize_str("tracking"),
            Self::TrackingEvents => serializer.serialize_str("tracking.events"),
            Self::TrackingPeople => serializer.serialize_str("tracking.people"),
            Self::Wallet => serializer.serialize_str("wallet"),
            Self::WalletActivity => serializer.serialize_str("wallet.activity"),
            Self::WalletBalances => serializer.serialize_str("wallet.balances"),
            Self::WalletBalancesBalance => serializer.serialize_str("wallet.balances.balance"),
            Self::WalletBalancesList => serializer.serialize_str("wallet.balances.list"),
            Self::WalletCards => serializer.serialize_str("wallet.cards"),
            Self::WalletDeposit => serializer.serialize_str("wallet.deposit"),
            Self::WalletSend => serializer.serialize_str("wallet.send"),
            Self::WalletWithdraw => serializer.serialize_str("wallet.withdraw"),
            Self::Websites => serializer.serialize_str("websites"),
            Self::WebsitesPixelSetup => serializer.serialize_str("websites.pixel-setup"),
            Self::WebsitesWebsites => serializer.serialize_str("websites.websites"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AppElementsUsedItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ads" => Ok(Self::Ads),
            "ads.billing-setup" => Ok(Self::AdsBillingSetup),
            "ads.campaign-creator" => Ok(Self::AdsCampaignCreator),
            "ads.reporting" => Ok(Self::AdsReporting),
            "ads.reporting.chart" => Ok(Self::AdsReportingChart),
            "ads.reporting.table" => Ok(Self::AdsReportingTable),
            "checkout" => Ok(Self::Checkout),
            "checkout.checkout" => Ok(Self::CheckoutCheckout),
            "checkout.expressCheckout" => Ok(Self::CheckoutExpressCheckout),
            "payments" => Ok(Self::Payments),
            "payments.address" => Ok(Self::PaymentsAddress),
            "payments.branding" => Ok(Self::PaymentsBranding),
            "payments.card" => Ok(Self::PaymentsCard),
            "payments.cardFields" => Ok(Self::PaymentsCardFields),
            "payments.cardFields.cardCvc" => Ok(Self::PaymentsCardFieldsCardCvc),
            "payments.cardFields.cardExpiry" => Ok(Self::PaymentsCardFieldsCardExpiry),
            "payments.cardFields.cardNumber" => Ok(Self::PaymentsCardFieldsCardNumber),
            "payments.email" => Ok(Self::PaymentsEmail),
            "payments.payment" => Ok(Self::PaymentsPayment),
            "payments.taxId" => Ok(Self::PaymentsTaxId),
            "tracking" => Ok(Self::Tracking),
            "tracking.events" => Ok(Self::TrackingEvents),
            "tracking.people" => Ok(Self::TrackingPeople),
            "wallet" => Ok(Self::Wallet),
            "wallet.activity" => Ok(Self::WalletActivity),
            "wallet.balances" => Ok(Self::WalletBalances),
            "wallet.balances.balance" => Ok(Self::WalletBalancesBalance),
            "wallet.balances.list" => Ok(Self::WalletBalancesList),
            "wallet.cards" => Ok(Self::WalletCards),
            "wallet.deposit" => Ok(Self::WalletDeposit),
            "wallet.send" => Ok(Self::WalletSend),
            "wallet.withdraw" => Ok(Self::WalletWithdraw),
            "websites" => Ok(Self::Websites),
            "websites.pixel-setup" => Ok(Self::WebsitesPixelSetup),
            "websites.websites" => Ok(Self::WebsitesWebsites),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AppElementsUsedItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ads => write!(f, "ads"),
            Self::AdsBillingSetup => write!(f, "ads.billing-setup"),
            Self::AdsCampaignCreator => write!(f, "ads.campaign-creator"),
            Self::AdsReporting => write!(f, "ads.reporting"),
            Self::AdsReportingChart => write!(f, "ads.reporting.chart"),
            Self::AdsReportingTable => write!(f, "ads.reporting.table"),
            Self::Checkout => write!(f, "checkout"),
            Self::CheckoutCheckout => write!(f, "checkout.checkout"),
            Self::CheckoutExpressCheckout => write!(f, "checkout.expressCheckout"),
            Self::Payments => write!(f, "payments"),
            Self::PaymentsAddress => write!(f, "payments.address"),
            Self::PaymentsBranding => write!(f, "payments.branding"),
            Self::PaymentsCard => write!(f, "payments.card"),
            Self::PaymentsCardFields => write!(f, "payments.cardFields"),
            Self::PaymentsCardFieldsCardCvc => write!(f, "payments.cardFields.cardCvc"),
            Self::PaymentsCardFieldsCardExpiry => write!(f, "payments.cardFields.cardExpiry"),
            Self::PaymentsCardFieldsCardNumber => write!(f, "payments.cardFields.cardNumber"),
            Self::PaymentsEmail => write!(f, "payments.email"),
            Self::PaymentsPayment => write!(f, "payments.payment"),
            Self::PaymentsTaxId => write!(f, "payments.taxId"),
            Self::Tracking => write!(f, "tracking"),
            Self::TrackingEvents => write!(f, "tracking.events"),
            Self::TrackingPeople => write!(f, "tracking.people"),
            Self::Wallet => write!(f, "wallet"),
            Self::WalletActivity => write!(f, "wallet.activity"),
            Self::WalletBalances => write!(f, "wallet.balances"),
            Self::WalletBalancesBalance => write!(f, "wallet.balances.balance"),
            Self::WalletBalancesList => write!(f, "wallet.balances.list"),
            Self::WalletCards => write!(f, "wallet.cards"),
            Self::WalletDeposit => write!(f, "wallet.deposit"),
            Self::WalletSend => write!(f, "wallet.send"),
            Self::WalletWithdraw => write!(f, "wallet.withdraw"),
            Self::Websites => write!(f, "websites"),
            Self::WebsitesPixelSetup => write!(f, "websites.pixel-setup"),
            Self::WebsitesWebsites => write!(f, "websites.websites"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
