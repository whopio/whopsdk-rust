pub use crate::prelude::*;

/// What needs attention, as a stable code.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainIssueCode {
    VerificationRequired,
    VerificationExpired,
    OwnershipConflict,
    OwnershipLost,
    DnsRequired,
    ProviderValidation,
    CertificatePending,
    ExpiringSoon,
    AccountUnavailable,
    CheckFailed,
    PurchaseExpired,
    DomainUnavailable,
    UnsupportedTld,
    RegistrationUnavailable,
    RegistrationFailed,
    RenewalFailed,
    PaymentActionRequired,
    DomainOnWhop,
    DomainTransferUnavailable,
    TransferLocked,
    TransferAuthCodeRejected,
    TransferRejected,
    TransferNotCancellable,
    DomainTransferFailed,
    ApplePayUnverified,
    PixelNotSeen,
    PixelNotInstalled,
    PixelWaiting,
    PixelUnreachable,
    PixelQuiet,
    EmailNeedsWhopDns,
    EmailUnreachable,
    EmailIdentityInUse,
    EmailVerificationFailed,
    SubdomainNeedsWhopDomain,
    RootDomainPending,
    RootDomainNeedsAction,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainIssueCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::VerificationRequired => serializer.serialize_str("verification_required"),
            Self::VerificationExpired => serializer.serialize_str("verification_expired"),
            Self::OwnershipConflict => serializer.serialize_str("ownership_conflict"),
            Self::OwnershipLost => serializer.serialize_str("ownership_lost"),
            Self::DnsRequired => serializer.serialize_str("dns_required"),
            Self::ProviderValidation => serializer.serialize_str("provider_validation"),
            Self::CertificatePending => serializer.serialize_str("certificate_pending"),
            Self::ExpiringSoon => serializer.serialize_str("expiring_soon"),
            Self::AccountUnavailable => serializer.serialize_str("account_unavailable"),
            Self::CheckFailed => serializer.serialize_str("check_failed"),
            Self::PurchaseExpired => serializer.serialize_str("purchase_expired"),
            Self::DomainUnavailable => serializer.serialize_str("domain_unavailable"),
            Self::UnsupportedTld => serializer.serialize_str("unsupported_tld"),
            Self::RegistrationUnavailable => serializer.serialize_str("registration_unavailable"),
            Self::RegistrationFailed => serializer.serialize_str("registration_failed"),
            Self::RenewalFailed => serializer.serialize_str("renewal_failed"),
            Self::PaymentActionRequired => serializer.serialize_str("payment_action_required"),
            Self::DomainOnWhop => serializer.serialize_str("domain_on_whop"),
            Self::DomainTransferUnavailable => {
                serializer.serialize_str("domain_transfer_unavailable")
            }
            Self::TransferLocked => serializer.serialize_str("transfer_locked"),
            Self::TransferAuthCodeRejected => {
                serializer.serialize_str("transfer_auth_code_rejected")
            }
            Self::TransferRejected => serializer.serialize_str("transfer_rejected"),
            Self::TransferNotCancellable => serializer.serialize_str("transfer_not_cancellable"),
            Self::DomainTransferFailed => serializer.serialize_str("domain_transfer_failed"),
            Self::ApplePayUnverified => serializer.serialize_str("apple_pay_unverified"),
            Self::PixelNotSeen => serializer.serialize_str("pixel_not_seen"),
            Self::PixelNotInstalled => serializer.serialize_str("pixel_not_installed"),
            Self::PixelWaiting => serializer.serialize_str("pixel_waiting"),
            Self::PixelUnreachable => serializer.serialize_str("pixel_unreachable"),
            Self::PixelQuiet => serializer.serialize_str("pixel_quiet"),
            Self::EmailNeedsWhopDns => serializer.serialize_str("email_needs_whop_dns"),
            Self::EmailUnreachable => serializer.serialize_str("email_unreachable"),
            Self::EmailIdentityInUse => serializer.serialize_str("email_identity_in_use"),
            Self::EmailVerificationFailed => serializer.serialize_str("email_verification_failed"),
            Self::SubdomainNeedsWhopDomain => {
                serializer.serialize_str("subdomain_needs_whop_domain")
            }
            Self::RootDomainPending => serializer.serialize_str("root_domain_pending"),
            Self::RootDomainNeedsAction => serializer.serialize_str("root_domain_needs_action"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainIssueCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "verification_required" => Ok(Self::VerificationRequired),
            "verification_expired" => Ok(Self::VerificationExpired),
            "ownership_conflict" => Ok(Self::OwnershipConflict),
            "ownership_lost" => Ok(Self::OwnershipLost),
            "dns_required" => Ok(Self::DnsRequired),
            "provider_validation" => Ok(Self::ProviderValidation),
            "certificate_pending" => Ok(Self::CertificatePending),
            "expiring_soon" => Ok(Self::ExpiringSoon),
            "account_unavailable" => Ok(Self::AccountUnavailable),
            "check_failed" => Ok(Self::CheckFailed),
            "purchase_expired" => Ok(Self::PurchaseExpired),
            "domain_unavailable" => Ok(Self::DomainUnavailable),
            "unsupported_tld" => Ok(Self::UnsupportedTld),
            "registration_unavailable" => Ok(Self::RegistrationUnavailable),
            "registration_failed" => Ok(Self::RegistrationFailed),
            "renewal_failed" => Ok(Self::RenewalFailed),
            "payment_action_required" => Ok(Self::PaymentActionRequired),
            "domain_on_whop" => Ok(Self::DomainOnWhop),
            "domain_transfer_unavailable" => Ok(Self::DomainTransferUnavailable),
            "transfer_locked" => Ok(Self::TransferLocked),
            "transfer_auth_code_rejected" => Ok(Self::TransferAuthCodeRejected),
            "transfer_rejected" => Ok(Self::TransferRejected),
            "transfer_not_cancellable" => Ok(Self::TransferNotCancellable),
            "domain_transfer_failed" => Ok(Self::DomainTransferFailed),
            "apple_pay_unverified" => Ok(Self::ApplePayUnverified),
            "pixel_not_seen" => Ok(Self::PixelNotSeen),
            "pixel_not_installed" => Ok(Self::PixelNotInstalled),
            "pixel_waiting" => Ok(Self::PixelWaiting),
            "pixel_unreachable" => Ok(Self::PixelUnreachable),
            "pixel_quiet" => Ok(Self::PixelQuiet),
            "email_needs_whop_dns" => Ok(Self::EmailNeedsWhopDns),
            "email_unreachable" => Ok(Self::EmailUnreachable),
            "email_identity_in_use" => Ok(Self::EmailIdentityInUse),
            "email_verification_failed" => Ok(Self::EmailVerificationFailed),
            "subdomain_needs_whop_domain" => Ok(Self::SubdomainNeedsWhopDomain),
            "root_domain_pending" => Ok(Self::RootDomainPending),
            "root_domain_needs_action" => Ok(Self::RootDomainNeedsAction),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainIssueCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VerificationRequired => write!(f, "verification_required"),
            Self::VerificationExpired => write!(f, "verification_expired"),
            Self::OwnershipConflict => write!(f, "ownership_conflict"),
            Self::OwnershipLost => write!(f, "ownership_lost"),
            Self::DnsRequired => write!(f, "dns_required"),
            Self::ProviderValidation => write!(f, "provider_validation"),
            Self::CertificatePending => write!(f, "certificate_pending"),
            Self::ExpiringSoon => write!(f, "expiring_soon"),
            Self::AccountUnavailable => write!(f, "account_unavailable"),
            Self::CheckFailed => write!(f, "check_failed"),
            Self::PurchaseExpired => write!(f, "purchase_expired"),
            Self::DomainUnavailable => write!(f, "domain_unavailable"),
            Self::UnsupportedTld => write!(f, "unsupported_tld"),
            Self::RegistrationUnavailable => write!(f, "registration_unavailable"),
            Self::RegistrationFailed => write!(f, "registration_failed"),
            Self::RenewalFailed => write!(f, "renewal_failed"),
            Self::PaymentActionRequired => write!(f, "payment_action_required"),
            Self::DomainOnWhop => write!(f, "domain_on_whop"),
            Self::DomainTransferUnavailable => write!(f, "domain_transfer_unavailable"),
            Self::TransferLocked => write!(f, "transfer_locked"),
            Self::TransferAuthCodeRejected => write!(f, "transfer_auth_code_rejected"),
            Self::TransferRejected => write!(f, "transfer_rejected"),
            Self::TransferNotCancellable => write!(f, "transfer_not_cancellable"),
            Self::DomainTransferFailed => write!(f, "domain_transfer_failed"),
            Self::ApplePayUnverified => write!(f, "apple_pay_unverified"),
            Self::PixelNotSeen => write!(f, "pixel_not_seen"),
            Self::PixelNotInstalled => write!(f, "pixel_not_installed"),
            Self::PixelWaiting => write!(f, "pixel_waiting"),
            Self::PixelUnreachable => write!(f, "pixel_unreachable"),
            Self::PixelQuiet => write!(f, "pixel_quiet"),
            Self::EmailNeedsWhopDns => write!(f, "email_needs_whop_dns"),
            Self::EmailUnreachable => write!(f, "email_unreachable"),
            Self::EmailIdentityInUse => write!(f, "email_identity_in_use"),
            Self::EmailVerificationFailed => write!(f, "email_verification_failed"),
            Self::SubdomainNeedsWhopDomain => write!(f, "subdomain_needs_whop_domain"),
            Self::RootDomainPending => write!(f, "root_domain_pending"),
            Self::RootDomainNeedsAction => write!(f, "root_domain_needs_action"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
