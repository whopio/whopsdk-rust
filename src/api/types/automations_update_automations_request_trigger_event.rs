pub use crate::prelude::*;

/// The webhook event that enrolls the member it's about, such as `payment.succeeded`. `null` for an automation that only `POST /automations/{id}/trigger` enrolls into. Fixed once the automation has been published.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAutomationsRequestTriggerEvent {
    PaymentSucceeded,
    PaymentFailed,
    MembershipActivated,
    MembershipTrialEndingSoon,
    MemberCreated,
    InvoiceCreated,
    InvoicePaid,
    InvoicePastDue,
    RefundCreated,
    DisputeCreated,
    ResolutionCenterCaseCreated,
    ResolutionCenterCaseDecided,
    CourseLessonInteractionCompleted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAutomationsRequestTriggerEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PaymentSucceeded => serializer.serialize_str("payment.succeeded"),
            Self::PaymentFailed => serializer.serialize_str("payment.failed"),
            Self::MembershipActivated => serializer.serialize_str("membership.activated"),
            Self::MembershipTrialEndingSoon => {
                serializer.serialize_str("membership.trial_ending_soon")
            }
            Self::MemberCreated => serializer.serialize_str("member.created"),
            Self::InvoiceCreated => serializer.serialize_str("invoice.created"),
            Self::InvoicePaid => serializer.serialize_str("invoice.paid"),
            Self::InvoicePastDue => serializer.serialize_str("invoice.past_due"),
            Self::RefundCreated => serializer.serialize_str("refund.created"),
            Self::DisputeCreated => serializer.serialize_str("dispute.created"),
            Self::ResolutionCenterCaseCreated => {
                serializer.serialize_str("resolution_center_case.created")
            }
            Self::ResolutionCenterCaseDecided => {
                serializer.serialize_str("resolution_center_case.decided")
            }
            Self::CourseLessonInteractionCompleted => {
                serializer.serialize_str("course_lesson_interaction.completed")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAutomationsRequestTriggerEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payment.succeeded" => Ok(Self::PaymentSucceeded),
            "payment.failed" => Ok(Self::PaymentFailed),
            "membership.activated" => Ok(Self::MembershipActivated),
            "membership.trial_ending_soon" => Ok(Self::MembershipTrialEndingSoon),
            "member.created" => Ok(Self::MemberCreated),
            "invoice.created" => Ok(Self::InvoiceCreated),
            "invoice.paid" => Ok(Self::InvoicePaid),
            "invoice.past_due" => Ok(Self::InvoicePastDue),
            "refund.created" => Ok(Self::RefundCreated),
            "dispute.created" => Ok(Self::DisputeCreated),
            "resolution_center_case.created" => Ok(Self::ResolutionCenterCaseCreated),
            "resolution_center_case.decided" => Ok(Self::ResolutionCenterCaseDecided),
            "course_lesson_interaction.completed" => Ok(Self::CourseLessonInteractionCompleted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAutomationsRequestTriggerEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PaymentSucceeded => write!(f, "payment.succeeded"),
            Self::PaymentFailed => write!(f, "payment.failed"),
            Self::MembershipActivated => write!(f, "membership.activated"),
            Self::MembershipTrialEndingSoon => write!(f, "membership.trial_ending_soon"),
            Self::MemberCreated => write!(f, "member.created"),
            Self::InvoiceCreated => write!(f, "invoice.created"),
            Self::InvoicePaid => write!(f, "invoice.paid"),
            Self::InvoicePastDue => write!(f, "invoice.past_due"),
            Self::RefundCreated => write!(f, "refund.created"),
            Self::DisputeCreated => write!(f, "dispute.created"),
            Self::ResolutionCenterCaseCreated => write!(f, "resolution_center_case.created"),
            Self::ResolutionCenterCaseDecided => write!(f, "resolution_center_case.decided"),
            Self::CourseLessonInteractionCompleted => {
                write!(f, "course_lesson_interaction.completed")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
