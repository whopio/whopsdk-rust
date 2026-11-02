pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountWithdrawalScheduleControl {
    /// Day the automatic withdrawal runs on: 0-6 (Sunday-Saturday) for `weekly`, 1-31 for `monthly`. `null` for `manual` and `daily`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i64>,
    /// How often the account's balance automatically withdraws.
    pub frequency: AccountWithdrawalScheduleControlFrequency,
    /// Next date the automatic withdrawal is scheduled to run, as an ISO 8601 date. `null` for `manual` and `daily`, where no single next date applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payout_date: Option<String>,
}

impl AccountWithdrawalScheduleControl {
    pub fn builder() -> AccountWithdrawalScheduleControlBuilder {
        <AccountWithdrawalScheduleControlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountWithdrawalScheduleControlBuilder {
    day: Option<i64>,
    frequency: Option<AccountWithdrawalScheduleControlFrequency>,
    next_payout_date: Option<String>,
}

impl AccountWithdrawalScheduleControlBuilder {
    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    pub fn frequency(mut self, value: AccountWithdrawalScheduleControlFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn next_payout_date(mut self, value: impl Into<String>) -> Self {
        self.next_payout_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountWithdrawalScheduleControl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`frequency`](AccountWithdrawalScheduleControlBuilder::frequency)
    pub fn build(self) -> Result<AccountWithdrawalScheduleControl, BuildError> {
        Ok(AccountWithdrawalScheduleControl {
            day: self.day,
            frequency: self
                .frequency
                .ok_or_else(|| BuildError::missing_field("frequency"))?,
            next_payout_date: self.next_payout_date,
        })
    }
}
