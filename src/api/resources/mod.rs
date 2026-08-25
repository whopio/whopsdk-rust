//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **AccessTokens**
//! - **AccountLinks**
//! - **Accounts**
//! - **Ad Campaigns**
//! - **Ad Groups**
//! - **AdReports**
//! - **Ads**
//! - **Affiliates**
//! - **AiChats**
//! - **API Keys**
//! - **App Builds**
//! - **Apps**
//! - **Audiences**
//! - **AuthorizedUsers**
//! - **Bounties**
//! - **Bounty Submissions**
//! - **CardTransactions**
//! - **Cards**
//! - **ChatChannels**
//! - **Checkout Configurations**
//! - **Checkout Sessions**
//! - **Companies**
//! - **CompanyTokenTransactions**
//! - **CourseChapters**
//! - **CourseLessonInteractions**
//! - **CourseLessons**
//! - **CourseStudents**
//! - **Courses**
//! - **Deposits**
//! - **Dispute alerts**
//! - **Disputes**
//! - **DmChannels**
//! - **DmMembers**
//! - **Entries**
//! - **Events**
//! - **Experiences**
//! - **Exports**
//! - **FeeMarkups**
//! - **Files**
//! - **FinancialActivity**
//! - **Ledgers**
//! - **ForumPosts**
//! - **Forums**
//! - **IdentityProfiles**
//! - **Invoices**
//! - **Leads**
//! - **LedgerAccounts**
//! - **Media**
//! - **Members**
//! - **Memberships**
//! - **Messages**
//! - **Notifications**
//! - **Partners**
//! - **Payment Method Domains**
//! - **PaymentMethods**
//! - **Payments**
//! - **PayoutAccounts**
//! - **PayoutMethods**
//! - **Payouts**
//! - **People**
//! - **Permissions**
//! - **Plans**
//! - **Products**
//! - **Promo Codes**
//! - **Reactions**
//! - **Recommended Actions**
//! - **Refunds**
//! - **Resolution Center Cases**
//! - **Reviews**
//! - **Setup Intents**
//! - **Shipments**
//! - **Social Accounts**
//! - **Stats**
//! - **SupportChannels**
//! - **Swaps**
//! - **Team Members**
//! - **Topups**
//! - **Transfers**
//! - **Users**
//! - **Verifications**
//! - **Webhooks**
//! - **Withdrawals**

use crate::{ApiError, ClientConfig};

pub mod access_tokens;
pub mod account_links;
pub mod accounts;
pub mod ad_campaigns;
pub mod ad_groups;
pub mod ad_reports;
pub mod ads;
pub mod affiliates;
pub mod ai_chats;
pub mod api_keys;
pub mod app_builds;
pub mod apps;
pub mod audiences;
pub mod authorized_users;
pub mod bounties;
pub mod bounty_submissions;
pub mod card_transactions;
pub mod cards;
pub mod chat_channels;
pub mod checkout_configurations;
pub mod checkout_sessions;
pub mod companies;
pub mod company_token_transactions;
pub mod course_chapters;
pub mod course_lesson_interactions;
pub mod course_lessons;
pub mod course_students;
pub mod courses;
pub mod deposits;
pub mod dispute_alerts;
pub mod disputes;
pub mod dm_channels;
pub mod dm_members;
pub mod entries;
pub mod events;
pub mod experiences;
pub mod exports;
pub mod fee_markups;
pub mod files;
pub mod financial_activity;
pub mod forum_posts;
pub mod forums;
pub mod identity_profiles;
pub mod invoices;
pub mod leads;
pub mod ledger_accounts;
pub mod ledgers;
pub mod media;
pub mod members;
pub mod memberships;
pub mod messages;
pub mod notifications;
pub mod partners;
pub mod payment_method_domains;
pub mod payment_methods;
pub mod payments;
pub mod payout_accounts;
pub mod payout_methods;
pub mod payouts;
pub mod people;
pub mod permissions;
pub mod plans;
pub mod products;
pub mod promo_codes;
pub mod reactions;
pub mod recommended_actions;
pub mod refunds;
pub mod resolution_center_cases;
pub mod reviews;
pub mod setup_intents;
pub mod shipments;
pub mod social_accounts;
pub mod stats;
pub mod support_channels;
pub mod swaps;
pub mod team_members;
pub mod topups;
pub mod transfers;
pub mod users;
pub mod verifications;
pub mod webhooks;
pub mod withdrawals;
pub struct Whop {
    pub config: ClientConfig,
    pub access_tokens: AccessTokensClient,
    pub account_links: AccountLinksClient,
    pub accounts: AccountsClient,
    pub ad_campaigns: AdCampaignsClient,
    pub ad_groups: AdGroupsClient,
    pub ad_reports: AdReportsClient,
    pub ads: AdsClient,
    pub affiliates: AffiliatesClient,
    pub ai_chats: AiChatsClient,
    pub api_keys: ApiKeysClient,
    pub app_builds: AppBuildsClient,
    pub apps: AppsClient,
    pub audiences: AudiencesClient,
    pub authorized_users: AuthorizedUsersClient,
    pub bounties: BountiesClient,
    pub bounty_submissions: BountySubmissionsClient,
    pub card_transactions: CardTransactionsClient,
    pub cards: CardsClient,
    pub chat_channels: ChatChannelsClient,
    pub checkout_configurations: CheckoutConfigurationsClient,
    pub checkout_sessions: CheckoutSessionsClient,
    pub companies: CompaniesClient,
    pub company_token_transactions: CompanyTokenTransactionsClient,
    pub course_chapters: CourseChaptersClient,
    pub course_lesson_interactions: CourseLessonInteractionsClient,
    pub course_lessons: CourseLessonsClient,
    pub course_students: CourseStudentsClient,
    pub courses: CoursesClient,
    pub deposits: DepositsClient,
    pub dispute_alerts: DisputeAlertsClient,
    pub disputes: DisputesClient,
    pub dm_channels: DmChannelsClient,
    pub dm_members: DmMembersClient,
    pub entries: EntriesClient,
    pub events: EventsClient,
    pub experiences: ExperiencesClient,
    pub exports: ExportsClient,
    pub fee_markups: FeeMarkupsClient,
    pub files: FilesClient,
    pub financial_activity: FinancialActivityClient,
    pub ledgers: LedgersClient,
    pub forum_posts: ForumPostsClient,
    pub forums: ForumsClient,
    pub identity_profiles: IdentityProfilesClient,
    pub invoices: InvoicesClient,
    pub leads: LeadsClient,
    pub ledger_accounts: LedgerAccountsClient,
    pub media: MediaClient,
    pub members: MembersClient,
    pub memberships: MembershipsClient,
    pub messages: MessagesClient,
    pub notifications: NotificationsClient,
    pub partners: PartnersClient,
    pub payment_method_domains: PaymentMethodDomainsClient,
    pub payment_methods: PaymentMethodsClient,
    pub payments: PaymentsClient,
    pub payout_accounts: PayoutAccountsClient,
    pub payout_methods: PayoutMethodsClient,
    pub payouts: PayoutsClient,
    pub people: PeopleClient,
    pub permissions: PermissionsClient,
    pub plans: PlansClient,
    pub products: ProductsClient,
    pub promo_codes: PromoCodesClient,
    pub reactions: ReactionsClient,
    pub recommended_actions: RecommendedActionsClient,
    pub refunds: RefundsClient,
    pub resolution_center_cases: ResolutionCenterCasesClient,
    pub reviews: ReviewsClient,
    pub setup_intents: SetupIntentsClient,
    pub shipments: ShipmentsClient,
    pub social_accounts: SocialAccountsClient,
    pub stats: StatsClient,
    pub support_channels: SupportChannelsClient,
    pub swaps: SwapsClient,
    pub team_members: TeamMembersClient,
    pub topups: TopupsClient,
    pub transfers: TransfersClient,
    pub users: UsersClient,
    pub verifications: VerificationsClient,
    pub webhooks: WebhooksClient,
    pub withdrawals: WithdrawalsClient,
}

impl Whop {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            access_tokens: AccessTokensClient::new(config.clone())?,
            account_links: AccountLinksClient::new(config.clone())?,
            accounts: AccountsClient::new(config.clone())?,
            ad_campaigns: AdCampaignsClient::new(config.clone())?,
            ad_groups: AdGroupsClient::new(config.clone())?,
            ad_reports: AdReportsClient::new(config.clone())?,
            ads: AdsClient::new(config.clone())?,
            affiliates: AffiliatesClient::new(config.clone())?,
            ai_chats: AiChatsClient::new(config.clone())?,
            api_keys: ApiKeysClient::new(config.clone())?,
            app_builds: AppBuildsClient::new(config.clone())?,
            apps: AppsClient::new(config.clone())?,
            audiences: AudiencesClient::new(config.clone())?,
            authorized_users: AuthorizedUsersClient::new(config.clone())?,
            bounties: BountiesClient::new(config.clone())?,
            bounty_submissions: BountySubmissionsClient::new(config.clone())?,
            card_transactions: CardTransactionsClient::new(config.clone())?,
            cards: CardsClient::new(config.clone())?,
            chat_channels: ChatChannelsClient::new(config.clone())?,
            checkout_configurations: CheckoutConfigurationsClient::new(config.clone())?,
            checkout_sessions: CheckoutSessionsClient::new(config.clone())?,
            companies: CompaniesClient::new(config.clone())?,
            company_token_transactions: CompanyTokenTransactionsClient::new(config.clone())?,
            course_chapters: CourseChaptersClient::new(config.clone())?,
            course_lesson_interactions: CourseLessonInteractionsClient::new(config.clone())?,
            course_lessons: CourseLessonsClient::new(config.clone())?,
            course_students: CourseStudentsClient::new(config.clone())?,
            courses: CoursesClient::new(config.clone())?,
            deposits: DepositsClient::new(config.clone())?,
            dispute_alerts: DisputeAlertsClient::new(config.clone())?,
            disputes: DisputesClient::new(config.clone())?,
            dm_channels: DmChannelsClient::new(config.clone())?,
            dm_members: DmMembersClient::new(config.clone())?,
            entries: EntriesClient::new(config.clone())?,
            events: EventsClient::new(config.clone())?,
            experiences: ExperiencesClient::new(config.clone())?,
            exports: ExportsClient::new(config.clone())?,
            fee_markups: FeeMarkupsClient::new(config.clone())?,
            files: FilesClient::new(config.clone())?,
            financial_activity: FinancialActivityClient::new(config.clone())?,
            ledgers: LedgersClient::new(config.clone())?,
            forum_posts: ForumPostsClient::new(config.clone())?,
            forums: ForumsClient::new(config.clone())?,
            identity_profiles: IdentityProfilesClient::new(config.clone())?,
            invoices: InvoicesClient::new(config.clone())?,
            leads: LeadsClient::new(config.clone())?,
            ledger_accounts: LedgerAccountsClient::new(config.clone())?,
            media: MediaClient::new(config.clone())?,
            members: MembersClient::new(config.clone())?,
            memberships: MembershipsClient::new(config.clone())?,
            messages: MessagesClient::new(config.clone())?,
            notifications: NotificationsClient::new(config.clone())?,
            partners: PartnersClient::new(config.clone())?,
            payment_method_domains: PaymentMethodDomainsClient::new(config.clone())?,
            payment_methods: PaymentMethodsClient::new(config.clone())?,
            payments: PaymentsClient::new(config.clone())?,
            payout_accounts: PayoutAccountsClient::new(config.clone())?,
            payout_methods: PayoutMethodsClient::new(config.clone())?,
            payouts: PayoutsClient::new(config.clone())?,
            people: PeopleClient::new(config.clone())?,
            permissions: PermissionsClient::new(config.clone())?,
            plans: PlansClient::new(config.clone())?,
            products: ProductsClient::new(config.clone())?,
            promo_codes: PromoCodesClient::new(config.clone())?,
            reactions: ReactionsClient::new(config.clone())?,
            recommended_actions: RecommendedActionsClient::new(config.clone())?,
            refunds: RefundsClient::new(config.clone())?,
            resolution_center_cases: ResolutionCenterCasesClient::new(config.clone())?,
            reviews: ReviewsClient::new(config.clone())?,
            setup_intents: SetupIntentsClient::new(config.clone())?,
            shipments: ShipmentsClient::new(config.clone())?,
            social_accounts: SocialAccountsClient::new(config.clone())?,
            stats: StatsClient::new(config.clone())?,
            support_channels: SupportChannelsClient::new(config.clone())?,
            swaps: SwapsClient::new(config.clone())?,
            team_members: TeamMembersClient::new(config.clone())?,
            topups: TopupsClient::new(config.clone())?,
            transfers: TransfersClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            verifications: VerificationsClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
            withdrawals: WithdrawalsClient::new(config.clone())?,
        })
    }
}

pub use access_tokens::AccessTokensClient;
pub use account_links::AccountLinksClient;
pub use accounts::AccountsClient;
pub use ad_campaigns::AdCampaignsClient;
pub use ad_groups::AdGroupsClient;
pub use ad_reports::AdReportsClient;
pub use ads::AdsClient;
pub use affiliates::AffiliatesClient;
pub use ai_chats::AiChatsClient;
pub use api_keys::ApiKeysClient;
pub use app_builds::AppBuildsClient;
pub use apps::AppsClient;
pub use audiences::AudiencesClient;
pub use authorized_users::AuthorizedUsersClient;
pub use bounties::BountiesClient;
pub use bounty_submissions::BountySubmissionsClient;
pub use card_transactions::CardTransactionsClient;
pub use cards::CardsClient;
pub use chat_channels::ChatChannelsClient;
pub use checkout_configurations::CheckoutConfigurationsClient;
pub use checkout_sessions::CheckoutSessionsClient;
pub use companies::CompaniesClient;
pub use company_token_transactions::CompanyTokenTransactionsClient;
pub use course_chapters::CourseChaptersClient;
pub use course_lesson_interactions::CourseLessonInteractionsClient;
pub use course_lessons::CourseLessonsClient;
pub use course_students::CourseStudentsClient;
pub use courses::CoursesClient;
pub use deposits::DepositsClient;
pub use dispute_alerts::DisputeAlertsClient;
pub use disputes::DisputesClient;
pub use dm_channels::DmChannelsClient;
pub use dm_members::DmMembersClient;
pub use entries::EntriesClient;
pub use events::EventsClient;
pub use experiences::ExperiencesClient;
pub use exports::ExportsClient;
pub use fee_markups::FeeMarkupsClient;
pub use files::FilesClient;
pub use financial_activity::FinancialActivityClient;
pub use forum_posts::ForumPostsClient;
pub use forums::ForumsClient;
pub use identity_profiles::IdentityProfilesClient;
pub use invoices::InvoicesClient;
pub use leads::LeadsClient;
pub use ledger_accounts::LedgerAccountsClient;
pub use ledgers::LedgersClient;
pub use media::MediaClient;
pub use members::MembersClient;
pub use memberships::MembershipsClient;
pub use messages::MessagesClient;
pub use notifications::NotificationsClient;
pub use partners::PartnersClient;
pub use payment_method_domains::PaymentMethodDomainsClient;
pub use payment_methods::PaymentMethodsClient;
pub use payments::PaymentsClient;
pub use payout_accounts::PayoutAccountsClient;
pub use payout_methods::PayoutMethodsClient;
pub use payouts::PayoutsClient;
pub use people::PeopleClient;
pub use permissions::PermissionsClient;
pub use plans::PlansClient;
pub use products::ProductsClient;
pub use promo_codes::PromoCodesClient;
pub use reactions::ReactionsClient;
pub use recommended_actions::RecommendedActionsClient;
pub use refunds::RefundsClient;
pub use resolution_center_cases::ResolutionCenterCasesClient;
pub use reviews::ReviewsClient;
pub use setup_intents::SetupIntentsClient;
pub use shipments::ShipmentsClient;
pub use social_accounts::SocialAccountsClient;
pub use stats::StatsClient;
pub use support_channels::SupportChannelsClient;
pub use swaps::SwapsClient;
pub use team_members::TeamMembersClient;
pub use topups::TopupsClient;
pub use transfers::TransfersClient;
pub use users::UsersClient;
pub use verifications::VerificationsClient;
pub use webhooks::WebhooksClient;
pub use withdrawals::WithdrawalsClient;
