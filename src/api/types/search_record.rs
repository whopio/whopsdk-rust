pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "object")]
#[non_exhaustive]
pub enum SearchRecord {
    #[serde(rename = "account")]
    #[non_exhaustive]
    Account {
        #[serde(skip_serializing_if = "Option::is_none")]
        banner_image_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent_account_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subtitle: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    #[serde(rename = "affiliate")]
    #[non_exhaustive]
    Affiliate {
        #[serde(default)]
        account_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subtitle: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default)]
        total_referrals_count: i64,
    },

    #[serde(rename = "member")]
    #[non_exhaustive]
    Member {
        #[serde(default)]
        account_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        status: SearchRecordMemberStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        subtitle: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
    },

    #[serde(rename = "membership")]
    #[non_exhaustive]
    Membership {
        #[serde(default)]
        account_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(default)]
        product_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<SearchRecordMembershipStatus>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subtitle: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    #[serde(rename = "payment")]
    #[non_exhaustive]
    Payment {
        #[serde(default)]
        account_id: String,
        #[serde(default)]
        created_at: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        paid_at: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        presentment_total: Option<Money>,
        status: ReceiptStatus,
        substatus: FriendlyReceiptStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        subtitle: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        total: Option<Money>,
    },

    #[serde(rename = "promo_code")]
    #[non_exhaustive]
    PromoCode {
        #[serde(default)]
        account_id: String,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        amount_off: f64,
        currency: SearchRecordPromoCodeCurrency,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(default)]
        promo_duration_months: i64,
        promo_type: SearchRecordPromoCodePromoType,
        #[serde(skip_serializing_if = "Option::is_none")]
        subtitle: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    #[serde(rename = "resolution_center_case")]
    #[non_exhaustive]
    ResolutionCenterCase {
        #[serde(default)]
        account_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subtitle: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    #[serde(rename = "review")]
    #[non_exhaustive]
    Review {
        #[serde(default)]
        account_id: String,
        #[serde(default)]
        created_at: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        subtitle: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl SearchRecord {
    pub fn account(id: String) -> Self {
        Self::Account {
            banner_image_url: None,
            description: None,
            id,
            image_url: None,
            parent_account_id: None,
            subtitle: None,
            title: None,
        }
    }

    pub fn affiliate(account_id: String, id: String, total_referrals_count: i64) -> Self {
        Self::Affiliate {
            account_id,
            description: None,
            id,
            image_url: None,
            subtitle: None,
            title: None,
            total_referrals_count,
        }
    }

    pub fn member(account_id: String, id: String, status: SearchRecordMemberStatus) -> Self {
        Self::Member {
            account_id,
            description: None,
            email: None,
            id,
            image_url: None,
            status,
            subtitle: None,
            title: None,
            username: None,
        }
    }

    pub fn membership(account_id: String, id: String, product_id: String) -> Self {
        Self::Membership {
            account_id,
            description: None,
            id,
            image_url: None,
            product_id,
            status: None,
            subtitle: None,
            title: None,
        }
    }

    pub fn payment(
        account_id: String,
        created_at: String,
        id: String,
        status: ReceiptStatus,
        substatus: FriendlyReceiptStatus,
    ) -> Self {
        Self::Payment {
            account_id,
            created_at,
            description: None,
            id,
            image_url: None,
            paid_at: None,
            presentment_total: None,
            status,
            substatus,
            subtitle: None,
            title: None,
            total: None,
        }
    }

    pub fn promo_code(
        account_id: String,
        amount_off: f64,
        currency: SearchRecordPromoCodeCurrency,
        id: String,
        promo_duration_months: i64,
        promo_type: SearchRecordPromoCodePromoType,
    ) -> Self {
        Self::PromoCode {
            account_id,
            amount_off,
            currency,
            description: None,
            id,
            image_url: None,
            promo_duration_months,
            promo_type,
            subtitle: None,
            title: None,
        }
    }

    pub fn resolution_center_case(account_id: String, id: String) -> Self {
        Self::ResolutionCenterCase {
            account_id,
            description: None,
            id,
            image_url: None,
            subtitle: None,
            title: None,
        }
    }

    pub fn review(account_id: String, created_at: String, id: String) -> Self {
        Self::Review {
            account_id,
            created_at,
            description: None,
            id,
            image_url: None,
            subtitle: None,
            title: None,
        }
    }

    pub fn account_with_banner_image_url(
        banner_image_url: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        parent_account_id: Option<String>,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Account {
            banner_image_url: Some(banner_image_url),
            description,
            id,
            image_url,
            parent_account_id,
            subtitle,
            title,
        }
    }

    pub fn account_with_description(
        banner_image_url: Option<String>,
        description: String,
        id: String,
        image_url: Option<String>,
        parent_account_id: Option<String>,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Account {
            banner_image_url,
            description: Some(description),
            id,
            image_url,
            parent_account_id,
            subtitle,
            title,
        }
    }

    pub fn account_with_image_url(
        banner_image_url: Option<String>,
        description: Option<String>,
        id: String,
        image_url: String,
        parent_account_id: Option<String>,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Account {
            banner_image_url,
            description,
            id,
            image_url: Some(image_url),
            parent_account_id,
            subtitle,
            title,
        }
    }

    pub fn account_with_parent_account_id(
        banner_image_url: Option<String>,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        parent_account_id: String,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Account {
            banner_image_url,
            description,
            id,
            image_url,
            parent_account_id: Some(parent_account_id),
            subtitle,
            title,
        }
    }

    pub fn account_with_subtitle(
        banner_image_url: Option<String>,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        parent_account_id: Option<String>,
        subtitle: String,
        title: Option<String>,
    ) -> Self {
        Self::Account {
            banner_image_url,
            description,
            id,
            image_url,
            parent_account_id,
            subtitle: Some(subtitle),
            title,
        }
    }

    pub fn account_with_title(
        banner_image_url: Option<String>,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        parent_account_id: Option<String>,
        subtitle: Option<String>,
        title: String,
    ) -> Self {
        Self::Account {
            banner_image_url,
            description,
            id,
            image_url,
            parent_account_id,
            subtitle,
            title: Some(title),
        }
    }

    pub fn affiliate_with_description(
        account_id: String,
        description: String,
        id: String,
        image_url: Option<String>,
        subtitle: Option<String>,
        title: Option<String>,
        total_referrals_count: i64,
    ) -> Self {
        Self::Affiliate {
            account_id,
            description: Some(description),
            id,
            image_url,
            subtitle,
            title,
            total_referrals_count,
        }
    }

    pub fn affiliate_with_image_url(
        account_id: String,
        description: Option<String>,
        id: String,
        image_url: String,
        subtitle: Option<String>,
        title: Option<String>,
        total_referrals_count: i64,
    ) -> Self {
        Self::Affiliate {
            account_id,
            description,
            id,
            image_url: Some(image_url),
            subtitle,
            title,
            total_referrals_count,
        }
    }

    pub fn affiliate_with_subtitle(
        account_id: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        subtitle: String,
        title: Option<String>,
        total_referrals_count: i64,
    ) -> Self {
        Self::Affiliate {
            account_id,
            description,
            id,
            image_url,
            subtitle: Some(subtitle),
            title,
            total_referrals_count,
        }
    }

    pub fn affiliate_with_title(
        account_id: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        subtitle: Option<String>,
        title: String,
        total_referrals_count: i64,
    ) -> Self {
        Self::Affiliate {
            account_id,
            description,
            id,
            image_url,
            subtitle,
            title: Some(title),
            total_referrals_count,
        }
    }

    pub fn member_with_description(
        account_id: String,
        description: String,
        email: Option<String>,
        id: String,
        image_url: Option<String>,
        status: SearchRecordMemberStatus,
        subtitle: Option<String>,
        title: Option<String>,
        username: Option<String>,
    ) -> Self {
        Self::Member {
            account_id,
            description: Some(description),
            email,
            id,
            image_url,
            status,
            subtitle,
            title,
            username,
        }
    }

    pub fn member_with_email(
        account_id: String,
        description: Option<String>,
        email: String,
        id: String,
        image_url: Option<String>,
        status: SearchRecordMemberStatus,
        subtitle: Option<String>,
        title: Option<String>,
        username: Option<String>,
    ) -> Self {
        Self::Member {
            account_id,
            description,
            email: Some(email),
            id,
            image_url,
            status,
            subtitle,
            title,
            username,
        }
    }

    pub fn member_with_image_url(
        account_id: String,
        description: Option<String>,
        email: Option<String>,
        id: String,
        image_url: String,
        status: SearchRecordMemberStatus,
        subtitle: Option<String>,
        title: Option<String>,
        username: Option<String>,
    ) -> Self {
        Self::Member {
            account_id,
            description,
            email,
            id,
            image_url: Some(image_url),
            status,
            subtitle,
            title,
            username,
        }
    }

    pub fn member_with_subtitle(
        account_id: String,
        description: Option<String>,
        email: Option<String>,
        id: String,
        image_url: Option<String>,
        status: SearchRecordMemberStatus,
        subtitle: String,
        title: Option<String>,
        username: Option<String>,
    ) -> Self {
        Self::Member {
            account_id,
            description,
            email,
            id,
            image_url,
            status,
            subtitle: Some(subtitle),
            title,
            username,
        }
    }

    pub fn member_with_title(
        account_id: String,
        description: Option<String>,
        email: Option<String>,
        id: String,
        image_url: Option<String>,
        status: SearchRecordMemberStatus,
        subtitle: Option<String>,
        title: String,
        username: Option<String>,
    ) -> Self {
        Self::Member {
            account_id,
            description,
            email,
            id,
            image_url,
            status,
            subtitle,
            title: Some(title),
            username,
        }
    }

    pub fn member_with_username(
        account_id: String,
        description: Option<String>,
        email: Option<String>,
        id: String,
        image_url: Option<String>,
        status: SearchRecordMemberStatus,
        subtitle: Option<String>,
        title: Option<String>,
        username: String,
    ) -> Self {
        Self::Member {
            account_id,
            description,
            email,
            id,
            image_url,
            status,
            subtitle,
            title,
            username: Some(username),
        }
    }

    pub fn membership_with_description(
        account_id: String,
        description: String,
        id: String,
        image_url: Option<String>,
        product_id: String,
        status: Option<SearchRecordMembershipStatus>,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Membership {
            account_id,
            description: Some(description),
            id,
            image_url,
            product_id,
            status,
            subtitle,
            title,
        }
    }

    pub fn membership_with_image_url(
        account_id: String,
        description: Option<String>,
        id: String,
        image_url: String,
        product_id: String,
        status: Option<SearchRecordMembershipStatus>,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Membership {
            account_id,
            description,
            id,
            image_url: Some(image_url),
            product_id,
            status,
            subtitle,
            title,
        }
    }

    pub fn membership_with_status(
        account_id: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        product_id: String,
        status: SearchRecordMembershipStatus,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Membership {
            account_id,
            description,
            id,
            image_url,
            product_id,
            status: Some(status),
            subtitle,
            title,
        }
    }

    pub fn membership_with_subtitle(
        account_id: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        product_id: String,
        status: Option<SearchRecordMembershipStatus>,
        subtitle: String,
        title: Option<String>,
    ) -> Self {
        Self::Membership {
            account_id,
            description,
            id,
            image_url,
            product_id,
            status,
            subtitle: Some(subtitle),
            title,
        }
    }

    pub fn membership_with_title(
        account_id: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        product_id: String,
        status: Option<SearchRecordMembershipStatus>,
        subtitle: Option<String>,
        title: String,
    ) -> Self {
        Self::Membership {
            account_id,
            description,
            id,
            image_url,
            product_id,
            status,
            subtitle,
            title: Some(title),
        }
    }

    pub fn payment_with_description(
        account_id: String,
        created_at: String,
        description: String,
        id: String,
        image_url: Option<String>,
        paid_at: Option<String>,
        presentment_total: Option<Money>,
        status: ReceiptStatus,
        substatus: FriendlyReceiptStatus,
        subtitle: Option<String>,
        title: Option<String>,
        total: Option<Money>,
    ) -> Self {
        Self::Payment {
            account_id,
            created_at,
            description: Some(description),
            id,
            image_url,
            paid_at,
            presentment_total,
            status,
            substatus,
            subtitle,
            title,
            total,
        }
    }

    pub fn payment_with_image_url(
        account_id: String,
        created_at: String,
        description: Option<String>,
        id: String,
        image_url: String,
        paid_at: Option<String>,
        presentment_total: Option<Money>,
        status: ReceiptStatus,
        substatus: FriendlyReceiptStatus,
        subtitle: Option<String>,
        title: Option<String>,
        total: Option<Money>,
    ) -> Self {
        Self::Payment {
            account_id,
            created_at,
            description,
            id,
            image_url: Some(image_url),
            paid_at,
            presentment_total,
            status,
            substatus,
            subtitle,
            title,
            total,
        }
    }

    pub fn payment_with_paid_at(
        account_id: String,
        created_at: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        paid_at: String,
        presentment_total: Option<Money>,
        status: ReceiptStatus,
        substatus: FriendlyReceiptStatus,
        subtitle: Option<String>,
        title: Option<String>,
        total: Option<Money>,
    ) -> Self {
        Self::Payment {
            account_id,
            created_at,
            description,
            id,
            image_url,
            paid_at: Some(paid_at),
            presentment_total,
            status,
            substatus,
            subtitle,
            title,
            total,
        }
    }

    pub fn payment_with_presentment_total(
        account_id: String,
        created_at: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        paid_at: Option<String>,
        presentment_total: Money,
        status: ReceiptStatus,
        substatus: FriendlyReceiptStatus,
        subtitle: Option<String>,
        title: Option<String>,
        total: Option<Money>,
    ) -> Self {
        Self::Payment {
            account_id,
            created_at,
            description,
            id,
            image_url,
            paid_at,
            presentment_total: Some(presentment_total),
            status,
            substatus,
            subtitle,
            title,
            total,
        }
    }

    pub fn payment_with_subtitle(
        account_id: String,
        created_at: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        paid_at: Option<String>,
        presentment_total: Option<Money>,
        status: ReceiptStatus,
        substatus: FriendlyReceiptStatus,
        subtitle: String,
        title: Option<String>,
        total: Option<Money>,
    ) -> Self {
        Self::Payment {
            account_id,
            created_at,
            description,
            id,
            image_url,
            paid_at,
            presentment_total,
            status,
            substatus,
            subtitle: Some(subtitle),
            title,
            total,
        }
    }

    pub fn payment_with_title(
        account_id: String,
        created_at: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        paid_at: Option<String>,
        presentment_total: Option<Money>,
        status: ReceiptStatus,
        substatus: FriendlyReceiptStatus,
        subtitle: Option<String>,
        title: String,
        total: Option<Money>,
    ) -> Self {
        Self::Payment {
            account_id,
            created_at,
            description,
            id,
            image_url,
            paid_at,
            presentment_total,
            status,
            substatus,
            subtitle,
            title: Some(title),
            total,
        }
    }

    pub fn payment_with_total(
        account_id: String,
        created_at: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        paid_at: Option<String>,
        presentment_total: Option<Money>,
        status: ReceiptStatus,
        substatus: FriendlyReceiptStatus,
        subtitle: Option<String>,
        title: Option<String>,
        total: Money,
    ) -> Self {
        Self::Payment {
            account_id,
            created_at,
            description,
            id,
            image_url,
            paid_at,
            presentment_total,
            status,
            substatus,
            subtitle,
            title,
            total: Some(total),
        }
    }

    pub fn promo_code_with_description(
        account_id: String,
        amount_off: f64,
        currency: SearchRecordPromoCodeCurrency,
        description: String,
        id: String,
        image_url: Option<String>,
        promo_duration_months: i64,
        promo_type: SearchRecordPromoCodePromoType,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::PromoCode {
            account_id,
            amount_off,
            currency,
            description: Some(description),
            id,
            image_url,
            promo_duration_months,
            promo_type,
            subtitle,
            title,
        }
    }

    pub fn promo_code_with_image_url(
        account_id: String,
        amount_off: f64,
        currency: SearchRecordPromoCodeCurrency,
        description: Option<String>,
        id: String,
        image_url: String,
        promo_duration_months: i64,
        promo_type: SearchRecordPromoCodePromoType,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::PromoCode {
            account_id,
            amount_off,
            currency,
            description,
            id,
            image_url: Some(image_url),
            promo_duration_months,
            promo_type,
            subtitle,
            title,
        }
    }

    pub fn promo_code_with_subtitle(
        account_id: String,
        amount_off: f64,
        currency: SearchRecordPromoCodeCurrency,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        promo_duration_months: i64,
        promo_type: SearchRecordPromoCodePromoType,
        subtitle: String,
        title: Option<String>,
    ) -> Self {
        Self::PromoCode {
            account_id,
            amount_off,
            currency,
            description,
            id,
            image_url,
            promo_duration_months,
            promo_type,
            subtitle: Some(subtitle),
            title,
        }
    }

    pub fn promo_code_with_title(
        account_id: String,
        amount_off: f64,
        currency: SearchRecordPromoCodeCurrency,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        promo_duration_months: i64,
        promo_type: SearchRecordPromoCodePromoType,
        subtitle: Option<String>,
        title: String,
    ) -> Self {
        Self::PromoCode {
            account_id,
            amount_off,
            currency,
            description,
            id,
            image_url,
            promo_duration_months,
            promo_type,
            subtitle,
            title: Some(title),
        }
    }

    pub fn resolution_center_case_with_description(
        account_id: String,
        description: String,
        id: String,
        image_url: Option<String>,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::ResolutionCenterCase {
            account_id,
            description: Some(description),
            id,
            image_url,
            subtitle,
            title,
        }
    }

    pub fn resolution_center_case_with_image_url(
        account_id: String,
        description: Option<String>,
        id: String,
        image_url: String,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::ResolutionCenterCase {
            account_id,
            description,
            id,
            image_url: Some(image_url),
            subtitle,
            title,
        }
    }

    pub fn resolution_center_case_with_subtitle(
        account_id: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        subtitle: String,
        title: Option<String>,
    ) -> Self {
        Self::ResolutionCenterCase {
            account_id,
            description,
            id,
            image_url,
            subtitle: Some(subtitle),
            title,
        }
    }

    pub fn resolution_center_case_with_title(
        account_id: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        subtitle: Option<String>,
        title: String,
    ) -> Self {
        Self::ResolutionCenterCase {
            account_id,
            description,
            id,
            image_url,
            subtitle,
            title: Some(title),
        }
    }

    pub fn review_with_description(
        account_id: String,
        created_at: String,
        description: String,
        id: String,
        image_url: Option<String>,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Review {
            account_id,
            created_at,
            description: Some(description),
            id,
            image_url,
            subtitle,
            title,
        }
    }

    pub fn review_with_image_url(
        account_id: String,
        created_at: String,
        description: Option<String>,
        id: String,
        image_url: String,
        subtitle: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Review {
            account_id,
            created_at,
            description,
            id,
            image_url: Some(image_url),
            subtitle,
            title,
        }
    }

    pub fn review_with_subtitle(
        account_id: String,
        created_at: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        subtitle: String,
        title: Option<String>,
    ) -> Self {
        Self::Review {
            account_id,
            created_at,
            description,
            id,
            image_url,
            subtitle: Some(subtitle),
            title,
        }
    }

    pub fn review_with_title(
        account_id: String,
        created_at: String,
        description: Option<String>,
        id: String,
        image_url: Option<String>,
        subtitle: Option<String>,
        title: String,
    ) -> Self {
        Self::Review {
            account_id,
            created_at,
            description,
            id,
            image_url,
            subtitle,
            title: Some(title),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
