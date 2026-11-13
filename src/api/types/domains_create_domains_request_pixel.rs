pub use crate::prelude::*;

/// Track page views on the domain with the Whop pixel. It needs no other capability: it stays `pending` until the pixel records a page view, and turns `action_required` if it goes 7 days without one. Pass `null` to stop tracking.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDomainsRequestPixel {}

impl CreateDomainsRequestPixel {
    pub fn builder() -> CreateDomainsRequestPixelBuilder {
        <CreateDomainsRequestPixelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDomainsRequestPixelBuilder {}

impl CreateDomainsRequestPixelBuilder {
    /// Consumes the builder and constructs a [`CreateDomainsRequestPixel`].
    pub fn build(self) -> Result<CreateDomainsRequestPixel, BuildError> {
        Ok(CreateDomainsRequestPixel {})
    }
}
