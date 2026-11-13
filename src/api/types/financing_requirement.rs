pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinancingRequirement {
    #[serde(default)]
    pub accepted_file_formats: Vec<String>,
    /// The information requested, such as business_description or processing_statements.
    #[serde(default)]
    pub field: String,
    /// Whether the requirement accepts no file, one file, or multiple files.
    pub file_collection_type: FinancingRequirementFileCollectionType,
    #[serde(default)]
    pub files: Vec<File>,
    /// Requirement ID to use when updating this application's answers.
    #[serde(default)]
    pub id: String,
    /// Maximum size in bytes for each uploaded document.
    #[serde(default)]
    pub max_file_size: i64,
    /// Minimum characters per text answer, when specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
    /// Saved maximum product price in its original currency. Null when unanswered or the saved legacy answer is invalid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money: Option<Money>,
    #[serde(default)]
    pub options: Vec<String>,
    /// Whether this requirement must be completed before submission.
    #[serde(default)]
    pub required: bool,
    /// Whether the requirement accepts no text, one value, or multiple values.
    pub text_collection_type: FinancingRequirementTextCollectionType,
    /// Expected text format. max_product_price uses money instead of values.
    pub text_format: FinancingRequirementTextFormat,
    #[serde(default)]
    pub values: Vec<String>,
}

impl FinancingRequirement {
    pub fn builder() -> FinancingRequirementBuilder {
        <FinancingRequirementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancingRequirementBuilder {
    accepted_file_formats: Option<Vec<String>>,
    field: Option<String>,
    file_collection_type: Option<FinancingRequirementFileCollectionType>,
    files: Option<Vec<File>>,
    id: Option<String>,
    max_file_size: Option<i64>,
    minimum_length: Option<i64>,
    money: Option<Money>,
    options: Option<Vec<String>>,
    required: Option<bool>,
    text_collection_type: Option<FinancingRequirementTextCollectionType>,
    text_format: Option<FinancingRequirementTextFormat>,
    values: Option<Vec<String>>,
}

impl FinancingRequirementBuilder {
    pub fn accepted_file_formats(mut self, value: Vec<String>) -> Self {
        self.accepted_file_formats = Some(value);
        self
    }

    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn file_collection_type(mut self, value: FinancingRequirementFileCollectionType) -> Self {
        self.file_collection_type = Some(value);
        self
    }

    pub fn files(mut self, value: Vec<File>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn max_file_size(mut self, value: i64) -> Self {
        self.max_file_size = Some(value);
        self
    }

    pub fn minimum_length(mut self, value: i64) -> Self {
        self.minimum_length = Some(value);
        self
    }

    pub fn money(mut self, value: Money) -> Self {
        self.money = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn text_collection_type(mut self, value: FinancingRequirementTextCollectionType) -> Self {
        self.text_collection_type = Some(value);
        self
    }

    pub fn text_format(mut self, value: FinancingRequirementTextFormat) -> Self {
        self.text_format = Some(value);
        self
    }

    pub fn values(mut self, value: Vec<String>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinancingRequirement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accepted_file_formats`](FinancingRequirementBuilder::accepted_file_formats)
    /// - [`field`](FinancingRequirementBuilder::field)
    /// - [`file_collection_type`](FinancingRequirementBuilder::file_collection_type)
    /// - [`files`](FinancingRequirementBuilder::files)
    /// - [`id`](FinancingRequirementBuilder::id)
    /// - [`max_file_size`](FinancingRequirementBuilder::max_file_size)
    /// - [`options`](FinancingRequirementBuilder::options)
    /// - [`required`](FinancingRequirementBuilder::required)
    /// - [`text_collection_type`](FinancingRequirementBuilder::text_collection_type)
    /// - [`text_format`](FinancingRequirementBuilder::text_format)
    /// - [`values`](FinancingRequirementBuilder::values)
    pub fn build(self) -> Result<FinancingRequirement, BuildError> {
        Ok(FinancingRequirement {
            accepted_file_formats: self
                .accepted_file_formats
                .ok_or_else(|| BuildError::missing_field("accepted_file_formats"))?,
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            file_collection_type: self
                .file_collection_type
                .ok_or_else(|| BuildError::missing_field("file_collection_type"))?,
            files: self
                .files
                .ok_or_else(|| BuildError::missing_field("files"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            max_file_size: self
                .max_file_size
                .ok_or_else(|| BuildError::missing_field("max_file_size"))?,
            minimum_length: self.minimum_length,
            money: self.money,
            options: self
                .options
                .ok_or_else(|| BuildError::missing_field("options"))?,
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
            text_collection_type: self
                .text_collection_type
                .ok_or_else(|| BuildError::missing_field("text_collection_type"))?,
            text_format: self
                .text_format
                .ok_or_else(|| BuildError::missing_field("text_format"))?,
            values: self
                .values
                .ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
