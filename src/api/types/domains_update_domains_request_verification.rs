pub use crate::prelude::*;

/// Prove you control the domain's DNS: Whop returns a TXT record to publish in `issues`, and whoever publishes it owns the domain on Whop. Pass `null` to release it. Can't be combined with `registration`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDomainsRequestVerification {}

impl UpdateDomainsRequestVerification {
    pub fn builder() -> UpdateDomainsRequestVerificationBuilder {
        <UpdateDomainsRequestVerificationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDomainsRequestVerificationBuilder {}

impl UpdateDomainsRequestVerificationBuilder {
    /// Consumes the builder and constructs a [`UpdateDomainsRequestVerification`].
    pub fn build(self) -> Result<UpdateDomainsRequestVerification, BuildError> {
        Ok(UpdateDomainsRequestVerification {})
    }
}
