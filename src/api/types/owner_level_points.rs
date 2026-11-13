pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OwnerLevelPoints {
    /// Points from paid Whop Ads charges, net of refunds: 10 for every dollar, rounded toward zero.
    #[serde(default)]
    pub ads: i64,
    /// Points from captured Whop Card spend, net of refunds: 10 for every dollar, rounded toward zero.
    #[serde(default)]
    pub cards: i64,
    /// Points from accounts the owner referred as a partner: 1 for every 3 they earn, rounded toward zero.
    #[serde(default)]
    pub partners: i64,
    /// Points from perpetual trading volume filled through Whop: 1 for every dollar traded, rounded toward zero.
    #[serde(default)]
    pub trading: i64,
}

impl OwnerLevelPoints {
    pub fn builder() -> OwnerLevelPointsBuilder {
        <OwnerLevelPointsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnerLevelPointsBuilder {
    ads: Option<i64>,
    cards: Option<i64>,
    partners: Option<i64>,
    trading: Option<i64>,
}

impl OwnerLevelPointsBuilder {
    pub fn ads(mut self, value: i64) -> Self {
        self.ads = Some(value);
        self
    }

    pub fn cards(mut self, value: i64) -> Self {
        self.cards = Some(value);
        self
    }

    pub fn partners(mut self, value: i64) -> Self {
        self.partners = Some(value);
        self
    }

    pub fn trading(mut self, value: i64) -> Self {
        self.trading = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnerLevelPoints`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ads`](OwnerLevelPointsBuilder::ads)
    /// - [`cards`](OwnerLevelPointsBuilder::cards)
    /// - [`partners`](OwnerLevelPointsBuilder::partners)
    /// - [`trading`](OwnerLevelPointsBuilder::trading)
    pub fn build(self) -> Result<OwnerLevelPoints, BuildError> {
        Ok(OwnerLevelPoints {
            ads: self.ads.ok_or_else(|| BuildError::missing_field("ads"))?,
            cards: self
                .cards
                .ok_or_else(|| BuildError::missing_field("cards"))?,
            partners: self
                .partners
                .ok_or_else(|| BuildError::missing_field("partners"))?,
            trading: self
                .trading
                .ok_or_else(|| BuildError::missing_field("trading"))?,
        })
    }
}
