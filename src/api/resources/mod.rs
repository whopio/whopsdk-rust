//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **AccessTokens**
//! - **AccountLinks**
//! - **Accounts**
//! - **Ad Campaigns**
//! - **Ad Groups**
//! - **Ads**
//! - **Ad Pixels**
//! - **Affiliates**
//! - **AiChats**
//! - **API Keys**
//! - **Api Logs**
//! - **App Builds**
//! - **App Installations**
//! - **Apps**
//! - **Audiences**
//! - **AuthorizedUsers**
//! - **Automation Enrollments**
//! - **Automation Steps**
//! - **Automations**
//! - **Bounties**
//! - **Bounty Submissions**
//! - **CardTransactions**
//! - **Cards**
//! - **Cashback Rules**
//! - **Channel Members**
//! - **Channels**
//! - **ChatChannels**
//! - **Checkout Configurations**
//! - **ClaimLinks**
//! - **CompanyTokenTransactions**
//! - **Confirmation Tokens**
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
//! - **Domains**
//! - **Economic Intelligence**
//! - **Email addresses**
//! - **Events**
//! - **Experiences**
//! - **Experiments**
//! - **Exports**
//! - **External Accounts**
//! - **FeeMarkups**
//! - **FeedbackSubmissions**
//! - **Files**
//! - **FinancialActivity**
//! - **FinancialReports**
//! - **Financing Applications**
//! - **ForumPosts**
//! - **Forums**
//! - **IdentityProfiles**
//! - **Invoices**
//! - **Leads**
//! - **LedgerAccounts**
//! - **Livestreams**
//! - **Media**
//! - **Members**
//! - **Memberships**
//! - **Messages**
//! - **Notifications**
//! - **Partner Referral Requests**
//! - **Partners**
//! - **Payment Method Domains**
//! - **PaymentMethods**
//! - **Payment Quotes**
//! - **Payment Rules**
//! - **Payments**
//! - **PayoutAccounts**
//! - **PayoutMethods**
//! - **Payouts**
//! - **People**
//! - **Permissions**
//! - **Plans**
//! - **Product Affiliates**
//! - **Products**
//! - **Promo Codes**
//! - **Reactions**
//! - **Refunds**
//! - **Resolution Center Cases**
//! - **Review reports**
//! - **Reviews**
//! - **Search**
//! - **Setup Intents**
//! - **Shipments**
//! - **Social Accounts**
//! - **Stats**
//! - **SupportChannels**
//! - **Swaps**
//! - **Team Members**
//! - **Trades**
//! - **Transfers**
//! - **Users**
//! - **Variants**
//! - **Verifications**
//! - **Waitlist Entries**
//! - **Webhooks**
//! - **Entries**
//! - **Ledgers**

use crate::{ApiError, ClientConfig};

pub mod access_tokens;
pub mod account_links;
pub mod accounts;
pub mod ad_campaigns;
pub mod ad_groups;
pub mod ad_pixels;
pub mod ads;
pub mod affiliates;
pub mod ai_chats;
pub mod api_keys;
pub mod api_logs;
pub mod app_builds;
pub mod app_installations;
pub mod apps;
pub mod audiences;
pub mod authorized_users;
pub mod automation_enrollments;
pub mod automation_steps;
pub mod automations;
pub mod bounties;
pub mod bounty_submissions;
pub mod card_transactions;
pub mod cards;
pub mod cashback_rules;
pub mod channel_members;
pub mod channels;
pub mod chat_channels;
pub mod checkout_configurations;
pub mod claim_links;
pub mod company_token_transactions;
pub mod confirmation_tokens;
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
pub mod domains;
pub mod economic_intelligence;
pub mod email_addresses;
pub mod entries;
pub mod events;
pub mod experiences;
pub mod experiments;
pub mod exports;
pub mod external_accounts;
pub mod fee_markups;
pub mod feedback_submissions;
pub mod files;
pub mod financial_activity;
pub mod financial_reports;
pub mod financing_applications;
pub mod forum_posts;
pub mod forums;
pub mod identity_profiles;
pub mod invoices;
pub mod leads;
pub mod ledger_accounts;
pub mod ledgers;
pub mod livestreams;
pub mod media;
pub mod members;
pub mod memberships;
pub mod messages;
pub mod notifications;
pub mod partner_referral_requests;
pub mod partners;
pub mod payment_method_domains;
pub mod payment_methods;
pub mod payment_quotes;
pub mod payment_rules;
pub mod payments;
pub mod payout_accounts;
pub mod payout_methods;
pub mod payouts;
pub mod people;
pub mod permissions;
pub mod plans;
pub mod product_affiliates;
pub mod products;
pub mod promo_codes;
pub mod reactions;
pub mod refunds;
pub mod resolution_center_cases;
pub mod review_reports;
pub mod reviews;
pub mod search;
pub mod setup_intents;
pub mod shipments;
pub mod social_accounts;
pub mod stats;
pub mod support_channels;
pub mod swaps;
pub mod team_members;
pub mod trades;
pub mod transfers;
pub mod users;
pub mod variants;
pub mod verifications;
pub mod waitlist_entries;
pub mod webhooks;
pub struct Whop {
    pub config: ClientConfig,
    pub access_tokens: AccessTokensClient,
    pub account_links: AccountLinksClient,
    pub accounts: AccountsClient,
    pub ad_campaigns: AdCampaignsClient,
    pub ad_groups: AdGroupsClient,
    pub ads: AdsClient,
    pub ad_pixels: AdPixelsClient,
    pub affiliates: AffiliatesClient,
    pub ai_chats: AiChatsClient,
    pub api_keys: ApiKeysClient,
    pub api_logs: ApiLogsClient,
    pub app_builds: AppBuildsClient,
    pub app_installations: AppInstallationsClient,
    pub apps: AppsClient,
    pub audiences: AudiencesClient,
    pub authorized_users: AuthorizedUsersClient,
    pub automation_enrollments: AutomationEnrollmentsClient,
    pub automation_steps: AutomationStepsClient,
    pub automations: AutomationsClient,
    pub bounties: BountiesClient,
    pub bounty_submissions: BountySubmissionsClient,
    pub card_transactions: CardTransactionsClient,
    pub cards: CardsClient,
    pub cashback_rules: CashbackRulesClient,
    pub channel_members: ChannelMembersClient,
    pub channels: ChannelsClient,
    pub chat_channels: ChatChannelsClient,
    pub checkout_configurations: CheckoutConfigurationsClient,
    pub claim_links: ClaimLinksClient,
    pub company_token_transactions: CompanyTokenTransactionsClient,
    pub confirmation_tokens: ConfirmationTokensClient,
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
    pub domains: DomainsClient,
    pub economic_intelligence: EconomicIntelligenceClient,
    pub email_addresses: EmailAddressesClient,
    pub events: EventsClient,
    pub experiences: ExperiencesClient,
    pub experiments: ExperimentsClient,
    pub exports: ExportsClient,
    pub external_accounts: ExternalAccountsClient,
    pub fee_markups: FeeMarkupsClient,
    pub feedback_submissions: FeedbackSubmissionsClient,
    pub files: FilesClient,
    pub financial_activity: FinancialActivityClient,
    pub financial_reports: FinancialReportsClient,
    pub financing_applications: FinancingApplicationsClient,
    pub forum_posts: ForumPostsClient,
    pub forums: ForumsClient,
    pub identity_profiles: IdentityProfilesClient,
    pub invoices: InvoicesClient,
    pub leads: LeadsClient,
    pub ledger_accounts: LedgerAccountsClient,
    pub livestreams: LivestreamsClient,
    pub media: MediaClient,
    pub members: MembersClient,
    pub memberships: MembershipsClient,
    pub messages: MessagesClient,
    pub notifications: NotificationsClient,
    pub partner_referral_requests: PartnerReferralRequestsClient,
    pub partners: PartnersClient,
    pub payment_method_domains: PaymentMethodDomainsClient,
    pub payment_methods: PaymentMethodsClient,
    pub payment_quotes: PaymentQuotesClient,
    pub payment_rules: PaymentRulesClient,
    pub payments: PaymentsClient,
    pub payout_accounts: PayoutAccountsClient,
    pub payout_methods: PayoutMethodsClient,
    pub payouts: PayoutsClient,
    pub people: PeopleClient,
    pub permissions: PermissionsClient,
    pub plans: PlansClient,
    pub product_affiliates: ProductAffiliatesClient,
    pub products: ProductsClient,
    pub promo_codes: PromoCodesClient,
    pub reactions: ReactionsClient,
    pub refunds: RefundsClient,
    pub resolution_center_cases: ResolutionCenterCasesClient,
    pub review_reports: ReviewReportsClient,
    pub reviews: ReviewsClient,
    pub search: SearchClient,
    pub setup_intents: SetupIntentsClient,
    pub shipments: ShipmentsClient,
    pub social_accounts: SocialAccountsClient,
    pub stats: StatsClient,
    pub support_channels: SupportChannelsClient,
    pub swaps: SwapsClient,
    pub team_members: TeamMembersClient,
    pub trades: TradesClient,
    pub transfers: TransfersClient,
    pub users: UsersClient,
    pub variants: VariantsClient,
    pub verifications: VerificationsClient,
    pub waitlist_entries: WaitlistEntriesClient,
    pub webhooks: WebhooksClient,
}

impl Whop {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            access_tokens: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AccessTokensClient::new(cfg)?
            },
            account_links: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AccountLinksClient::new(cfg)?
            },
            accounts: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AccountsClient::new(cfg)?
            },
            ad_campaigns: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AdCampaignsClient::new(cfg)?
            },
            ad_groups: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AdGroupsClient::new(cfg)?
            },
            ads: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AdsClient::new(cfg)?
            },
            ad_pixels: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AdPixelsClient::new(cfg)?
            },
            affiliates: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AffiliatesClient::new(cfg)?
            },
            ai_chats: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AiChatsClient::new(cfg)?
            },
            api_keys: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ApiKeysClient::new(cfg)?
            },
            api_logs: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ApiLogsClient::new(cfg)?
            },
            app_builds: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AppBuildsClient::new(cfg)?
            },
            app_installations: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AppInstallationsClient::new(cfg)?
            },
            apps: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AppsClient::new(cfg)?
            },
            audiences: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AudiencesClient::new(cfg)?
            },
            authorized_users: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AuthorizedUsersClient::new(cfg)?
            },
            automation_enrollments: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AutomationEnrollmentsClient::new(cfg)?
            },
            automation_steps: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AutomationStepsClient::new(cfg)?
            },
            automations: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                AutomationsClient::new(cfg)?
            },
            bounties: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                BountiesClient::new(cfg)?
            },
            bounty_submissions: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                BountySubmissionsClient::new(cfg)?
            },
            card_transactions: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                CardTransactionsClient::new(cfg)?
            },
            cards: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                CardsClient::new(cfg)?
            },
            cashback_rules: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                CashbackRulesClient::new(cfg)?
            },
            channel_members: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ChannelMembersClient::new(cfg)?
            },
            channels: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ChannelsClient::new(cfg)?
            },
            chat_channels: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ChatChannelsClient::new(cfg)?
            },
            checkout_configurations: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                CheckoutConfigurationsClient::new(cfg)?
            },
            claim_links: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ClaimLinksClient::new(cfg)?
            },
            company_token_transactions: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                CompanyTokenTransactionsClient::new(cfg)?
            },
            confirmation_tokens: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ConfirmationTokensClient::new(cfg)?
            },
            course_chapters: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                CourseChaptersClient::new(cfg)?
            },
            course_lesson_interactions: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                CourseLessonInteractionsClient::new(cfg)?
            },
            course_lessons: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                CourseLessonsClient::new(cfg)?
            },
            course_students: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                CourseStudentsClient::new(cfg)?
            },
            courses: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                CoursesClient::new(cfg)?
            },
            deposits: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                DepositsClient::new(cfg)?
            },
            dispute_alerts: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                DisputeAlertsClient::new(cfg)?
            },
            disputes: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                DisputesClient::new(cfg)?
            },
            dm_channels: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                DmChannelsClient::new(cfg)?
            },
            dm_members: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                DmMembersClient::new(cfg)?
            },
            domains: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                DomainsClient::new(cfg)?
            },
            economic_intelligence: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                EconomicIntelligenceClient::new(cfg)?
            },
            email_addresses: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                EmailAddressesClient::new(cfg)?
            },
            events: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                EventsClient::new(cfg)?
            },
            experiences: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ExperiencesClient::new(cfg)?
            },
            experiments: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ExperimentsClient::new(cfg)?
            },
            exports: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ExportsClient::new(cfg)?
            },
            external_accounts: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ExternalAccountsClient::new(cfg)?
            },
            fee_markups: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                FeeMarkupsClient::new(cfg)?
            },
            feedback_submissions: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                FeedbackSubmissionsClient::new(cfg)?
            },
            files: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                FilesClient::new(cfg)?
            },
            financial_activity: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                FinancialActivityClient::new(cfg)?
            },
            financial_reports: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                FinancialReportsClient::new(cfg)?
            },
            financing_applications: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                FinancingApplicationsClient::new(cfg)?
            },
            forum_posts: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ForumPostsClient::new(cfg)?
            },
            forums: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ForumsClient::new(cfg)?
            },
            identity_profiles: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                IdentityProfilesClient::new(cfg)?
            },
            invoices: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                InvoicesClient::new(cfg)?
            },
            leads: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                LeadsClient::new(cfg)?
            },
            ledger_accounts: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                LedgerAccountsClient::new(cfg)?
            },
            livestreams: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                LivestreamsClient::new(cfg)?
            },
            media: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                MediaClient::new(cfg)?
            },
            members: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                MembersClient::new(cfg)?
            },
            memberships: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                MembershipsClient::new(cfg)?
            },
            messages: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                MessagesClient::new(cfg)?
            },
            notifications: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                NotificationsClient::new(cfg)?
            },
            partner_referral_requests: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PartnerReferralRequestsClient::new(cfg)?
            },
            partners: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PartnersClient::new(cfg)?
            },
            payment_method_domains: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PaymentMethodDomainsClient::new(cfg)?
            },
            payment_methods: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PaymentMethodsClient::new(cfg)?
            },
            payment_quotes: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PaymentQuotesClient::new(cfg)?
            },
            payment_rules: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PaymentRulesClient::new(cfg)?
            },
            payments: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PaymentsClient::new(cfg)?
            },
            payout_accounts: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PayoutAccountsClient::new(cfg)?
            },
            payout_methods: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PayoutMethodsClient::new(cfg)?
            },
            payouts: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PayoutsClient::new(cfg)?
            },
            people: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PeopleClient::new(cfg)?
            },
            permissions: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PermissionsClient::new(cfg)?
            },
            plans: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PlansClient::new(cfg)?
            },
            product_affiliates: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ProductAffiliatesClient::new(cfg)?
            },
            products: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ProductsClient::new(cfg)?
            },
            promo_codes: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                PromoCodesClient::new(cfg)?
            },
            reactions: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ReactionsClient::new(cfg)?
            },
            refunds: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                RefundsClient::new(cfg)?
            },
            resolution_center_cases: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ResolutionCenterCasesClient::new(cfg)?
            },
            review_reports: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ReviewReportsClient::new(cfg)?
            },
            reviews: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ReviewsClient::new(cfg)?
            },
            search: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                SearchClient::new(cfg)?
            },
            setup_intents: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                SetupIntentsClient::new(cfg)?
            },
            shipments: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                ShipmentsClient::new(cfg)?
            },
            social_accounts: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                SocialAccountsClient::new(cfg)?
            },
            stats: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                StatsClient::new(cfg)?
            },
            support_channels: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                SupportChannelsClient::new(cfg)?
            },
            swaps: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                SwapsClient::new(cfg)?
            },
            team_members: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                TeamMembersClient::new(cfg)?
            },
            trades: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                TradesClient::new(cfg)?
            },
            transfers: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                TransfersClient::new(cfg)?
            },
            users: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                UsersClient::new(cfg)?
            },
            variants: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                VariantsClient::new(cfg)?
            },
            verifications: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                VerificationsClient::new(cfg)?
            },
            waitlist_entries: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                WaitlistEntriesClient::new(cfg)?
            },
            webhooks: {
                let mut cfg = config.clone();
                cfg.base_url = cfg
                    .service_url(|environment| environment.api_url())
                    .to_string();
                WebhooksClient::new(cfg)?
            },
        })
    }
}

pub use access_tokens::AccessTokensClient;
pub use account_links::AccountLinksClient;
pub use accounts::AccountsClient;
pub use ad_campaigns::AdCampaignsClient;
pub use ad_groups::AdGroupsClient;
pub use ad_pixels::AdPixelsClient;
pub use ads::AdsClient;
pub use affiliates::AffiliatesClient;
pub use ai_chats::AiChatsClient;
pub use api_keys::ApiKeysClient;
pub use api_logs::ApiLogsClient;
pub use app_builds::AppBuildsClient;
pub use app_installations::AppInstallationsClient;
pub use apps::AppsClient;
pub use audiences::AudiencesClient;
pub use authorized_users::AuthorizedUsersClient;
pub use automation_enrollments::AutomationEnrollmentsClient;
pub use automation_steps::AutomationStepsClient;
pub use automations::AutomationsClient;
pub use bounties::BountiesClient;
pub use bounty_submissions::BountySubmissionsClient;
pub use card_transactions::CardTransactionsClient;
pub use cards::CardsClient;
pub use cashback_rules::CashbackRulesClient;
pub use channel_members::ChannelMembersClient;
pub use channels::ChannelsClient;
pub use chat_channels::ChatChannelsClient;
pub use checkout_configurations::CheckoutConfigurationsClient;
pub use claim_links::ClaimLinksClient;
pub use company_token_transactions::CompanyTokenTransactionsClient;
pub use confirmation_tokens::ConfirmationTokensClient;
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
pub use domains::DomainsClient;
pub use economic_intelligence::EconomicIntelligenceClient;
pub use email_addresses::EmailAddressesClient;
pub use entries::EntriesClient;
pub use events::EventsClient;
pub use experiences::ExperiencesClient;
pub use experiments::ExperimentsClient;
pub use exports::ExportsClient;
pub use external_accounts::ExternalAccountsClient;
pub use fee_markups::FeeMarkupsClient;
pub use feedback_submissions::FeedbackSubmissionsClient;
pub use files::FilesClient;
pub use financial_activity::FinancialActivityClient;
pub use financial_reports::FinancialReportsClient;
pub use financing_applications::FinancingApplicationsClient;
pub use forum_posts::ForumPostsClient;
pub use forums::ForumsClient;
pub use identity_profiles::IdentityProfilesClient;
pub use invoices::InvoicesClient;
pub use leads::LeadsClient;
pub use ledger_accounts::LedgerAccountsClient;
pub use ledgers::LedgersClient;
pub use livestreams::LivestreamsClient;
pub use media::MediaClient;
pub use members::MembersClient;
pub use memberships::MembershipsClient;
pub use messages::MessagesClient;
pub use notifications::NotificationsClient;
pub use partner_referral_requests::PartnerReferralRequestsClient;
pub use partners::PartnersClient;
pub use payment_method_domains::PaymentMethodDomainsClient;
pub use payment_methods::PaymentMethodsClient;
pub use payment_quotes::PaymentQuotesClient;
pub use payment_rules::PaymentRulesClient;
pub use payments::PaymentsClient;
pub use payout_accounts::PayoutAccountsClient;
pub use payout_methods::PayoutMethodsClient;
pub use payouts::PayoutsClient;
pub use people::PeopleClient;
pub use permissions::PermissionsClient;
pub use plans::PlansClient;
pub use product_affiliates::ProductAffiliatesClient;
pub use products::ProductsClient;
pub use promo_codes::PromoCodesClient;
pub use reactions::ReactionsClient;
pub use refunds::RefundsClient;
pub use resolution_center_cases::ResolutionCenterCasesClient;
pub use review_reports::ReviewReportsClient;
pub use reviews::ReviewsClient;
pub use search::SearchClient;
pub use setup_intents::SetupIntentsClient;
pub use shipments::ShipmentsClient;
pub use social_accounts::SocialAccountsClient;
pub use stats::StatsClient;
pub use support_channels::SupportChannelsClient;
pub use swaps::SwapsClient;
pub use team_members::TeamMembersClient;
pub use trades::TradesClient;
pub use transfers::TransfersClient;
pub use users::UsersClient;
pub use variants::VariantsClient;
pub use verifications::VerificationsClient;
pub use waitlist_entries::WaitlistEntriesClient;
pub use webhooks::WebhooksClient;
