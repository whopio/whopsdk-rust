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
    AdReportsClient, AdsClient, AffiliatesClient, AiChatsClient, ApiKeysClient, AppBuildsClient,
    AppsClient, AudiencesClient, AuthorizedUsersClient, BountiesClient, BountySubmissionsClient,
    CardTransactionsClient, CardsClient, ChatChannelsClient, CheckoutConfigurationsClient,
    CompaniesClient, CompanyTokenTransactionsClient, CourseChaptersClient,
    CourseLessonInteractionsClient, CourseLessonsClient, CourseStudentsClient, CoursesClient,
    DepositsClient, DisputeAlertsClient, DisputesClient, DmChannelsClient, DmMembersClient,
    EntriesClient, EventsClient, ExperiencesClient, ExportsClient, FeeMarkupsClient, FilesClient,
    FinancialActivityClient, FinancialReportsClient, ForumPostsClient, ForumsClient,
    IdentityProfilesClient, InvoicesClient, LeadsClient, LedgerAccountsClient, MediaClient,
    MembersClient, MembershipsClient, MessagesClient, NotificationsClient, PartnersClient,
    PaymentMethodDomainsClient, PaymentMethodsClient, PaymentsClient, PayoutAccountsClient,
    PayoutMethodsClient, PayoutsClient, PeopleClient, PermissionsClient, PlansClient,
    ProductsClient, PromoCodesClient, ReactionsClient, RecommendedActionsClient, RefundsClient,
    ResolutionCenterCasesClient, ReviewsClient, SetupIntentsClient, ShipmentsClient,
    SocialAccountsClient, StatsClient, SupportChannelsClient, SwapsClient, TeamMembersClient,
    TopupsClient, TransfersClient, UsersClient, VerificationsClient, WebhooksClient, Whop,
};
pub use types::*;
