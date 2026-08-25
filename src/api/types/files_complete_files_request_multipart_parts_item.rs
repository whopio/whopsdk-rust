pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompleteFilesRequestMultipartPartsItem {
    /// The `ETag` response header from the part's upload.
    #[serde(default)]
    pub etag: String,
    /// The 1-based index of the part.
    #[serde(default)]
    pub part_number: i64,
}

impl CompleteFilesRequestMultipartPartsItem {
    pub fn builder() -> CompleteFilesRequestMultipartPartsItemBuilder {
        <CompleteFilesRequestMultipartPartsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteFilesRequestMultipartPartsItemBuilder {
    etag: Option<String>,
    part_number: Option<i64>,
}

impl CompleteFilesRequestMultipartPartsItemBuilder {
    pub fn etag(mut self, value: impl Into<String>) -> Self {
        self.etag = Some(value.into());
        self
    }

    pub fn part_number(mut self, value: i64) -> Self {
        self.part_number = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompleteFilesRequestMultipartPartsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`etag`](CompleteFilesRequestMultipartPartsItemBuilder::etag)
    /// - [`part_number`](CompleteFilesRequestMultipartPartsItemBuilder::part_number)
    pub fn build(self) -> Result<CompleteFilesRequestMultipartPartsItem, BuildError> {
        Ok(CompleteFilesRequestMultipartPartsItem {
            etag: self.etag.ok_or_else(|| BuildError::missing_field("etag"))?,
            part_number: self
                .part_number
                .ok_or_else(|| BuildError::missing_field("part_number"))?,
        })
    }
}
