pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDraftReplyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<DraftLabels>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<DraftReplyTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<DraftTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<DraftCc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<DraftBcc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_all: Option<DraftReplyAll>,
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

impl CreateDraftReplyRequest {
    pub fn builder() -> CreateDraftReplyRequestBuilder {
        <CreateDraftReplyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDraftReplyRequestBuilder {
    labels: Option<DraftLabels>,
    reply_to: Option<DraftReplyTo>,
    to: Option<DraftTo>,
    cc: Option<DraftCc>,
    bcc: Option<DraftBcc>,
    reply_all: Option<DraftReplyAll>,
    subject: Option<DraftSubject>,
    text: Option<DraftText>,
    html: Option<DraftHtml>,
    attachments: Option<Vec<SendAttachment>>,
    send_at: Option<DraftSendAt>,
    client_id: Option<DraftClientId>,
}

impl CreateDraftReplyRequestBuilder {
    pub fn labels(mut self, value: DraftLabels) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn reply_to(mut self, value: DraftReplyTo) -> Self {
        self.reply_to = Some(value);
        self
    }

    pub fn to(mut self, value: DraftTo) -> Self {
        self.to = Some(value);
        self
    }

    pub fn cc(mut self, value: DraftCc) -> Self {
        self.cc = Some(value);
        self
    }

    pub fn bcc(mut self, value: DraftBcc) -> Self {
        self.bcc = Some(value);
        self
    }

    pub fn reply_all(mut self, value: DraftReplyAll) -> Self {
        self.reply_all = Some(value);
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

    /// Consumes the builder and constructs a [`CreateDraftReplyRequest`].
    pub fn build(self) -> Result<CreateDraftReplyRequest, BuildError> {
        Ok(CreateDraftReplyRequest {
            labels: self.labels,
            reply_to: self.reply_to,
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            reply_all: self.reply_all,
            subject: self.subject,
            text: self.text,
            html: self.html,
            attachments: self.attachments,
            send_at: self.send_at,
            client_id: self.client_id,
        })
    }
}

