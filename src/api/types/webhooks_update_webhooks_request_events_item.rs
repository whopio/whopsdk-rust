pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateWebhooksRequestEventsItem {
    AccountUpdated,
    InvoiceCreated,
    InvoiceMarkedUncollectible,
    InvoicePaid,
    InvoicePastDue,
    InvoiceVoided,
    MembershipActivated,
    MembershipDeactivated,
    MembershipTrialEndingSoon,
    EntryCreated,
    EntryApproved,
    EntryDenied,
    EntryDeleted,
    ExportCompleted,
    ExportFailed,
    SetupIntentRequiresAction,
    SetupIntentSucceeded,
    SetupIntentCanceled,
    LedgerAccountFundsAvailable,
    SwapCompleted,
    DepositSucceeded,
    TransferCreated,
    TransferCompleted,
    TransferFailed,
    WithdrawalCreated,
    WithdrawalUpdated,
    WithdrawalReversed,
    CardTransactionCreated,
    CardTransactionUpdated,
    CardTransactionCompleted,
    CardTransactionDeclined,
    CardTransactionReversed,
    CardCreated,
    CardUpdated,
    CardFrozen,
    CardCanceled,
    CardApplicationCreated,
    CardApplicationUpdated,
    CardApplicationApproved,
    CardApplicationDenied,
    CourseLessonInteractionCompleted,
    PayoutMethodCreated,
    VerificationSucceeded,
    IdentityProfileApproved,
    IdentityProfileRejected,
    IdentityProfileNeedsAction,
    IdentityProfileUpdated,
    PayoutAccountStatusUpdated,
    ResolutionCenterCaseCreated,
    ResolutionCenterCaseUpdated,
    ResolutionCenterCaseDecided,
    ProductCreated,
    ProductUpdated,
    ProductDeleted,
    ProductPublished,
    ProductUnpublished,
    PlanCreated,
    PlanUpdated,
    PlanDeleted,
    ShipmentCreated,
    ShipmentUpdated,
    MemberCreated,
    AdCampaignPaymentFailed,
    ChatMessageCreated,
    ChatReactionCreated,
    PaymentCreated,
    PaymentSucceeded,
    PaymentFailed,
    PaymentPending,
    DisputeCreated,
    DisputeUpdated,
    RefundCreated,
    RefundUpdated,
    DisputeAlertCreated,
    MembershipCancelAtPeriodEndChanged,
    MembershipWentValidUnderscore,
    MembershipWentInvalidUnderscore,
    MembershipMetadataUpdatedUnderscore,
    ResolutionCreatedUnderscore,
    ResolutionUpdatedUnderscore,
    ResolutionDecidedUnderscore,
    PaymentAffiliateRewardCreatedUnderscore,
    MembershipExperienceClaimedUnderscore,
    AppMembershipWentValidUnderscore,
    AppMembershipWentInvalidUnderscore,
    AppPaymentCreatedUnderscore,
    AppPaymentSucceededUnderscore,
    AppPaymentFailedUnderscore,
    AppPaymentPendingUnderscore,
    AppMembershipCancelAtPeriodEndChangedUnderscore,
    PaymentCreatedUnderscore,
    PaymentSucceededUnderscore,
    PaymentFailedUnderscore,
    PaymentPendingUnderscore,
    DisputeCreatedUnderscore,
    DisputeUpdatedUnderscore,
    RefundCreatedUnderscore,
    RefundUpdatedUnderscore,
    DisputeAlertCreatedUnderscore,
    MembershipCancelAtPeriodEndChangedUnderscore,
    MembershipWentValid,
    MembershipWentInvalid,
    MembershipMetadataUpdated,
    ResolutionCreated,
    ResolutionUpdated,
    ResolutionDecided,
    PaymentAffiliateRewardCreated,
    MembershipExperienceClaimed,
    AppMembershipWentValid,
    AppMembershipWentInvalid,
    AppPaymentCreated,
    AppPaymentSucceeded,
    AppPaymentFailed,
    AppPaymentPending,
    AppMembershipCancelAtPeriodEndChanged,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateWebhooksRequestEventsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AccountUpdated => serializer.serialize_str("account.updated"),
            Self::InvoiceCreated => serializer.serialize_str("invoice.created"),
            Self::InvoiceMarkedUncollectible => {
                serializer.serialize_str("invoice.marked_uncollectible")
            }
            Self::InvoicePaid => serializer.serialize_str("invoice.paid"),
            Self::InvoicePastDue => serializer.serialize_str("invoice.past_due"),
            Self::InvoiceVoided => serializer.serialize_str("invoice.voided"),
            Self::MembershipActivated => serializer.serialize_str("membership.activated"),
            Self::MembershipDeactivated => serializer.serialize_str("membership.deactivated"),
            Self::MembershipTrialEndingSoon => {
                serializer.serialize_str("membership.trial_ending_soon")
            }
            Self::EntryCreated => serializer.serialize_str("entry.created"),
            Self::EntryApproved => serializer.serialize_str("entry.approved"),
            Self::EntryDenied => serializer.serialize_str("entry.denied"),
            Self::EntryDeleted => serializer.serialize_str("entry.deleted"),
            Self::ExportCompleted => serializer.serialize_str("export.completed"),
            Self::ExportFailed => serializer.serialize_str("export.failed"),
            Self::SetupIntentRequiresAction => {
                serializer.serialize_str("setup_intent.requires_action")
            }
            Self::SetupIntentSucceeded => serializer.serialize_str("setup_intent.succeeded"),
            Self::SetupIntentCanceled => serializer.serialize_str("setup_intent.canceled"),
            Self::LedgerAccountFundsAvailable => {
                serializer.serialize_str("ledger_account.funds_available")
            }
            Self::SwapCompleted => serializer.serialize_str("swap.completed"),
            Self::DepositSucceeded => serializer.serialize_str("deposit.succeeded"),
            Self::TransferCreated => serializer.serialize_str("transfer.created"),
            Self::TransferCompleted => serializer.serialize_str("transfer.completed"),
            Self::TransferFailed => serializer.serialize_str("transfer.failed"),
            Self::WithdrawalCreated => serializer.serialize_str("withdrawal.created"),
            Self::WithdrawalUpdated => serializer.serialize_str("withdrawal.updated"),
            Self::WithdrawalReversed => serializer.serialize_str("withdrawal.reversed"),
            Self::CardTransactionCreated => serializer.serialize_str("card_transaction.created"),
            Self::CardTransactionUpdated => serializer.serialize_str("card_transaction.updated"),
            Self::CardTransactionCompleted => {
                serializer.serialize_str("card_transaction.completed")
            }
            Self::CardTransactionDeclined => serializer.serialize_str("card_transaction.declined"),
            Self::CardTransactionReversed => serializer.serialize_str("card_transaction.reversed"),
            Self::CardCreated => serializer.serialize_str("card.created"),
            Self::CardUpdated => serializer.serialize_str("card.updated"),
            Self::CardFrozen => serializer.serialize_str("card.frozen"),
            Self::CardCanceled => serializer.serialize_str("card.canceled"),
            Self::CardApplicationCreated => serializer.serialize_str("card_application.created"),
            Self::CardApplicationUpdated => serializer.serialize_str("card_application.updated"),
            Self::CardApplicationApproved => serializer.serialize_str("card_application.approved"),
            Self::CardApplicationDenied => serializer.serialize_str("card_application.denied"),
            Self::CourseLessonInteractionCompleted => {
                serializer.serialize_str("course_lesson_interaction.completed")
            }
            Self::PayoutMethodCreated => serializer.serialize_str("payout_method.created"),
            Self::VerificationSucceeded => serializer.serialize_str("verification.succeeded"),
            Self::IdentityProfileApproved => serializer.serialize_str("identity_profile.approved"),
            Self::IdentityProfileRejected => serializer.serialize_str("identity_profile.rejected"),
            Self::IdentityProfileNeedsAction => {
                serializer.serialize_str("identity_profile.needs_action")
            }
            Self::IdentityProfileUpdated => serializer.serialize_str("identity_profile.updated"),
            Self::PayoutAccountStatusUpdated => {
                serializer.serialize_str("payout_account.status_updated")
            }
            Self::ResolutionCenterCaseCreated => {
                serializer.serialize_str("resolution_center_case.created")
            }
            Self::ResolutionCenterCaseUpdated => {
                serializer.serialize_str("resolution_center_case.updated")
            }
            Self::ResolutionCenterCaseDecided => {
                serializer.serialize_str("resolution_center_case.decided")
            }
            Self::ProductCreated => serializer.serialize_str("product.created"),
            Self::ProductUpdated => serializer.serialize_str("product.updated"),
            Self::ProductDeleted => serializer.serialize_str("product.deleted"),
            Self::ProductPublished => serializer.serialize_str("product.published"),
            Self::ProductUnpublished => serializer.serialize_str("product.unpublished"),
            Self::PlanCreated => serializer.serialize_str("plan.created"),
            Self::PlanUpdated => serializer.serialize_str("plan.updated"),
            Self::PlanDeleted => serializer.serialize_str("plan.deleted"),
            Self::ShipmentCreated => serializer.serialize_str("shipment.created"),
            Self::ShipmentUpdated => serializer.serialize_str("shipment.updated"),
            Self::MemberCreated => serializer.serialize_str("member.created"),
            Self::AdCampaignPaymentFailed => serializer.serialize_str("ad_campaign.payment_failed"),
            Self::ChatMessageCreated => serializer.serialize_str("chat.message.created"),
            Self::ChatReactionCreated => serializer.serialize_str("chat.reaction.created"),
            Self::PaymentCreated => serializer.serialize_str("payment.created"),
            Self::PaymentSucceeded => serializer.serialize_str("payment.succeeded"),
            Self::PaymentFailed => serializer.serialize_str("payment.failed"),
            Self::PaymentPending => serializer.serialize_str("payment.pending"),
            Self::DisputeCreated => serializer.serialize_str("dispute.created"),
            Self::DisputeUpdated => serializer.serialize_str("dispute.updated"),
            Self::RefundCreated => serializer.serialize_str("refund.created"),
            Self::RefundUpdated => serializer.serialize_str("refund.updated"),
            Self::DisputeAlertCreated => serializer.serialize_str("dispute_alert.created"),
            Self::MembershipCancelAtPeriodEndChanged => {
                serializer.serialize_str("membership.cancel_at_period_end_changed")
            }
            Self::MembershipWentValidUnderscore => {
                serializer.serialize_str("membership_went_valid")
            }
            Self::MembershipWentInvalidUnderscore => {
                serializer.serialize_str("membership_went_invalid")
            }
            Self::MembershipMetadataUpdatedUnderscore => {
                serializer.serialize_str("membership_metadata_updated")
            }
            Self::ResolutionCreatedUnderscore => serializer.serialize_str("resolution_created"),
            Self::ResolutionUpdatedUnderscore => serializer.serialize_str("resolution_updated"),
            Self::ResolutionDecidedUnderscore => serializer.serialize_str("resolution_decided"),
            Self::PaymentAffiliateRewardCreatedUnderscore => {
                serializer.serialize_str("payment_affiliate_reward_created")
            }
            Self::MembershipExperienceClaimedUnderscore => {
                serializer.serialize_str("membership_experience_claimed")
            }
            Self::AppMembershipWentValidUnderscore => {
                serializer.serialize_str("app_membership_went_valid")
            }
            Self::AppMembershipWentInvalidUnderscore => {
                serializer.serialize_str("app_membership_went_invalid")
            }
            Self::AppPaymentCreatedUnderscore => serializer.serialize_str("app_payment_created"),
            Self::AppPaymentSucceededUnderscore => {
                serializer.serialize_str("app_payment_succeeded")
            }
            Self::AppPaymentFailedUnderscore => serializer.serialize_str("app_payment_failed"),
            Self::AppPaymentPendingUnderscore => serializer.serialize_str("app_payment_pending"),
            Self::AppMembershipCancelAtPeriodEndChangedUnderscore => {
                serializer.serialize_str("app_membership_cancel_at_period_end_changed")
            }
            Self::PaymentCreatedUnderscore => serializer.serialize_str("payment_created"),
            Self::PaymentSucceededUnderscore => serializer.serialize_str("payment_succeeded"),
            Self::PaymentFailedUnderscore => serializer.serialize_str("payment_failed"),
            Self::PaymentPendingUnderscore => serializer.serialize_str("payment_pending"),
            Self::DisputeCreatedUnderscore => serializer.serialize_str("dispute_created"),
            Self::DisputeUpdatedUnderscore => serializer.serialize_str("dispute_updated"),
            Self::RefundCreatedUnderscore => serializer.serialize_str("refund_created"),
            Self::RefundUpdatedUnderscore => serializer.serialize_str("refund_updated"),
            Self::DisputeAlertCreatedUnderscore => {
                serializer.serialize_str("dispute_alert_created")
            }
            Self::MembershipCancelAtPeriodEndChangedUnderscore => {
                serializer.serialize_str("membership_cancel_at_period_end_changed")
            }
            Self::MembershipWentValid => serializer.serialize_str("membership.went_valid"),
            Self::MembershipWentInvalid => serializer.serialize_str("membership.went_invalid"),
            Self::MembershipMetadataUpdated => {
                serializer.serialize_str("membership.metadata_updated")
            }
            Self::ResolutionCreated => serializer.serialize_str("resolution.created"),
            Self::ResolutionUpdated => serializer.serialize_str("resolution.updated"),
            Self::ResolutionDecided => serializer.serialize_str("resolution.decided"),
            Self::PaymentAffiliateRewardCreated => {
                serializer.serialize_str("payment.affiliate_reward_created")
            }
            Self::MembershipExperienceClaimed => {
                serializer.serialize_str("membership.experience_claimed")
            }
            Self::AppMembershipWentValid => serializer.serialize_str("app_membership.went_valid"),
            Self::AppMembershipWentInvalid => {
                serializer.serialize_str("app_membership.went_invalid")
            }
            Self::AppPaymentCreated => serializer.serialize_str("app_payment.created"),
            Self::AppPaymentSucceeded => serializer.serialize_str("app_payment.succeeded"),
            Self::AppPaymentFailed => serializer.serialize_str("app_payment.failed"),
            Self::AppPaymentPending => serializer.serialize_str("app_payment.pending"),
            Self::AppMembershipCancelAtPeriodEndChanged => {
                serializer.serialize_str("app_membership.cancel_at_period_end_changed")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateWebhooksRequestEventsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "account.updated" => Ok(Self::AccountUpdated),
            "invoice.created" => Ok(Self::InvoiceCreated),
            "invoice.marked_uncollectible" => Ok(Self::InvoiceMarkedUncollectible),
            "invoice.paid" => Ok(Self::InvoicePaid),
            "invoice.past_due" => Ok(Self::InvoicePastDue),
            "invoice.voided" => Ok(Self::InvoiceVoided),
            "membership.activated" => Ok(Self::MembershipActivated),
            "membership.deactivated" => Ok(Self::MembershipDeactivated),
            "membership.trial_ending_soon" => Ok(Self::MembershipTrialEndingSoon),
            "entry.created" => Ok(Self::EntryCreated),
            "entry.approved" => Ok(Self::EntryApproved),
            "entry.denied" => Ok(Self::EntryDenied),
            "entry.deleted" => Ok(Self::EntryDeleted),
            "export.completed" => Ok(Self::ExportCompleted),
            "export.failed" => Ok(Self::ExportFailed),
            "setup_intent.requires_action" => Ok(Self::SetupIntentRequiresAction),
            "setup_intent.succeeded" => Ok(Self::SetupIntentSucceeded),
            "setup_intent.canceled" => Ok(Self::SetupIntentCanceled),
            "ledger_account.funds_available" => Ok(Self::LedgerAccountFundsAvailable),
            "swap.completed" => Ok(Self::SwapCompleted),
            "deposit.succeeded" => Ok(Self::DepositSucceeded),
            "transfer.created" => Ok(Self::TransferCreated),
            "transfer.completed" => Ok(Self::TransferCompleted),
            "transfer.failed" => Ok(Self::TransferFailed),
            "withdrawal.created" => Ok(Self::WithdrawalCreated),
            "withdrawal.updated" => Ok(Self::WithdrawalUpdated),
            "withdrawal.reversed" => Ok(Self::WithdrawalReversed),
            "card_transaction.created" => Ok(Self::CardTransactionCreated),
            "card_transaction.updated" => Ok(Self::CardTransactionUpdated),
            "card_transaction.completed" => Ok(Self::CardTransactionCompleted),
            "card_transaction.declined" => Ok(Self::CardTransactionDeclined),
            "card_transaction.reversed" => Ok(Self::CardTransactionReversed),
            "card.created" => Ok(Self::CardCreated),
            "card.updated" => Ok(Self::CardUpdated),
            "card.frozen" => Ok(Self::CardFrozen),
            "card.canceled" => Ok(Self::CardCanceled),
            "card_application.created" => Ok(Self::CardApplicationCreated),
            "card_application.updated" => Ok(Self::CardApplicationUpdated),
            "card_application.approved" => Ok(Self::CardApplicationApproved),
            "card_application.denied" => Ok(Self::CardApplicationDenied),
            "course_lesson_interaction.completed" => Ok(Self::CourseLessonInteractionCompleted),
            "payout_method.created" => Ok(Self::PayoutMethodCreated),
            "verification.succeeded" => Ok(Self::VerificationSucceeded),
            "identity_profile.approved" => Ok(Self::IdentityProfileApproved),
            "identity_profile.rejected" => Ok(Self::IdentityProfileRejected),
            "identity_profile.needs_action" => Ok(Self::IdentityProfileNeedsAction),
            "identity_profile.updated" => Ok(Self::IdentityProfileUpdated),
            "payout_account.status_updated" => Ok(Self::PayoutAccountStatusUpdated),
            "resolution_center_case.created" => Ok(Self::ResolutionCenterCaseCreated),
            "resolution_center_case.updated" => Ok(Self::ResolutionCenterCaseUpdated),
            "resolution_center_case.decided" => Ok(Self::ResolutionCenterCaseDecided),
            "product.created" => Ok(Self::ProductCreated),
            "product.updated" => Ok(Self::ProductUpdated),
            "product.deleted" => Ok(Self::ProductDeleted),
            "product.published" => Ok(Self::ProductPublished),
            "product.unpublished" => Ok(Self::ProductUnpublished),
            "plan.created" => Ok(Self::PlanCreated),
            "plan.updated" => Ok(Self::PlanUpdated),
            "plan.deleted" => Ok(Self::PlanDeleted),
            "shipment.created" => Ok(Self::ShipmentCreated),
            "shipment.updated" => Ok(Self::ShipmentUpdated),
            "member.created" => Ok(Self::MemberCreated),
            "ad_campaign.payment_failed" => Ok(Self::AdCampaignPaymentFailed),
            "chat.message.created" => Ok(Self::ChatMessageCreated),
            "chat.reaction.created" => Ok(Self::ChatReactionCreated),
            "payment.created" => Ok(Self::PaymentCreated),
            "payment.succeeded" => Ok(Self::PaymentSucceeded),
            "payment.failed" => Ok(Self::PaymentFailed),
            "payment.pending" => Ok(Self::PaymentPending),
            "dispute.created" => Ok(Self::DisputeCreated),
            "dispute.updated" => Ok(Self::DisputeUpdated),
            "refund.created" => Ok(Self::RefundCreated),
            "refund.updated" => Ok(Self::RefundUpdated),
            "dispute_alert.created" => Ok(Self::DisputeAlertCreated),
            "membership.cancel_at_period_end_changed" => {
                Ok(Self::MembershipCancelAtPeriodEndChanged)
            }
            "membership_went_valid" => Ok(Self::MembershipWentValidUnderscore),
            "membership_went_invalid" => Ok(Self::MembershipWentInvalidUnderscore),
            "membership_metadata_updated" => Ok(Self::MembershipMetadataUpdatedUnderscore),
            "resolution_created" => Ok(Self::ResolutionCreatedUnderscore),
            "resolution_updated" => Ok(Self::ResolutionUpdatedUnderscore),
            "resolution_decided" => Ok(Self::ResolutionDecidedUnderscore),
            "payment_affiliate_reward_created" => Ok(Self::PaymentAffiliateRewardCreatedUnderscore),
            "membership_experience_claimed" => Ok(Self::MembershipExperienceClaimedUnderscore),
            "app_membership_went_valid" => Ok(Self::AppMembershipWentValidUnderscore),
            "app_membership_went_invalid" => Ok(Self::AppMembershipWentInvalidUnderscore),
            "app_payment_created" => Ok(Self::AppPaymentCreatedUnderscore),
            "app_payment_succeeded" => Ok(Self::AppPaymentSucceededUnderscore),
            "app_payment_failed" => Ok(Self::AppPaymentFailedUnderscore),
            "app_payment_pending" => Ok(Self::AppPaymentPendingUnderscore),
            "app_membership_cancel_at_period_end_changed" => {
                Ok(Self::AppMembershipCancelAtPeriodEndChangedUnderscore)
            }
            "payment_created" => Ok(Self::PaymentCreatedUnderscore),
            "payment_succeeded" => Ok(Self::PaymentSucceededUnderscore),
            "payment_failed" => Ok(Self::PaymentFailedUnderscore),
            "payment_pending" => Ok(Self::PaymentPendingUnderscore),
            "dispute_created" => Ok(Self::DisputeCreatedUnderscore),
            "dispute_updated" => Ok(Self::DisputeUpdatedUnderscore),
            "refund_created" => Ok(Self::RefundCreatedUnderscore),
            "refund_updated" => Ok(Self::RefundUpdatedUnderscore),
            "dispute_alert_created" => Ok(Self::DisputeAlertCreatedUnderscore),
            "membership_cancel_at_period_end_changed" => {
                Ok(Self::MembershipCancelAtPeriodEndChangedUnderscore)
            }
            "membership.went_valid" => Ok(Self::MembershipWentValid),
            "membership.went_invalid" => Ok(Self::MembershipWentInvalid),
            "membership.metadata_updated" => Ok(Self::MembershipMetadataUpdated),
            "resolution.created" => Ok(Self::ResolutionCreated),
            "resolution.updated" => Ok(Self::ResolutionUpdated),
            "resolution.decided" => Ok(Self::ResolutionDecided),
            "payment.affiliate_reward_created" => Ok(Self::PaymentAffiliateRewardCreated),
            "membership.experience_claimed" => Ok(Self::MembershipExperienceClaimed),
            "app_membership.went_valid" => Ok(Self::AppMembershipWentValid),
            "app_membership.went_invalid" => Ok(Self::AppMembershipWentInvalid),
            "app_payment.created" => Ok(Self::AppPaymentCreated),
            "app_payment.succeeded" => Ok(Self::AppPaymentSucceeded),
            "app_payment.failed" => Ok(Self::AppPaymentFailed),
            "app_payment.pending" => Ok(Self::AppPaymentPending),
            "app_membership.cancel_at_period_end_changed" => {
                Ok(Self::AppMembershipCancelAtPeriodEndChanged)
            }
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateWebhooksRequestEventsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountUpdated => write!(f, "account.updated"),
            Self::InvoiceCreated => write!(f, "invoice.created"),
            Self::InvoiceMarkedUncollectible => write!(f, "invoice.marked_uncollectible"),
            Self::InvoicePaid => write!(f, "invoice.paid"),
            Self::InvoicePastDue => write!(f, "invoice.past_due"),
            Self::InvoiceVoided => write!(f, "invoice.voided"),
            Self::MembershipActivated => write!(f, "membership.activated"),
            Self::MembershipDeactivated => write!(f, "membership.deactivated"),
            Self::MembershipTrialEndingSoon => write!(f, "membership.trial_ending_soon"),
            Self::EntryCreated => write!(f, "entry.created"),
            Self::EntryApproved => write!(f, "entry.approved"),
            Self::EntryDenied => write!(f, "entry.denied"),
            Self::EntryDeleted => write!(f, "entry.deleted"),
            Self::ExportCompleted => write!(f, "export.completed"),
            Self::ExportFailed => write!(f, "export.failed"),
            Self::SetupIntentRequiresAction => write!(f, "setup_intent.requires_action"),
            Self::SetupIntentSucceeded => write!(f, "setup_intent.succeeded"),
            Self::SetupIntentCanceled => write!(f, "setup_intent.canceled"),
            Self::LedgerAccountFundsAvailable => write!(f, "ledger_account.funds_available"),
            Self::SwapCompleted => write!(f, "swap.completed"),
            Self::DepositSucceeded => write!(f, "deposit.succeeded"),
            Self::TransferCreated => write!(f, "transfer.created"),
            Self::TransferCompleted => write!(f, "transfer.completed"),
            Self::TransferFailed => write!(f, "transfer.failed"),
            Self::WithdrawalCreated => write!(f, "withdrawal.created"),
            Self::WithdrawalUpdated => write!(f, "withdrawal.updated"),
            Self::WithdrawalReversed => write!(f, "withdrawal.reversed"),
            Self::CardTransactionCreated => write!(f, "card_transaction.created"),
            Self::CardTransactionUpdated => write!(f, "card_transaction.updated"),
            Self::CardTransactionCompleted => write!(f, "card_transaction.completed"),
            Self::CardTransactionDeclined => write!(f, "card_transaction.declined"),
            Self::CardTransactionReversed => write!(f, "card_transaction.reversed"),
            Self::CardCreated => write!(f, "card.created"),
            Self::CardUpdated => write!(f, "card.updated"),
            Self::CardFrozen => write!(f, "card.frozen"),
            Self::CardCanceled => write!(f, "card.canceled"),
            Self::CardApplicationCreated => write!(f, "card_application.created"),
            Self::CardApplicationUpdated => write!(f, "card_application.updated"),
            Self::CardApplicationApproved => write!(f, "card_application.approved"),
            Self::CardApplicationDenied => write!(f, "card_application.denied"),
            Self::CourseLessonInteractionCompleted => {
                write!(f, "course_lesson_interaction.completed")
            }
            Self::PayoutMethodCreated => write!(f, "payout_method.created"),
            Self::VerificationSucceeded => write!(f, "verification.succeeded"),
            Self::IdentityProfileApproved => write!(f, "identity_profile.approved"),
            Self::IdentityProfileRejected => write!(f, "identity_profile.rejected"),
            Self::IdentityProfileNeedsAction => write!(f, "identity_profile.needs_action"),
            Self::IdentityProfileUpdated => write!(f, "identity_profile.updated"),
            Self::PayoutAccountStatusUpdated => write!(f, "payout_account.status_updated"),
            Self::ResolutionCenterCaseCreated => write!(f, "resolution_center_case.created"),
            Self::ResolutionCenterCaseUpdated => write!(f, "resolution_center_case.updated"),
            Self::ResolutionCenterCaseDecided => write!(f, "resolution_center_case.decided"),
            Self::ProductCreated => write!(f, "product.created"),
            Self::ProductUpdated => write!(f, "product.updated"),
            Self::ProductDeleted => write!(f, "product.deleted"),
            Self::ProductPublished => write!(f, "product.published"),
            Self::ProductUnpublished => write!(f, "product.unpublished"),
            Self::PlanCreated => write!(f, "plan.created"),
            Self::PlanUpdated => write!(f, "plan.updated"),
            Self::PlanDeleted => write!(f, "plan.deleted"),
            Self::ShipmentCreated => write!(f, "shipment.created"),
            Self::ShipmentUpdated => write!(f, "shipment.updated"),
            Self::MemberCreated => write!(f, "member.created"),
            Self::AdCampaignPaymentFailed => write!(f, "ad_campaign.payment_failed"),
            Self::ChatMessageCreated => write!(f, "chat.message.created"),
            Self::ChatReactionCreated => write!(f, "chat.reaction.created"),
            Self::PaymentCreated => write!(f, "payment.created"),
            Self::PaymentSucceeded => write!(f, "payment.succeeded"),
            Self::PaymentFailed => write!(f, "payment.failed"),
            Self::PaymentPending => write!(f, "payment.pending"),
            Self::DisputeCreated => write!(f, "dispute.created"),
            Self::DisputeUpdated => write!(f, "dispute.updated"),
            Self::RefundCreated => write!(f, "refund.created"),
            Self::RefundUpdated => write!(f, "refund.updated"),
            Self::DisputeAlertCreated => write!(f, "dispute_alert.created"),
            Self::MembershipCancelAtPeriodEndChanged => {
                write!(f, "membership.cancel_at_period_end_changed")
            }
            Self::MembershipWentValidUnderscore => write!(f, "membership_went_valid"),
            Self::MembershipWentInvalidUnderscore => write!(f, "membership_went_invalid"),
            Self::MembershipMetadataUpdatedUnderscore => write!(f, "membership_metadata_updated"),
            Self::ResolutionCreatedUnderscore => write!(f, "resolution_created"),
            Self::ResolutionUpdatedUnderscore => write!(f, "resolution_updated"),
            Self::ResolutionDecidedUnderscore => write!(f, "resolution_decided"),
            Self::PaymentAffiliateRewardCreatedUnderscore => {
                write!(f, "payment_affiliate_reward_created")
            }
            Self::MembershipExperienceClaimedUnderscore => {
                write!(f, "membership_experience_claimed")
            }
            Self::AppMembershipWentValidUnderscore => write!(f, "app_membership_went_valid"),
            Self::AppMembershipWentInvalidUnderscore => write!(f, "app_membership_went_invalid"),
            Self::AppPaymentCreatedUnderscore => write!(f, "app_payment_created"),
            Self::AppPaymentSucceededUnderscore => write!(f, "app_payment_succeeded"),
            Self::AppPaymentFailedUnderscore => write!(f, "app_payment_failed"),
            Self::AppPaymentPendingUnderscore => write!(f, "app_payment_pending"),
            Self::AppMembershipCancelAtPeriodEndChangedUnderscore => {
                write!(f, "app_membership_cancel_at_period_end_changed")
            }
            Self::PaymentCreatedUnderscore => write!(f, "payment_created"),
            Self::PaymentSucceededUnderscore => write!(f, "payment_succeeded"),
            Self::PaymentFailedUnderscore => write!(f, "payment_failed"),
            Self::PaymentPendingUnderscore => write!(f, "payment_pending"),
            Self::DisputeCreatedUnderscore => write!(f, "dispute_created"),
            Self::DisputeUpdatedUnderscore => write!(f, "dispute_updated"),
            Self::RefundCreatedUnderscore => write!(f, "refund_created"),
            Self::RefundUpdatedUnderscore => write!(f, "refund_updated"),
            Self::DisputeAlertCreatedUnderscore => write!(f, "dispute_alert_created"),
            Self::MembershipCancelAtPeriodEndChangedUnderscore => {
                write!(f, "membership_cancel_at_period_end_changed")
            }
            Self::MembershipWentValid => write!(f, "membership.went_valid"),
            Self::MembershipWentInvalid => write!(f, "membership.went_invalid"),
            Self::MembershipMetadataUpdated => write!(f, "membership.metadata_updated"),
            Self::ResolutionCreated => write!(f, "resolution.created"),
            Self::ResolutionUpdated => write!(f, "resolution.updated"),
            Self::ResolutionDecided => write!(f, "resolution.decided"),
            Self::PaymentAffiliateRewardCreated => write!(f, "payment.affiliate_reward_created"),
            Self::MembershipExperienceClaimed => write!(f, "membership.experience_claimed"),
            Self::AppMembershipWentValid => write!(f, "app_membership.went_valid"),
            Self::AppMembershipWentInvalid => write!(f, "app_membership.went_invalid"),
            Self::AppPaymentCreated => write!(f, "app_payment.created"),
            Self::AppPaymentSucceeded => write!(f, "app_payment.succeeded"),
            Self::AppPaymentFailed => write!(f, "app_payment.failed"),
            Self::AppPaymentPending => write!(f, "app_payment.pending"),
            Self::AppMembershipCancelAtPeriodEndChanged => {
                write!(f, "app_membership.cancel_at_period_end_changed")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
