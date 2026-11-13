pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateFinancingApplicationsRequestAnswersItem {
    /// Complete replacement set of file IDs for this requirement, prefixed `file_`. Upload documents through the Files API first: new files must belong to the caller, be ready and private, and satisfy the requirement's formats and 20 MB limit. An empty array removes all files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Price answer, such as `max_product_price`. `null` clears the price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money: Option<UpdateFinancingApplicationsRequestAnswersItemMoney>,
    /// ID of a merchant requirement returned on this application.
    #[serde(default)]
    pub requirement_id: String,
    /// Text answers. Use `money` instead for `max_product_price`. An empty array clears the answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl UpdateFinancingApplicationsRequestAnswersItem {
    pub fn builder() -> UpdateFinancingApplicationsRequestAnswersItemBuilder {
        <UpdateFinancingApplicationsRequestAnswersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFinancingApplicationsRequestAnswersItemBuilder {
    file_ids: Option<Vec<String>>,
    money: Option<UpdateFinancingApplicationsRequestAnswersItemMoney>,
    requirement_id: Option<String>,
    values: Option<Vec<String>>,
}

impl UpdateFinancingApplicationsRequestAnswersItemBuilder {
    pub fn file_ids(mut self, value: Vec<String>) -> Self {
        self.file_ids = Some(value);
        self
    }

    pub fn money(mut self, value: UpdateFinancingApplicationsRequestAnswersItemMoney) -> Self {
        self.money = Some(value);
        self
    }

    pub fn requirement_id(mut self, value: impl Into<String>) -> Self {
        self.requirement_id = Some(value.into());
        self
    }

    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFinancingApplicationsRequestAnswersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requirement_id`](UpdateFinancingApplicationsRequestAnswersItemBuilder::requirement_id)
    pub fn build(self) -> Result<UpdateFinancingApplicationsRequestAnswersItem, BuildError> {
        Ok(UpdateFinancingApplicationsRequestAnswersItem {
            file_ids: self.file_ids,
            money: self.money,
            requirement_id: self
                .requirement_id
                .ok_or_else(|| BuildError::missing_field("requirement_id"))?,
            values: self.values,
        })
    }
}
