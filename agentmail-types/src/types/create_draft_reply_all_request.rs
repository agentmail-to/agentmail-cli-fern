pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDraftReplyAllRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<DraftLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<DraftReplyTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<DraftSubject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<DraftText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<DraftHtml>,
    /// Attachments to include in draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<SendAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<DraftSendAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<DraftClientId>,
}

impl CreateDraftReplyAllRequest {
    pub fn builder() -> CreateDraftReplyAllRequestBuilder {
        <CreateDraftReplyAllRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDraftReplyAllRequestBuilder {
    labels: Option<DraftLabels>,
    reply_to: Option<DraftReplyTo>,
    subject: Option<DraftSubject>,
    text: Option<DraftText>,
    html: Option<DraftHtml>,
    attachments: Option<Vec<SendAttachment>>,
    send_at: Option<DraftSendAt>,
    client_id: Option<DraftClientId>,
}

impl CreateDraftReplyAllRequestBuilder {
    pub fn labels(mut self, value: DraftLabels) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn reply_to(mut self, value: DraftReplyTo) -> Self {
        self.reply_to = Some(value);
        self
    }

    pub fn subject(mut self, value: DraftSubject) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn text(mut self, value: DraftText) -> Self {
        self.text = Some(value);
        self
    }

    pub fn html(mut self, value: DraftHtml) -> Self {
        self.html = Some(value);
        self
    }

    pub fn attachments(mut self, value: Vec<SendAttachment>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn send_at(mut self, value: DraftSendAt) -> Self {
        self.send_at = Some(value);
        self
    }

    pub fn client_id(mut self, value: DraftClientId) -> Self {
        self.client_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDraftReplyAllRequest`].
    pub fn build(self) -> Result<CreateDraftReplyAllRequest, BuildError> {
        Ok(CreateDraftReplyAllRequest {
            labels: self.labels,
            reply_to: self.reply_to,
            subject: self.subject,
            text: self.text,
            html: self.html,
            attachments: self.attachments,
            send_at: self.send_at,
            client_id: self.client_id,
        })
    }
}

