pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainDnsRecord {
    /// Full hostname where the record must be published.
    #[serde(default)]
    pub name: String,
    /// DNS record type.
    pub r#type: DomainDnsRecordType,
    /// DNS record content.
    #[serde(default)]
    pub value: String,
}

impl DomainDnsRecord {
    pub fn builder() -> DomainDnsRecordBuilder {
        <DomainDnsRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainDnsRecordBuilder {
    name: Option<String>,
    r#type: Option<DomainDnsRecordType>,
    value: Option<String>,
}

impl DomainDnsRecordBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: DomainDnsRecordType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainDnsRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DomainDnsRecordBuilder::name)
    /// - [`r#type`](DomainDnsRecordBuilder::r#type)
    /// - [`value`](DomainDnsRecordBuilder::value)
    pub fn build(self) -> Result<DomainDnsRecord, BuildError> {
        Ok(DomainDnsRecord {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
