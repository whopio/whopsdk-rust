pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum InviteMembershipsRequestBody {
    InviteMembershipsRequestBodyUserId(InviteMembershipsRequestBodyUserId),

    InviteMembershipsRequestBodyEmail(InviteMembershipsRequestBodyEmail),
}

impl InviteMembershipsRequestBody {
    pub fn is_invite_memberships_request_body_user_id(&self) -> bool {
        matches!(self, Self::InviteMembershipsRequestBodyUserId(_))
    }

    pub fn is_invite_memberships_request_body_email(&self) -> bool {
        matches!(self, Self::InviteMembershipsRequestBodyEmail(_))
    }

    pub fn as_invite_memberships_request_body_user_id(
        &self,
    ) -> Option<&InviteMembershipsRequestBodyUserId> {
        match self {
            Self::InviteMembershipsRequestBodyUserId(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_invite_memberships_request_body_user_id(
        self,
    ) -> Option<InviteMembershipsRequestBodyUserId> {
        match self {
            Self::InviteMembershipsRequestBodyUserId(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_invite_memberships_request_body_email(
        &self,
    ) -> Option<&InviteMembershipsRequestBodyEmail> {
        match self {
            Self::InviteMembershipsRequestBodyEmail(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_invite_memberships_request_body_email(
        self,
    ) -> Option<InviteMembershipsRequestBodyEmail> {
        match self {
            Self::InviteMembershipsRequestBodyEmail(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for InviteMembershipsRequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InviteMembershipsRequestBodyUserId(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::InviteMembershipsRequestBodyEmail(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
