//! API client and types for the Whop API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    AccessTokensClient, AccountLinksClient, AccountsClient, AdCampaignsClient, AdGroupsClient,
    AdPixelsClient, AdsClient, AffiliatesClient, AiChatsClient, ApiKeysClient, ApiLogsClient,
    AppBuildsClient, AppInstallationsClient, AppsClient, AudiencesClient, AuthorizedUsersClient,
    AutomationEnrollmentsClient, AutomationStepsClient, AutomationsClient, BountiesClient,
    BountySubmissionsClient, CardTransactionsClient, CardsClient, CashbackRulesClient,
    ChannelMembersClient, ChannelsClient, ChatChannelsClient, CheckoutConfigurationsClient,
    ClaimLinksClient, CompanyTokenTransactionsClient, ConfirmationTokensClient,
    CourseChaptersClient, CourseLessonInteractionsClient, CourseLessonsClient,
    CourseStudentsClient, CoursesClient, DepositsClient, DisputeAlertsClient, DisputesClient,
    DmChannelsClient, DmMembersClient, DomainsClient, EconomicIntelligenceClient,
    EmailAddressesClient, EntriesClient, EventsClient, ExperiencesClient, ExperimentsClient,
    ExportsClient, ExternalAccountsClient, FeeMarkupsClient, FeedbackSubmissionsClient,
    FilesClient, FinancialActivityClient, FinancialReportsClient, FinancingApplicationsClient,
    ForumPostsClient, ForumsClient, IdentityProfilesClient, InvoicesClient, LeadsClient,
    LedgerAccountsClient, LedgersClient, LivestreamsClient, MediaClient, MembersClient,
    MembershipsClient, MessagesClient, NotificationsClient, PartnerReferralRequestsClient,
    PartnersClient, PaymentMethodDomainsClient, PaymentMethodsClient, PaymentQuotesClient,
    PaymentRulesClient, PaymentsClient, PayoutAccountsClient, PayoutMethodsClient, PayoutsClient,
    PeopleClient, PermissionsClient, PlansClient, ProductAffiliatesClient, ProductsClient,
    PromoCodesClient, ReactionsClient, RefundsClient, ResolutionCenterCasesClient,
    ReviewReportsClient, ReviewsClient, SearchClient, SetupIntentsClient, ShipmentsClient,
    SocialAccountsClient, StatsClient, SupportChannelsClient, SwapsClient, TeamMembersClient,
    TradesClient, TransfersClient, UsersClient, VariantsClient, VerificationsClient,
    WaitlistEntriesClient, WebhooksClient, Whop,
};
pub use types::*;
