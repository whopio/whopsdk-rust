pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateForumPostsRequest {
    /// A list of file attachments to include with the post, such as images or videos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<CreateForumPostsRequestAttachmentsItem>>,
    /// The unique identifier of the company whose public forum to post in. Required when experience_id is 'public'. For example, 'biz_xxxxx'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// The main body of the post in Markdown format. For example, 'Check out this **update**'. Hidden if the post is paywalled and the viewer has not purchased access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The unique identifier of the experience to create this post in. For example, 'exp_xxxxx'. Pass 'public' along with company_id to automatically use the company's public forum.
    #[serde(default)]
    pub experience_id: String,
    /// Whether to send this post as a mention notification to all users in the experience who have mentions enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mention: Option<bool>,
    /// The unique identifier of the parent post to comment on. Omit this field to create a top-level post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// The price to unlock this post in the specified paywall currency. For example, 5.00 for $5.00. When set, users must purchase access to view the post content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paywall_amount: Option<f64>,
    /// The currency for the paywall price on this post. When set along with paywall_amount, users must purchase access to view the post content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paywall_currency: Option<Currencies>,
    /// Whether this post should be pinned to the top of the forum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    /// A poll to attach to this post, allowing members to vote on options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<CreateForumPostsRequestPoll>,
    /// The rich content of the post in Tiptap JSON format. When provided, takes priority over the markdown content field for rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_content: Option<String>,
    /// The title of the post, displayed prominently at the top. Required for paywalled posts as it remains visible to non-purchasers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Controls who can see this forum post, such as members only or public.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ForumPostVisibilityTypes>,
}

impl CreateForumPostsRequest {
    pub fn builder() -> CreateForumPostsRequestBuilder {
        <CreateForumPostsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateForumPostsRequestBuilder {
    attachments: Option<Vec<CreateForumPostsRequestAttachmentsItem>>,
    company_id: Option<String>,
    content: Option<String>,
    experience_id: Option<String>,
    is_mention: Option<bool>,
    parent_id: Option<String>,
    paywall_amount: Option<f64>,
    paywall_currency: Option<Currencies>,
    pinned: Option<bool>,
    poll: Option<CreateForumPostsRequestPoll>,
    rich_content: Option<String>,
    title: Option<String>,
    visibility: Option<ForumPostVisibilityTypes>,
}

impl CreateForumPostsRequestBuilder {
    pub fn attachments(mut self, value: Vec<CreateForumPostsRequestAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn is_mention(mut self, value: bool) -> Self {
        self.is_mention = Some(value);
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn paywall_amount(mut self, value: f64) -> Self {
        self.paywall_amount = Some(value);
        self
    }

    pub fn paywall_currency(mut self, value: Currencies) -> Self {
        self.paywall_currency = Some(value);
        self
    }

    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    pub fn poll(mut self, value: CreateForumPostsRequestPoll) -> Self {
        self.poll = Some(value);
        self
    }

    pub fn rich_content(mut self, value: impl Into<String>) -> Self {
        self.rich_content = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: ForumPostVisibilityTypes) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateForumPostsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`experience_id`](CreateForumPostsRequestBuilder::experience_id)
    pub fn build(self) -> Result<CreateForumPostsRequest, BuildError> {
        Ok(CreateForumPostsRequest {
            attachments: self.attachments,
            company_id: self.company_id,
            content: self.content,
            experience_id: self
                .experience_id
                .ok_or_else(|| BuildError::missing_field("experience_id"))?,
            is_mention: self.is_mention,
            parent_id: self.parent_id,
            paywall_amount: self.paywall_amount,
            paywall_currency: self.paywall_currency,
            pinned: self.pinned,
            poll: self.poll,
            rich_content: self.rich_content,
            title: self.title,
            visibility: self.visibility,
        })
    }
}
