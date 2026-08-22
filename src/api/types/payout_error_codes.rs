pub use crate::prelude::*;

/// The different error codes a payout can be in.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PayoutErrorCodes {
    AccountClosed,
    AccountDoesNotExist,
    AccountInformationInvalid,
    AccountNumberInvalidRegion,
    AccountFrozen,
    AccountLookupFailed,
    AccountNotFound,
    AmountOutOfBounds,
    AttributesNotValidated,
    B2BPaymentsProhibited,
    BankStatementRequired,
    ComplianceReview,
    CurrencyNotSupported,
    DepositCanceled,
    DepositFailed,
    DepositRejected,
    DestinationUnavailable,
    ExceededAccountLimit,
    ExpiredQuote,
    GenericPayoutError,
    JunkFailureReason,
    TechnicalProblem,
    ProviderTemporarilyUnavailable,
    ProviderValidationFailed,
    IdentificationNumberInvalid,
    InvalidAccountNumber,
    InvalidBankCode,
    InvalidBeneficiary,
    InvalidMailingAddress,
    InvalidBranchNumber,
    InvalidBranchCode,
    InvalidPhoneNumber,
    InvalidRoutingNumber,
    InvalidSwiftCode,
    InvalidCompanyDetails,
    ManualCancelation,
    MiscError,
    MissingCityAndCountry,
    MissingPhoneNumber,
    MissingRemittanceInfo,
    PayeeNameInvalid,
    BeneficiaryNameMismatch,
    ReceivingAccountLocked,
    RejectedByCompliance,
    RtpNotSupported,
    NonTransactionAccount,
    SourceTokenInsufficientFunds,
    SsnInvalid,
    WalletScreenshotRequired,
    UnsupportedRegion,
    PayoutProviderTimeout,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PayoutErrorCodes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AccountClosed => serializer.serialize_str("account_closed"),
            Self::AccountDoesNotExist => serializer.serialize_str("account_does_not_exist"),
            Self::AccountInformationInvalid => {
                serializer.serialize_str("account_information_invalid")
            }
            Self::AccountNumberInvalidRegion => {
                serializer.serialize_str("account_number_invalid_region")
            }
            Self::AccountFrozen => serializer.serialize_str("account_frozen"),
            Self::AccountLookupFailed => serializer.serialize_str("account_lookup_failed"),
            Self::AccountNotFound => serializer.serialize_str("account_not_found"),
            Self::AmountOutOfBounds => serializer.serialize_str("amount_out_of_bounds"),
            Self::AttributesNotValidated => serializer.serialize_str("attributes_not_validated"),
            Self::B2BPaymentsProhibited => serializer.serialize_str("b2b_payments_prohibited"),
            Self::BankStatementRequired => serializer.serialize_str("bank_statement_required"),
            Self::ComplianceReview => serializer.serialize_str("compliance_review"),
            Self::CurrencyNotSupported => serializer.serialize_str("currency_not_supported"),
            Self::DepositCanceled => serializer.serialize_str("deposit_canceled"),
            Self::DepositFailed => serializer.serialize_str("deposit_failed"),
            Self::DepositRejected => serializer.serialize_str("deposit_rejected"),
            Self::DestinationUnavailable => serializer.serialize_str("destination_unavailable"),
            Self::ExceededAccountLimit => serializer.serialize_str("exceeded_account_limit"),
            Self::ExpiredQuote => serializer.serialize_str("expired_quote"),
            Self::GenericPayoutError => serializer.serialize_str("generic_payout_error"),
            Self::JunkFailureReason => serializer.serialize_str("junk_failure_reason"),
            Self::TechnicalProblem => serializer.serialize_str("technical_problem"),
            Self::ProviderTemporarilyUnavailable => {
                serializer.serialize_str("provider_temporarily_unavailable")
            }
            Self::ProviderValidationFailed => {
                serializer.serialize_str("provider_validation_failed")
            }
            Self::IdentificationNumberInvalid => {
                serializer.serialize_str("identification_number_invalid")
            }
            Self::InvalidAccountNumber => serializer.serialize_str("invalid_account_number"),
            Self::InvalidBankCode => serializer.serialize_str("invalid_bank_code"),
            Self::InvalidBeneficiary => serializer.serialize_str("invalid_beneficiary"),
            Self::InvalidMailingAddress => serializer.serialize_str("invalid_mailing_address"),
            Self::InvalidBranchNumber => serializer.serialize_str("invalid_branch_number"),
            Self::InvalidBranchCode => serializer.serialize_str("invalid_branch_code"),
            Self::InvalidPhoneNumber => serializer.serialize_str("invalid_phone_number"),
            Self::InvalidRoutingNumber => serializer.serialize_str("invalid_routing_number"),
            Self::InvalidSwiftCode => serializer.serialize_str("invalid_swift_code"),
            Self::InvalidCompanyDetails => serializer.serialize_str("invalid_company_details"),
            Self::ManualCancelation => serializer.serialize_str("manual_cancelation"),
            Self::MiscError => serializer.serialize_str("misc_error"),
            Self::MissingCityAndCountry => serializer.serialize_str("missing_city_and_country"),
            Self::MissingPhoneNumber => serializer.serialize_str("missing_phone_number"),
            Self::MissingRemittanceInfo => serializer.serialize_str("missing_remittance_info"),
            Self::PayeeNameInvalid => serializer.serialize_str("payee_name_invalid"),
            Self::BeneficiaryNameMismatch => serializer.serialize_str("beneficiary_name_mismatch"),
            Self::ReceivingAccountLocked => serializer.serialize_str("receiving_account_locked"),
            Self::RejectedByCompliance => serializer.serialize_str("rejected_by_compliance"),
            Self::RtpNotSupported => serializer.serialize_str("rtp_not_supported"),
            Self::NonTransactionAccount => serializer.serialize_str("non_transaction_account"),
            Self::SourceTokenInsufficientFunds => {
                serializer.serialize_str("source_token_insufficient_funds")
            }
            Self::SsnInvalid => serializer.serialize_str("ssn_invalid"),
            Self::WalletScreenshotRequired => {
                serializer.serialize_str("wallet_screenshot_required")
            }
            Self::UnsupportedRegion => serializer.serialize_str("unsupported_region"),
            Self::PayoutProviderTimeout => serializer.serialize_str("payout_provider_timeout"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PayoutErrorCodes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "account_closed" => Ok(Self::AccountClosed),
            "account_does_not_exist" => Ok(Self::AccountDoesNotExist),
            "account_information_invalid" => Ok(Self::AccountInformationInvalid),
            "account_number_invalid_region" => Ok(Self::AccountNumberInvalidRegion),
            "account_frozen" => Ok(Self::AccountFrozen),
            "account_lookup_failed" => Ok(Self::AccountLookupFailed),
            "account_not_found" => Ok(Self::AccountNotFound),
            "amount_out_of_bounds" => Ok(Self::AmountOutOfBounds),
            "attributes_not_validated" => Ok(Self::AttributesNotValidated),
            "b2b_payments_prohibited" => Ok(Self::B2BPaymentsProhibited),
            "bank_statement_required" => Ok(Self::BankStatementRequired),
            "compliance_review" => Ok(Self::ComplianceReview),
            "currency_not_supported" => Ok(Self::CurrencyNotSupported),
            "deposit_canceled" => Ok(Self::DepositCanceled),
            "deposit_failed" => Ok(Self::DepositFailed),
            "deposit_rejected" => Ok(Self::DepositRejected),
            "destination_unavailable" => Ok(Self::DestinationUnavailable),
            "exceeded_account_limit" => Ok(Self::ExceededAccountLimit),
            "expired_quote" => Ok(Self::ExpiredQuote),
            "generic_payout_error" => Ok(Self::GenericPayoutError),
            "junk_failure_reason" => Ok(Self::JunkFailureReason),
            "technical_problem" => Ok(Self::TechnicalProblem),
            "provider_temporarily_unavailable" => Ok(Self::ProviderTemporarilyUnavailable),
            "provider_validation_failed" => Ok(Self::ProviderValidationFailed),
            "identification_number_invalid" => Ok(Self::IdentificationNumberInvalid),
            "invalid_account_number" => Ok(Self::InvalidAccountNumber),
            "invalid_bank_code" => Ok(Self::InvalidBankCode),
            "invalid_beneficiary" => Ok(Self::InvalidBeneficiary),
            "invalid_mailing_address" => Ok(Self::InvalidMailingAddress),
            "invalid_branch_number" => Ok(Self::InvalidBranchNumber),
            "invalid_branch_code" => Ok(Self::InvalidBranchCode),
            "invalid_phone_number" => Ok(Self::InvalidPhoneNumber),
            "invalid_routing_number" => Ok(Self::InvalidRoutingNumber),
            "invalid_swift_code" => Ok(Self::InvalidSwiftCode),
            "invalid_company_details" => Ok(Self::InvalidCompanyDetails),
            "manual_cancelation" => Ok(Self::ManualCancelation),
            "misc_error" => Ok(Self::MiscError),
            "missing_city_and_country" => Ok(Self::MissingCityAndCountry),
            "missing_phone_number" => Ok(Self::MissingPhoneNumber),
            "missing_remittance_info" => Ok(Self::MissingRemittanceInfo),
            "payee_name_invalid" => Ok(Self::PayeeNameInvalid),
            "beneficiary_name_mismatch" => Ok(Self::BeneficiaryNameMismatch),
            "receiving_account_locked" => Ok(Self::ReceivingAccountLocked),
            "rejected_by_compliance" => Ok(Self::RejectedByCompliance),
            "rtp_not_supported" => Ok(Self::RtpNotSupported),
            "non_transaction_account" => Ok(Self::NonTransactionAccount),
            "source_token_insufficient_funds" => Ok(Self::SourceTokenInsufficientFunds),
            "ssn_invalid" => Ok(Self::SsnInvalid),
            "wallet_screenshot_required" => Ok(Self::WalletScreenshotRequired),
            "unsupported_region" => Ok(Self::UnsupportedRegion),
            "payout_provider_timeout" => Ok(Self::PayoutProviderTimeout),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PayoutErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountClosed => write!(f, "account_closed"),
            Self::AccountDoesNotExist => write!(f, "account_does_not_exist"),
            Self::AccountInformationInvalid => write!(f, "account_information_invalid"),
            Self::AccountNumberInvalidRegion => write!(f, "account_number_invalid_region"),
            Self::AccountFrozen => write!(f, "account_frozen"),
            Self::AccountLookupFailed => write!(f, "account_lookup_failed"),
            Self::AccountNotFound => write!(f, "account_not_found"),
            Self::AmountOutOfBounds => write!(f, "amount_out_of_bounds"),
            Self::AttributesNotValidated => write!(f, "attributes_not_validated"),
            Self::B2BPaymentsProhibited => write!(f, "b2b_payments_prohibited"),
            Self::BankStatementRequired => write!(f, "bank_statement_required"),
            Self::ComplianceReview => write!(f, "compliance_review"),
            Self::CurrencyNotSupported => write!(f, "currency_not_supported"),
            Self::DepositCanceled => write!(f, "deposit_canceled"),
            Self::DepositFailed => write!(f, "deposit_failed"),
            Self::DepositRejected => write!(f, "deposit_rejected"),
            Self::DestinationUnavailable => write!(f, "destination_unavailable"),
            Self::ExceededAccountLimit => write!(f, "exceeded_account_limit"),
            Self::ExpiredQuote => write!(f, "expired_quote"),
            Self::GenericPayoutError => write!(f, "generic_payout_error"),
            Self::JunkFailureReason => write!(f, "junk_failure_reason"),
            Self::TechnicalProblem => write!(f, "technical_problem"),
            Self::ProviderTemporarilyUnavailable => write!(f, "provider_temporarily_unavailable"),
            Self::ProviderValidationFailed => write!(f, "provider_validation_failed"),
            Self::IdentificationNumberInvalid => write!(f, "identification_number_invalid"),
            Self::InvalidAccountNumber => write!(f, "invalid_account_number"),
            Self::InvalidBankCode => write!(f, "invalid_bank_code"),
            Self::InvalidBeneficiary => write!(f, "invalid_beneficiary"),
            Self::InvalidMailingAddress => write!(f, "invalid_mailing_address"),
            Self::InvalidBranchNumber => write!(f, "invalid_branch_number"),
            Self::InvalidBranchCode => write!(f, "invalid_branch_code"),
            Self::InvalidPhoneNumber => write!(f, "invalid_phone_number"),
            Self::InvalidRoutingNumber => write!(f, "invalid_routing_number"),
            Self::InvalidSwiftCode => write!(f, "invalid_swift_code"),
            Self::InvalidCompanyDetails => write!(f, "invalid_company_details"),
            Self::ManualCancelation => write!(f, "manual_cancelation"),
            Self::MiscError => write!(f, "misc_error"),
            Self::MissingCityAndCountry => write!(f, "missing_city_and_country"),
            Self::MissingPhoneNumber => write!(f, "missing_phone_number"),
            Self::MissingRemittanceInfo => write!(f, "missing_remittance_info"),
            Self::PayeeNameInvalid => write!(f, "payee_name_invalid"),
            Self::BeneficiaryNameMismatch => write!(f, "beneficiary_name_mismatch"),
            Self::ReceivingAccountLocked => write!(f, "receiving_account_locked"),
            Self::RejectedByCompliance => write!(f, "rejected_by_compliance"),
            Self::RtpNotSupported => write!(f, "rtp_not_supported"),
            Self::NonTransactionAccount => write!(f, "non_transaction_account"),
            Self::SourceTokenInsufficientFunds => write!(f, "source_token_insufficient_funds"),
            Self::SsnInvalid => write!(f, "ssn_invalid"),
            Self::WalletScreenshotRequired => write!(f, "wallet_screenshot_required"),
            Self::UnsupportedRegion => write!(f, "unsupported_region"),
            Self::PayoutProviderTimeout => write!(f, "payout_provider_timeout"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
