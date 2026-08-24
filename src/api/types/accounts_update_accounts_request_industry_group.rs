pub use crate::prelude::*;

/// Account industry group. See the [business types and industries glossary](/api-reference/beta/accounts/account#business-types-and-industries-glossary) for valid values.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAccountsRequestIndustryGroup {
    AcademicAndTestPrep,
    Accessories,
    AgricultureAndFarming,
    AiAndAutomationAgencies,
    AiAndAutomationSoftware,
    ArtsAndCrafts,
    Automotive,
    B2BAndProfessionalMarketplaces,
    BabyAndKids,
    BarsAndBreweries,
    BeautyAndPersonalCare,
    BeautyAndWellness,
    BusinessAndEntrepreneurship,
    BusinessAndMoneyGroups,
    CafesAndQuickService,
    CareerAndProfessional,
    CharityAndCauseEvents,
    ClassActionSettlement,
    ClothingAndApparel,
    CommunicationAndMessagingSoftware,
    CommunityAndEducationSoftware,
    ConferenceAndExpoEvents,
    Consulting,
    ContentAndClippingAgencies,
    CreativeAndContentCreation,
    CreativeAndContentGroups,
    CreativeAndEducation,
    CreativeGigs,
    CreativeServices,
    CustomerSupportAgencies,
    DatingAndRelationships,
    DeliveryAndLogistics,
    DentalAndVision,
    DermatologyAndSkin,
    DesignAndCreativeAgencies,
    DeveloperAndTechnicalTools,
    DevelopmentAgencies,
    DigitalAndEducationMarketplaces,
    DigitalGoodsAndAccounts,
    ECommerceSoftware,
    EducationAndChildcare,
    EducationalTrainingEvents,
    ElectronicsAndGadgets,
    EntertainmentAndLeisure,
    FamilyAndCommunityEvents,
    FinanceAndInvesting,
    FitnessAndAthletics,
    FitnessAndHealthGroups,
    FitnessAndRecreation,
    FitnessEquipmentAndGear,
    FoodAndBeverages,
    FoodAndHospitalityMarketplaces,
    FuneralAndDeathCare,
    GamingAndEntertainmentSoftware,
    GamingGroups,
    GeneticAndSpecialized,
    GovernmentAndPublic,
    HealthAndWellness,
    HealthAndWellnessServices,
    Healthcare,
    HealthcareAndWellnessSoftware,
    HobbiesAndLifestyle,
    HobbyAndInterestGroups,
    HomeAndLiving,
    HomeAndTradeServices,
    HomeAndTradeStorefronts,
    HomeImprovementAndTools,
    HomeServicesGigs,
    HospitalityAndLodging,
    IndustrialAndManufacturing,
    IndustrySpecificSoftware,
    LanguageAndCommunication,
    LegalAndCompliance,
    LifestyleAndCulture,
    LifestyleAndPersonalGrowth,
    LifestyleAndPersonalGrowthGroups,
    LifestyleAndWellnessEvents,
    LogisticsAndTransportationServices,
    MarketingAgencies,
    MarketingAndAdvertising,
    MarketingAndSalesSoftware,
    MediaAndPublishingCompanies,
    MentalHealthAndBehavioral,
    Miscellaneous,
    MusicAndPerformingArts,
    NewsAndPolitics,
    NonprofitAndCharity,
    OfficeAndBusinessSupplies,
    OutdoorAndSports,
    PerformanceAndShowEvents,
    PersonalDevelopment,
    PersonalFinance,
    PersonalServices,
    PetServices,
    PetsAndAnimals,
    PrimaryAndGeneralCare,
    ProductMarketplaces,
    ProductivityAndBusinessOps,
    ProfessionalGigs,
    ProfessionalServices,
    ProfessionalServicesStorefront,
    PublishingAndInfoProducts,
    RealEstate,
    RealEstateSoftware,
    RecruitingAndStaffing,
    RehabilitationAndTherapy,
    ReligionAndFaith,
    RentalMarketplaces,
    Restaurants,
    Retail,
    SalesAgencies,
    SalesAndRevenue,
    SecurityAndInvestigations,
    SecurityAndPrivacySoftware,
    ServiceMarketplaces,
    SleepAndChronicConditions,
    SocialAndNetworkingEvents,
    SocialEntertainmentEvents,
    SpecializedGigs,
    SpecialtyMedicalCare,
    SpiritualityAndMindfulness,
    SpiritualityAndPersonalGrowth,
    SportsAndFitnessEvents,
    SportsBettingAndGambling,
    SportsBettingGroups,
    SupplementsAndNutrition,
    SustainabilityAndEcoProducts,
    TaskAndErrands,
    TechAndAi,
    TechAndDevGroups,
    TechAndDevelopment,
    TradingAndFinanceSoftware,
    TradingAndInvesting,
    TradingAndInvestingGroups,
    Transportation,
    Veterinary,
    VideoGamesAndEsports,
    WeightAndMetabolicHealth,
    WellnessAndAlternative,
    WomensAndMensHealth,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAccountsRequestIndustryGroup {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AcademicAndTestPrep => serializer.serialize_str("academic_and_test_prep"),
            Self::Accessories => serializer.serialize_str("accessories"),
            Self::AgricultureAndFarming => serializer.serialize_str("agriculture_and_farming"),
            Self::AiAndAutomationAgencies => serializer.serialize_str("ai_and_automation_agencies"),
            Self::AiAndAutomationSoftware => serializer.serialize_str("ai_and_automation_software"),
            Self::ArtsAndCrafts => serializer.serialize_str("arts_and_crafts"),
            Self::Automotive => serializer.serialize_str("automotive"),
            Self::B2BAndProfessionalMarketplaces => {
                serializer.serialize_str("b2b_and_professional_marketplaces")
            }
            Self::BabyAndKids => serializer.serialize_str("baby_and_kids"),
            Self::BarsAndBreweries => serializer.serialize_str("bars_and_breweries"),
            Self::BeautyAndPersonalCare => serializer.serialize_str("beauty_and_personal_care"),
            Self::BeautyAndWellness => serializer.serialize_str("beauty_and_wellness"),
            Self::BusinessAndEntrepreneurship => {
                serializer.serialize_str("business_and_entrepreneurship")
            }
            Self::BusinessAndMoneyGroups => serializer.serialize_str("business_and_money_groups"),
            Self::CafesAndQuickService => serializer.serialize_str("cafes_and_quick_service"),
            Self::CareerAndProfessional => serializer.serialize_str("career_and_professional"),
            Self::CharityAndCauseEvents => serializer.serialize_str("charity_and_cause_events"),
            Self::ClassActionSettlement => serializer.serialize_str("class_action_settlement"),
            Self::ClothingAndApparel => serializer.serialize_str("clothing_and_apparel"),
            Self::CommunicationAndMessagingSoftware => {
                serializer.serialize_str("communication_and_messaging_software")
            }
            Self::CommunityAndEducationSoftware => {
                serializer.serialize_str("community_and_education_software")
            }
            Self::ConferenceAndExpoEvents => serializer.serialize_str("conference_and_expo_events"),
            Self::Consulting => serializer.serialize_str("consulting"),
            Self::ContentAndClippingAgencies => {
                serializer.serialize_str("content_and_clipping_agencies")
            }
            Self::CreativeAndContentCreation => {
                serializer.serialize_str("creative_and_content_creation")
            }
            Self::CreativeAndContentGroups => {
                serializer.serialize_str("creative_and_content_groups")
            }
            Self::CreativeAndEducation => serializer.serialize_str("creative_and_education"),
            Self::CreativeGigs => serializer.serialize_str("creative_gigs"),
            Self::CreativeServices => serializer.serialize_str("creative_services"),
            Self::CustomerSupportAgencies => serializer.serialize_str("customer_support_agencies"),
            Self::DatingAndRelationships => serializer.serialize_str("dating_and_relationships"),
            Self::DeliveryAndLogistics => serializer.serialize_str("delivery_and_logistics"),
            Self::DentalAndVision => serializer.serialize_str("dental_and_vision"),
            Self::DermatologyAndSkin => serializer.serialize_str("dermatology_and_skin"),
            Self::DesignAndCreativeAgencies => {
                serializer.serialize_str("design_and_creative_agencies")
            }
            Self::DeveloperAndTechnicalTools => {
                serializer.serialize_str("developer_and_technical_tools")
            }
            Self::DevelopmentAgencies => serializer.serialize_str("development_agencies"),
            Self::DigitalAndEducationMarketplaces => {
                serializer.serialize_str("digital_and_education_marketplaces")
            }
            Self::DigitalGoodsAndAccounts => serializer.serialize_str("digital_goods_and_accounts"),
            Self::ECommerceSoftware => serializer.serialize_str("e_commerce_software"),
            Self::EducationAndChildcare => serializer.serialize_str("education_and_childcare"),
            Self::EducationalTrainingEvents => {
                serializer.serialize_str("educational_training_events")
            }
            Self::ElectronicsAndGadgets => serializer.serialize_str("electronics_and_gadgets"),
            Self::EntertainmentAndLeisure => serializer.serialize_str("entertainment_and_leisure"),
            Self::FamilyAndCommunityEvents => {
                serializer.serialize_str("family_and_community_events")
            }
            Self::FinanceAndInvesting => serializer.serialize_str("finance_and_investing"),
            Self::FitnessAndAthletics => serializer.serialize_str("fitness_and_athletics"),
            Self::FitnessAndHealthGroups => serializer.serialize_str("fitness_and_health_groups"),
            Self::FitnessAndRecreation => serializer.serialize_str("fitness_and_recreation"),
            Self::FitnessEquipmentAndGear => serializer.serialize_str("fitness_equipment_and_gear"),
            Self::FoodAndBeverages => serializer.serialize_str("food_and_beverages"),
            Self::FoodAndHospitalityMarketplaces => {
                serializer.serialize_str("food_and_hospitality_marketplaces")
            }
            Self::FuneralAndDeathCare => serializer.serialize_str("funeral_and_death_care"),
            Self::GamingAndEntertainmentSoftware => {
                serializer.serialize_str("gaming_and_entertainment_software")
            }
            Self::GamingGroups => serializer.serialize_str("gaming_groups"),
            Self::GeneticAndSpecialized => serializer.serialize_str("genetic_and_specialized"),
            Self::GovernmentAndPublic => serializer.serialize_str("government_and_public"),
            Self::HealthAndWellness => serializer.serialize_str("health_and_wellness"),
            Self::HealthAndWellnessServices => {
                serializer.serialize_str("health_and_wellness_services")
            }
            Self::Healthcare => serializer.serialize_str("healthcare"),
            Self::HealthcareAndWellnessSoftware => {
                serializer.serialize_str("healthcare_and_wellness_software")
            }
            Self::HobbiesAndLifestyle => serializer.serialize_str("hobbies_and_lifestyle"),
            Self::HobbyAndInterestGroups => serializer.serialize_str("hobby_and_interest_groups"),
            Self::HomeAndLiving => serializer.serialize_str("home_and_living"),
            Self::HomeAndTradeServices => serializer.serialize_str("home_and_trade_services"),
            Self::HomeAndTradeStorefronts => serializer.serialize_str("home_and_trade_storefronts"),
            Self::HomeImprovementAndTools => serializer.serialize_str("home_improvement_and_tools"),
            Self::HomeServicesGigs => serializer.serialize_str("home_services_gigs"),
            Self::HospitalityAndLodging => serializer.serialize_str("hospitality_and_lodging"),
            Self::IndustrialAndManufacturing => {
                serializer.serialize_str("industrial_and_manufacturing")
            }
            Self::IndustrySpecificSoftware => {
                serializer.serialize_str("industry_specific_software")
            }
            Self::LanguageAndCommunication => {
                serializer.serialize_str("language_and_communication")
            }
            Self::LegalAndCompliance => serializer.serialize_str("legal_and_compliance"),
            Self::LifestyleAndCulture => serializer.serialize_str("lifestyle_and_culture"),
            Self::LifestyleAndPersonalGrowth => {
                serializer.serialize_str("lifestyle_and_personal_growth")
            }
            Self::LifestyleAndPersonalGrowthGroups => {
                serializer.serialize_str("lifestyle_and_personal_growth_groups")
            }
            Self::LifestyleAndWellnessEvents => {
                serializer.serialize_str("lifestyle_and_wellness_events")
            }
            Self::LogisticsAndTransportationServices => {
                serializer.serialize_str("logistics_and_transportation_services")
            }
            Self::MarketingAgencies => serializer.serialize_str("marketing_agencies"),
            Self::MarketingAndAdvertising => serializer.serialize_str("marketing_and_advertising"),
            Self::MarketingAndSalesSoftware => {
                serializer.serialize_str("marketing_and_sales_software")
            }
            Self::MediaAndPublishingCompanies => {
                serializer.serialize_str("media_and_publishing_companies")
            }
            Self::MentalHealthAndBehavioral => {
                serializer.serialize_str("mental_health_and_behavioral")
            }
            Self::Miscellaneous => serializer.serialize_str("miscellaneous"),
            Self::MusicAndPerformingArts => serializer.serialize_str("music_and_performing_arts"),
            Self::NewsAndPolitics => serializer.serialize_str("news_and_politics"),
            Self::NonprofitAndCharity => serializer.serialize_str("nonprofit_and_charity"),
            Self::OfficeAndBusinessSupplies => {
                serializer.serialize_str("office_and_business_supplies")
            }
            Self::OutdoorAndSports => serializer.serialize_str("outdoor_and_sports"),
            Self::PerformanceAndShowEvents => {
                serializer.serialize_str("performance_and_show_events")
            }
            Self::PersonalDevelopment => serializer.serialize_str("personal_development"),
            Self::PersonalFinance => serializer.serialize_str("personal_finance"),
            Self::PersonalServices => serializer.serialize_str("personal_services"),
            Self::PetServices => serializer.serialize_str("pet_services"),
            Self::PetsAndAnimals => serializer.serialize_str("pets_and_animals"),
            Self::PrimaryAndGeneralCare => serializer.serialize_str("primary_and_general_care"),
            Self::ProductMarketplaces => serializer.serialize_str("product_marketplaces"),
            Self::ProductivityAndBusinessOps => {
                serializer.serialize_str("productivity_and_business_ops")
            }
            Self::ProfessionalGigs => serializer.serialize_str("professional_gigs"),
            Self::ProfessionalServices => serializer.serialize_str("professional_services"),
            Self::ProfessionalServicesStorefront => {
                serializer.serialize_str("professional_services_storefront")
            }
            Self::PublishingAndInfoProducts => {
                serializer.serialize_str("publishing_and_info_products")
            }
            Self::RealEstate => serializer.serialize_str("real_estate"),
            Self::RealEstateSoftware => serializer.serialize_str("real_estate_software"),
            Self::RecruitingAndStaffing => serializer.serialize_str("recruiting_and_staffing"),
            Self::RehabilitationAndTherapy => {
                serializer.serialize_str("rehabilitation_and_therapy")
            }
            Self::ReligionAndFaith => serializer.serialize_str("religion_and_faith"),
            Self::RentalMarketplaces => serializer.serialize_str("rental_marketplaces"),
            Self::Restaurants => serializer.serialize_str("restaurants"),
            Self::Retail => serializer.serialize_str("retail"),
            Self::SalesAgencies => serializer.serialize_str("sales_agencies"),
            Self::SalesAndRevenue => serializer.serialize_str("sales_and_revenue"),
            Self::SecurityAndInvestigations => {
                serializer.serialize_str("security_and_investigations")
            }
            Self::SecurityAndPrivacySoftware => {
                serializer.serialize_str("security_and_privacy_software")
            }
            Self::ServiceMarketplaces => serializer.serialize_str("service_marketplaces"),
            Self::SleepAndChronicConditions => {
                serializer.serialize_str("sleep_and_chronic_conditions")
            }
            Self::SocialAndNetworkingEvents => {
                serializer.serialize_str("social_and_networking_events")
            }
            Self::SocialEntertainmentEvents => {
                serializer.serialize_str("social_entertainment_events")
            }
            Self::SpecializedGigs => serializer.serialize_str("specialized_gigs"),
            Self::SpecialtyMedicalCare => serializer.serialize_str("specialty_medical_care"),
            Self::SpiritualityAndMindfulness => {
                serializer.serialize_str("spirituality_and_mindfulness")
            }
            Self::SpiritualityAndPersonalGrowth => {
                serializer.serialize_str("spirituality_and_personal_growth")
            }
            Self::SportsAndFitnessEvents => serializer.serialize_str("sports_and_fitness_events"),
            Self::SportsBettingAndGambling => {
                serializer.serialize_str("sports_betting_and_gambling")
            }
            Self::SportsBettingGroups => serializer.serialize_str("sports_betting_groups"),
            Self::SupplementsAndNutrition => serializer.serialize_str("supplements_and_nutrition"),
            Self::SustainabilityAndEcoProducts => {
                serializer.serialize_str("sustainability_and_eco_products")
            }
            Self::TaskAndErrands => serializer.serialize_str("task_and_errands"),
            Self::TechAndAi => serializer.serialize_str("tech_and_ai"),
            Self::TechAndDevGroups => serializer.serialize_str("tech_and_dev_groups"),
            Self::TechAndDevelopment => serializer.serialize_str("tech_and_development"),
            Self::TradingAndFinanceSoftware => {
                serializer.serialize_str("trading_and_finance_software")
            }
            Self::TradingAndInvesting => serializer.serialize_str("trading_and_investing"),
            Self::TradingAndInvestingGroups => {
                serializer.serialize_str("trading_and_investing_groups")
            }
            Self::Transportation => serializer.serialize_str("transportation"),
            Self::Veterinary => serializer.serialize_str("veterinary"),
            Self::VideoGamesAndEsports => serializer.serialize_str("video_games_and_esports"),
            Self::WeightAndMetabolicHealth => {
                serializer.serialize_str("weight_and_metabolic_health")
            }
            Self::WellnessAndAlternative => serializer.serialize_str("wellness_and_alternative"),
            Self::WomensAndMensHealth => serializer.serialize_str("womens_and_mens_health"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAccountsRequestIndustryGroup {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "academic_and_test_prep" => Ok(Self::AcademicAndTestPrep),
            "accessories" => Ok(Self::Accessories),
            "agriculture_and_farming" => Ok(Self::AgricultureAndFarming),
            "ai_and_automation_agencies" => Ok(Self::AiAndAutomationAgencies),
            "ai_and_automation_software" => Ok(Self::AiAndAutomationSoftware),
            "arts_and_crafts" => Ok(Self::ArtsAndCrafts),
            "automotive" => Ok(Self::Automotive),
            "b2b_and_professional_marketplaces" => Ok(Self::B2BAndProfessionalMarketplaces),
            "baby_and_kids" => Ok(Self::BabyAndKids),
            "bars_and_breweries" => Ok(Self::BarsAndBreweries),
            "beauty_and_personal_care" => Ok(Self::BeautyAndPersonalCare),
            "beauty_and_wellness" => Ok(Self::BeautyAndWellness),
            "business_and_entrepreneurship" => Ok(Self::BusinessAndEntrepreneurship),
            "business_and_money_groups" => Ok(Self::BusinessAndMoneyGroups),
            "cafes_and_quick_service" => Ok(Self::CafesAndQuickService),
            "career_and_professional" => Ok(Self::CareerAndProfessional),
            "charity_and_cause_events" => Ok(Self::CharityAndCauseEvents),
            "class_action_settlement" => Ok(Self::ClassActionSettlement),
            "clothing_and_apparel" => Ok(Self::ClothingAndApparel),
            "communication_and_messaging_software" => Ok(Self::CommunicationAndMessagingSoftware),
            "community_and_education_software" => Ok(Self::CommunityAndEducationSoftware),
            "conference_and_expo_events" => Ok(Self::ConferenceAndExpoEvents),
            "consulting" => Ok(Self::Consulting),
            "content_and_clipping_agencies" => Ok(Self::ContentAndClippingAgencies),
            "creative_and_content_creation" => Ok(Self::CreativeAndContentCreation),
            "creative_and_content_groups" => Ok(Self::CreativeAndContentGroups),
            "creative_and_education" => Ok(Self::CreativeAndEducation),
            "creative_gigs" => Ok(Self::CreativeGigs),
            "creative_services" => Ok(Self::CreativeServices),
            "customer_support_agencies" => Ok(Self::CustomerSupportAgencies),
            "dating_and_relationships" => Ok(Self::DatingAndRelationships),
            "delivery_and_logistics" => Ok(Self::DeliveryAndLogistics),
            "dental_and_vision" => Ok(Self::DentalAndVision),
            "dermatology_and_skin" => Ok(Self::DermatologyAndSkin),
            "design_and_creative_agencies" => Ok(Self::DesignAndCreativeAgencies),
            "developer_and_technical_tools" => Ok(Self::DeveloperAndTechnicalTools),
            "development_agencies" => Ok(Self::DevelopmentAgencies),
            "digital_and_education_marketplaces" => Ok(Self::DigitalAndEducationMarketplaces),
            "digital_goods_and_accounts" => Ok(Self::DigitalGoodsAndAccounts),
            "e_commerce_software" => Ok(Self::ECommerceSoftware),
            "education_and_childcare" => Ok(Self::EducationAndChildcare),
            "educational_training_events" => Ok(Self::EducationalTrainingEvents),
            "electronics_and_gadgets" => Ok(Self::ElectronicsAndGadgets),
            "entertainment_and_leisure" => Ok(Self::EntertainmentAndLeisure),
            "family_and_community_events" => Ok(Self::FamilyAndCommunityEvents),
            "finance_and_investing" => Ok(Self::FinanceAndInvesting),
            "fitness_and_athletics" => Ok(Self::FitnessAndAthletics),
            "fitness_and_health_groups" => Ok(Self::FitnessAndHealthGroups),
            "fitness_and_recreation" => Ok(Self::FitnessAndRecreation),
            "fitness_equipment_and_gear" => Ok(Self::FitnessEquipmentAndGear),
            "food_and_beverages" => Ok(Self::FoodAndBeverages),
            "food_and_hospitality_marketplaces" => Ok(Self::FoodAndHospitalityMarketplaces),
            "funeral_and_death_care" => Ok(Self::FuneralAndDeathCare),
            "gaming_and_entertainment_software" => Ok(Self::GamingAndEntertainmentSoftware),
            "gaming_groups" => Ok(Self::GamingGroups),
            "genetic_and_specialized" => Ok(Self::GeneticAndSpecialized),
            "government_and_public" => Ok(Self::GovernmentAndPublic),
            "health_and_wellness" => Ok(Self::HealthAndWellness),
            "health_and_wellness_services" => Ok(Self::HealthAndWellnessServices),
            "healthcare" => Ok(Self::Healthcare),
            "healthcare_and_wellness_software" => Ok(Self::HealthcareAndWellnessSoftware),
            "hobbies_and_lifestyle" => Ok(Self::HobbiesAndLifestyle),
            "hobby_and_interest_groups" => Ok(Self::HobbyAndInterestGroups),
            "home_and_living" => Ok(Self::HomeAndLiving),
            "home_and_trade_services" => Ok(Self::HomeAndTradeServices),
            "home_and_trade_storefronts" => Ok(Self::HomeAndTradeStorefronts),
            "home_improvement_and_tools" => Ok(Self::HomeImprovementAndTools),
            "home_services_gigs" => Ok(Self::HomeServicesGigs),
            "hospitality_and_lodging" => Ok(Self::HospitalityAndLodging),
            "industrial_and_manufacturing" => Ok(Self::IndustrialAndManufacturing),
            "industry_specific_software" => Ok(Self::IndustrySpecificSoftware),
            "language_and_communication" => Ok(Self::LanguageAndCommunication),
            "legal_and_compliance" => Ok(Self::LegalAndCompliance),
            "lifestyle_and_culture" => Ok(Self::LifestyleAndCulture),
            "lifestyle_and_personal_growth" => Ok(Self::LifestyleAndPersonalGrowth),
            "lifestyle_and_personal_growth_groups" => Ok(Self::LifestyleAndPersonalGrowthGroups),
            "lifestyle_and_wellness_events" => Ok(Self::LifestyleAndWellnessEvents),
            "logistics_and_transportation_services" => Ok(Self::LogisticsAndTransportationServices),
            "marketing_agencies" => Ok(Self::MarketingAgencies),
            "marketing_and_advertising" => Ok(Self::MarketingAndAdvertising),
            "marketing_and_sales_software" => Ok(Self::MarketingAndSalesSoftware),
            "media_and_publishing_companies" => Ok(Self::MediaAndPublishingCompanies),
            "mental_health_and_behavioral" => Ok(Self::MentalHealthAndBehavioral),
            "miscellaneous" => Ok(Self::Miscellaneous),
            "music_and_performing_arts" => Ok(Self::MusicAndPerformingArts),
            "news_and_politics" => Ok(Self::NewsAndPolitics),
            "nonprofit_and_charity" => Ok(Self::NonprofitAndCharity),
            "office_and_business_supplies" => Ok(Self::OfficeAndBusinessSupplies),
            "outdoor_and_sports" => Ok(Self::OutdoorAndSports),
            "performance_and_show_events" => Ok(Self::PerformanceAndShowEvents),
            "personal_development" => Ok(Self::PersonalDevelopment),
            "personal_finance" => Ok(Self::PersonalFinance),
            "personal_services" => Ok(Self::PersonalServices),
            "pet_services" => Ok(Self::PetServices),
            "pets_and_animals" => Ok(Self::PetsAndAnimals),
            "primary_and_general_care" => Ok(Self::PrimaryAndGeneralCare),
            "product_marketplaces" => Ok(Self::ProductMarketplaces),
            "productivity_and_business_ops" => Ok(Self::ProductivityAndBusinessOps),
            "professional_gigs" => Ok(Self::ProfessionalGigs),
            "professional_services" => Ok(Self::ProfessionalServices),
            "professional_services_storefront" => Ok(Self::ProfessionalServicesStorefront),
            "publishing_and_info_products" => Ok(Self::PublishingAndInfoProducts),
            "real_estate" => Ok(Self::RealEstate),
            "real_estate_software" => Ok(Self::RealEstateSoftware),
            "recruiting_and_staffing" => Ok(Self::RecruitingAndStaffing),
            "rehabilitation_and_therapy" => Ok(Self::RehabilitationAndTherapy),
            "religion_and_faith" => Ok(Self::ReligionAndFaith),
            "rental_marketplaces" => Ok(Self::RentalMarketplaces),
            "restaurants" => Ok(Self::Restaurants),
            "retail" => Ok(Self::Retail),
            "sales_agencies" => Ok(Self::SalesAgencies),
            "sales_and_revenue" => Ok(Self::SalesAndRevenue),
            "security_and_investigations" => Ok(Self::SecurityAndInvestigations),
            "security_and_privacy_software" => Ok(Self::SecurityAndPrivacySoftware),
            "service_marketplaces" => Ok(Self::ServiceMarketplaces),
            "sleep_and_chronic_conditions" => Ok(Self::SleepAndChronicConditions),
            "social_and_networking_events" => Ok(Self::SocialAndNetworkingEvents),
            "social_entertainment_events" => Ok(Self::SocialEntertainmentEvents),
            "specialized_gigs" => Ok(Self::SpecializedGigs),
            "specialty_medical_care" => Ok(Self::SpecialtyMedicalCare),
            "spirituality_and_mindfulness" => Ok(Self::SpiritualityAndMindfulness),
            "spirituality_and_personal_growth" => Ok(Self::SpiritualityAndPersonalGrowth),
            "sports_and_fitness_events" => Ok(Self::SportsAndFitnessEvents),
            "sports_betting_and_gambling" => Ok(Self::SportsBettingAndGambling),
            "sports_betting_groups" => Ok(Self::SportsBettingGroups),
            "supplements_and_nutrition" => Ok(Self::SupplementsAndNutrition),
            "sustainability_and_eco_products" => Ok(Self::SustainabilityAndEcoProducts),
            "task_and_errands" => Ok(Self::TaskAndErrands),
            "tech_and_ai" => Ok(Self::TechAndAi),
            "tech_and_dev_groups" => Ok(Self::TechAndDevGroups),
            "tech_and_development" => Ok(Self::TechAndDevelopment),
            "trading_and_finance_software" => Ok(Self::TradingAndFinanceSoftware),
            "trading_and_investing" => Ok(Self::TradingAndInvesting),
            "trading_and_investing_groups" => Ok(Self::TradingAndInvestingGroups),
            "transportation" => Ok(Self::Transportation),
            "veterinary" => Ok(Self::Veterinary),
            "video_games_and_esports" => Ok(Self::VideoGamesAndEsports),
            "weight_and_metabolic_health" => Ok(Self::WeightAndMetabolicHealth),
            "wellness_and_alternative" => Ok(Self::WellnessAndAlternative),
            "womens_and_mens_health" => Ok(Self::WomensAndMensHealth),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAccountsRequestIndustryGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AcademicAndTestPrep => write!(f, "academic_and_test_prep"),
            Self::Accessories => write!(f, "accessories"),
            Self::AgricultureAndFarming => write!(f, "agriculture_and_farming"),
            Self::AiAndAutomationAgencies => write!(f, "ai_and_automation_agencies"),
            Self::AiAndAutomationSoftware => write!(f, "ai_and_automation_software"),
            Self::ArtsAndCrafts => write!(f, "arts_and_crafts"),
            Self::Automotive => write!(f, "automotive"),
            Self::B2BAndProfessionalMarketplaces => write!(f, "b2b_and_professional_marketplaces"),
            Self::BabyAndKids => write!(f, "baby_and_kids"),
            Self::BarsAndBreweries => write!(f, "bars_and_breweries"),
            Self::BeautyAndPersonalCare => write!(f, "beauty_and_personal_care"),
            Self::BeautyAndWellness => write!(f, "beauty_and_wellness"),
            Self::BusinessAndEntrepreneurship => write!(f, "business_and_entrepreneurship"),
            Self::BusinessAndMoneyGroups => write!(f, "business_and_money_groups"),
            Self::CafesAndQuickService => write!(f, "cafes_and_quick_service"),
            Self::CareerAndProfessional => write!(f, "career_and_professional"),
            Self::CharityAndCauseEvents => write!(f, "charity_and_cause_events"),
            Self::ClassActionSettlement => write!(f, "class_action_settlement"),
            Self::ClothingAndApparel => write!(f, "clothing_and_apparel"),
            Self::CommunicationAndMessagingSoftware => {
                write!(f, "communication_and_messaging_software")
            }
            Self::CommunityAndEducationSoftware => write!(f, "community_and_education_software"),
            Self::ConferenceAndExpoEvents => write!(f, "conference_and_expo_events"),
            Self::Consulting => write!(f, "consulting"),
            Self::ContentAndClippingAgencies => write!(f, "content_and_clipping_agencies"),
            Self::CreativeAndContentCreation => write!(f, "creative_and_content_creation"),
            Self::CreativeAndContentGroups => write!(f, "creative_and_content_groups"),
            Self::CreativeAndEducation => write!(f, "creative_and_education"),
            Self::CreativeGigs => write!(f, "creative_gigs"),
            Self::CreativeServices => write!(f, "creative_services"),
            Self::CustomerSupportAgencies => write!(f, "customer_support_agencies"),
            Self::DatingAndRelationships => write!(f, "dating_and_relationships"),
            Self::DeliveryAndLogistics => write!(f, "delivery_and_logistics"),
            Self::DentalAndVision => write!(f, "dental_and_vision"),
            Self::DermatologyAndSkin => write!(f, "dermatology_and_skin"),
            Self::DesignAndCreativeAgencies => write!(f, "design_and_creative_agencies"),
            Self::DeveloperAndTechnicalTools => write!(f, "developer_and_technical_tools"),
            Self::DevelopmentAgencies => write!(f, "development_agencies"),
            Self::DigitalAndEducationMarketplaces => {
                write!(f, "digital_and_education_marketplaces")
            }
            Self::DigitalGoodsAndAccounts => write!(f, "digital_goods_and_accounts"),
            Self::ECommerceSoftware => write!(f, "e_commerce_software"),
            Self::EducationAndChildcare => write!(f, "education_and_childcare"),
            Self::EducationalTrainingEvents => write!(f, "educational_training_events"),
            Self::ElectronicsAndGadgets => write!(f, "electronics_and_gadgets"),
            Self::EntertainmentAndLeisure => write!(f, "entertainment_and_leisure"),
            Self::FamilyAndCommunityEvents => write!(f, "family_and_community_events"),
            Self::FinanceAndInvesting => write!(f, "finance_and_investing"),
            Self::FitnessAndAthletics => write!(f, "fitness_and_athletics"),
            Self::FitnessAndHealthGroups => write!(f, "fitness_and_health_groups"),
            Self::FitnessAndRecreation => write!(f, "fitness_and_recreation"),
            Self::FitnessEquipmentAndGear => write!(f, "fitness_equipment_and_gear"),
            Self::FoodAndBeverages => write!(f, "food_and_beverages"),
            Self::FoodAndHospitalityMarketplaces => write!(f, "food_and_hospitality_marketplaces"),
            Self::FuneralAndDeathCare => write!(f, "funeral_and_death_care"),
            Self::GamingAndEntertainmentSoftware => write!(f, "gaming_and_entertainment_software"),
            Self::GamingGroups => write!(f, "gaming_groups"),
            Self::GeneticAndSpecialized => write!(f, "genetic_and_specialized"),
            Self::GovernmentAndPublic => write!(f, "government_and_public"),
            Self::HealthAndWellness => write!(f, "health_and_wellness"),
            Self::HealthAndWellnessServices => write!(f, "health_and_wellness_services"),
            Self::Healthcare => write!(f, "healthcare"),
            Self::HealthcareAndWellnessSoftware => write!(f, "healthcare_and_wellness_software"),
            Self::HobbiesAndLifestyle => write!(f, "hobbies_and_lifestyle"),
            Self::HobbyAndInterestGroups => write!(f, "hobby_and_interest_groups"),
            Self::HomeAndLiving => write!(f, "home_and_living"),
            Self::HomeAndTradeServices => write!(f, "home_and_trade_services"),
            Self::HomeAndTradeStorefronts => write!(f, "home_and_trade_storefronts"),
            Self::HomeImprovementAndTools => write!(f, "home_improvement_and_tools"),
            Self::HomeServicesGigs => write!(f, "home_services_gigs"),
            Self::HospitalityAndLodging => write!(f, "hospitality_and_lodging"),
            Self::IndustrialAndManufacturing => write!(f, "industrial_and_manufacturing"),
            Self::IndustrySpecificSoftware => write!(f, "industry_specific_software"),
            Self::LanguageAndCommunication => write!(f, "language_and_communication"),
            Self::LegalAndCompliance => write!(f, "legal_and_compliance"),
            Self::LifestyleAndCulture => write!(f, "lifestyle_and_culture"),
            Self::LifestyleAndPersonalGrowth => write!(f, "lifestyle_and_personal_growth"),
            Self::LifestyleAndPersonalGrowthGroups => {
                write!(f, "lifestyle_and_personal_growth_groups")
            }
            Self::LifestyleAndWellnessEvents => write!(f, "lifestyle_and_wellness_events"),
            Self::LogisticsAndTransportationServices => {
                write!(f, "logistics_and_transportation_services")
            }
            Self::MarketingAgencies => write!(f, "marketing_agencies"),
            Self::MarketingAndAdvertising => write!(f, "marketing_and_advertising"),
            Self::MarketingAndSalesSoftware => write!(f, "marketing_and_sales_software"),
            Self::MediaAndPublishingCompanies => write!(f, "media_and_publishing_companies"),
            Self::MentalHealthAndBehavioral => write!(f, "mental_health_and_behavioral"),
            Self::Miscellaneous => write!(f, "miscellaneous"),
            Self::MusicAndPerformingArts => write!(f, "music_and_performing_arts"),
            Self::NewsAndPolitics => write!(f, "news_and_politics"),
            Self::NonprofitAndCharity => write!(f, "nonprofit_and_charity"),
            Self::OfficeAndBusinessSupplies => write!(f, "office_and_business_supplies"),
            Self::OutdoorAndSports => write!(f, "outdoor_and_sports"),
            Self::PerformanceAndShowEvents => write!(f, "performance_and_show_events"),
            Self::PersonalDevelopment => write!(f, "personal_development"),
            Self::PersonalFinance => write!(f, "personal_finance"),
            Self::PersonalServices => write!(f, "personal_services"),
            Self::PetServices => write!(f, "pet_services"),
            Self::PetsAndAnimals => write!(f, "pets_and_animals"),
            Self::PrimaryAndGeneralCare => write!(f, "primary_and_general_care"),
            Self::ProductMarketplaces => write!(f, "product_marketplaces"),
            Self::ProductivityAndBusinessOps => write!(f, "productivity_and_business_ops"),
            Self::ProfessionalGigs => write!(f, "professional_gigs"),
            Self::ProfessionalServices => write!(f, "professional_services"),
            Self::ProfessionalServicesStorefront => write!(f, "professional_services_storefront"),
            Self::PublishingAndInfoProducts => write!(f, "publishing_and_info_products"),
            Self::RealEstate => write!(f, "real_estate"),
            Self::RealEstateSoftware => write!(f, "real_estate_software"),
            Self::RecruitingAndStaffing => write!(f, "recruiting_and_staffing"),
            Self::RehabilitationAndTherapy => write!(f, "rehabilitation_and_therapy"),
            Self::ReligionAndFaith => write!(f, "religion_and_faith"),
            Self::RentalMarketplaces => write!(f, "rental_marketplaces"),
            Self::Restaurants => write!(f, "restaurants"),
            Self::Retail => write!(f, "retail"),
            Self::SalesAgencies => write!(f, "sales_agencies"),
            Self::SalesAndRevenue => write!(f, "sales_and_revenue"),
            Self::SecurityAndInvestigations => write!(f, "security_and_investigations"),
            Self::SecurityAndPrivacySoftware => write!(f, "security_and_privacy_software"),
            Self::ServiceMarketplaces => write!(f, "service_marketplaces"),
            Self::SleepAndChronicConditions => write!(f, "sleep_and_chronic_conditions"),
            Self::SocialAndNetworkingEvents => write!(f, "social_and_networking_events"),
            Self::SocialEntertainmentEvents => write!(f, "social_entertainment_events"),
            Self::SpecializedGigs => write!(f, "specialized_gigs"),
            Self::SpecialtyMedicalCare => write!(f, "specialty_medical_care"),
            Self::SpiritualityAndMindfulness => write!(f, "spirituality_and_mindfulness"),
            Self::SpiritualityAndPersonalGrowth => write!(f, "spirituality_and_personal_growth"),
            Self::SportsAndFitnessEvents => write!(f, "sports_and_fitness_events"),
            Self::SportsBettingAndGambling => write!(f, "sports_betting_and_gambling"),
            Self::SportsBettingGroups => write!(f, "sports_betting_groups"),
            Self::SupplementsAndNutrition => write!(f, "supplements_and_nutrition"),
            Self::SustainabilityAndEcoProducts => write!(f, "sustainability_and_eco_products"),
            Self::TaskAndErrands => write!(f, "task_and_errands"),
            Self::TechAndAi => write!(f, "tech_and_ai"),
            Self::TechAndDevGroups => write!(f, "tech_and_dev_groups"),
            Self::TechAndDevelopment => write!(f, "tech_and_development"),
            Self::TradingAndFinanceSoftware => write!(f, "trading_and_finance_software"),
            Self::TradingAndInvesting => write!(f, "trading_and_investing"),
            Self::TradingAndInvestingGroups => write!(f, "trading_and_investing_groups"),
            Self::Transportation => write!(f, "transportation"),
            Self::Veterinary => write!(f, "veterinary"),
            Self::VideoGamesAndEsports => write!(f, "video_games_and_esports"),
            Self::WeightAndMetabolicHealth => write!(f, "weight_and_metabolic_health"),
            Self::WellnessAndAlternative => write!(f, "wellness_and_alternative"),
            Self::WomensAndMensHealth => write!(f, "womens_and_mens_health"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
