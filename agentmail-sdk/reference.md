# Reference
## Inboxes
<details><summary><code>client.inboxes.<a href="/src/api/resources/inboxes/client.rs">list</a>(limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;InboxesListInboxesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes list
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .list(
            &InboxesListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes.<a href="/src/api/resources/inboxes/client.rs">create</a>(request: InboxesCreateInboxRequest) -> Result&lt;InboxesInbox, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes create --display-name "My Agent" --username myagent --domain agentmail.to
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .create(
            &InboxesCreateInboxRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes.<a href="/src/api/resources/inboxes/client.rs">get</a>(inbox_id: InboxesInboxId) -> Result&lt;InboxesInbox, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes get --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .get(&InboxesInboxID("inbox_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes.<a href="/src/api/resources/inboxes/client.rs">delete</a>(inbox_id: InboxesInboxId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes delete --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .delete(&InboxesInboxID("inbox_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes.<a href="/src/api/resources/inboxes/client.rs">update</a>(inbox_id: InboxesInboxId, request: InboxesUpdateInboxRequest) -> Result&lt;InboxesInbox, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes update --inbox-id <inbox_id> --display-name "Updated Name"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .update(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesUpdateInboxRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Pods
<details><summary><code>client.pods.<a href="/src/api/resources/pods/client.rs">list</a>(limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;PodsListPodsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods list
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .list(
            &PodsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods.<a href="/src/api/resources/pods/client.rs">create</a>(request: PodsCreatePodRequest) -> Result&lt;PodsPod, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods create --client-id my-pod
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .create(
            &PodsCreatePodRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `Option<PodsName>` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<PodsClientId>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods.<a href="/src/api/resources/pods/client.rs">get</a>(pod_id: PodsPodId) -> Result&lt;PodsPod, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods get --pod-id <pod_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .get(&PodsPodID("pod_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods.<a href="/src/api/resources/pods/client.rs">delete</a>(pod_id: PodsPodId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods delete --pod-id <pod_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .delete(&PodsPodID("pod_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Webhooks
<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">list</a>(limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;WebhooksListWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail webhooks list
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .list(
            &WebhooksListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">create</a>(request: WebhooksCreateWebhookRequest) -> Result&lt;WebhooksWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail webhooks create --url https://example.com/webhook --event-type message.received
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .create(
            &WebhooksCreateWebhookRequest {
                url: WebhooksURL("url".to_string()),
                event_types: WebhooksCreateWebhookEventTypes(EventTypes(vec![
                    EventType::MessageReceived,
                ])),
                inbox_ids: None,
                client_id: None,
                pod_ids: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_ids:** `Option<PodIds>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">get</a>(webhook_id: WebhooksWebhookId) -> Result&lt;WebhooksWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail webhooks get --webhook-id <webhook_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .get(&WebhooksWebhookID("webhook_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `WebhooksWebhookId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">delete</a>(webhook_id: WebhooksWebhookId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail webhooks delete --webhook-id <webhook_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .delete(&WebhooksWebhookID("webhook_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `WebhooksWebhookId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.webhooks.<a href="/src/api/resources/webhooks/client.rs">update</a>(webhook_id: WebhooksWebhookId, request: WebhooksUpdateWebhookRequest) -> Result&lt;WebhooksWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update inbox or pod subscriptions, or replace the webhook's `event_types` in full when you pass a
non-empty `event_types` array (see request field docs). Inbox and pod changes use add/remove lists.

**CLI:**
```bash
agentmail webhooks update --webhook-id <webhook_id> --add-inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .webhooks
        .update(
            &WebhooksWebhookID("webhook_id".to_string()),
            &WebhooksUpdateWebhookRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**webhook_id:** `WebhooksWebhookId` 
    
</dd>
</dl>

<dl>
<dd>

**add_pod_ids:** `Option<PodIds>` — Pod IDs to subscribe to the webhook.
    
</dd>
</dl>

<dl>
<dd>

**remove_pod_ids:** `Option<PodIds>` — Pod IDs to unsubscribe from the webhook.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Agent
<details><summary><code>client.agent.<a href="/src/api/resources/agent/client.rs">sign_up</a>(request: AgentSignupRequest) -> Result&lt;AgentSignupResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new agent organization with an inbox and API key. This endpoint is for signing up for the first time. If you've already signed up, you're all set — just use your existing API key.

A 6-digit OTP is sent to the human's email for verification.

This endpoint is idempotent. Calling it again with the same `human_email` will rotate the API key and resend the OTP if expired.

The returned API key has limited permissions until the organization is verified via the verify endpoint.

**CLI:**
```bash
agentmail agent sign-up --human-email user@example.com --username my-agent
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .agent
        .sign_up(
            &AgentSignupRequest {
                human_email: "human_email".to_string(),
                username: "username".to_string(),
                source: None,
                referrer: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**human_email:** `String` — Email address of the human who owns the agent. A 6-digit OTP will be sent to this address.
    
</dd>
</dl>

<dl>
<dd>

**username:** `String` — Username for the auto-created inbox (e.g. "my-agent" creates my-agent@agentmail.to).
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<Option<String>>` 

The SDK, framework, or platform issuing this sign-up (e.g. `agentmail-python`, `agentmail-cli`, `agentmail-mcp`).
Identifies the caller — answers "who is signing up".
Max 2048 characters.
    
</dd>
</dl>

<dl>
<dd>

**referrer:** `Option<Option<String>>` 

The channel that drove this sign-up — where the agent or its developer discovered AgentMail
(e.g. `agent.email`, a partner URL, a campaign tag). Answers "where did this sign-up come from".
Max 2048 characters.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.agent.<a href="/src/api/resources/agent/client.rs">verify</a>(request: AgentVerifyRequest) -> Result&lt;AgentVerifyResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Verify an agent organization using the 6-digit OTP sent to the human's email during sign-up.

On success, the organization is upgraded from `agent_unverified` to `agent_verified`, the send allowlist is removed, and free plan entitlements are applied.

The OTP expires after 24 hours and allows a maximum of 10 attempts. If you run into any difficulties receiving the OTP code, you can also create an account on [console.agentmail.to](https://console.agentmail.to) using the human email address you provided to verify your account.

**CLI:**
```bash
agentmail agent verify --otp-code 123456
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .agent
        .verify(
            &AgentVerifyRequest {
                otp_code: "otp_code".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**otp_code:** `String` — 6-digit verification code sent to the human's email address.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ApiKeys
<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">list</a>(limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;ListApiKeysResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail api-keys list
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .api_keys
        .list(
            &APIKeysListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">create</a>(request: CreateApiKeyRequest) -> Result&lt;CreateApiKeyResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail api-keys create --name "My Key"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .api_keys
        .create(
            &CreateAPIKeyRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.api_keys.<a href="/src/api/resources/api_keys/client.rs">delete</a>(api_key_id: ApiKeyId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail api-keys delete --api-key-id <api_key_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .api_keys
        .delete(&APIKeyID("api_key_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**api_key_id:** `ApiKeyId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Auth
<details><summary><code>client.auth.<a href="/src/api/resources/auth/client.rs">me</a>() -> Result&lt;Identity, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the identity and scope of the authenticated credential. Useful when a client holds a pod-scoped or inbox-scoped API key and needs to discover the parent organization, pod, or inbox without prior knowledge.

**CLI:**
```bash
agentmail auth me
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client.auth.me(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Domains
<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">list</a>(limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;ListDomainsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail domains list
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .domains
        .list(
            &DomainsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">create</a>(request: CreateDomainRequest) -> Result&lt;Domain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail domains create --domain example.com
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .domains
        .create(
            &CreateDomainRequest {
                domain: DomainName("domain".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">get</a>(domain_id: DomainId) -> Result&lt;Domain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail domains get --domain-id <domain_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .domains
        .get(&DomainID("domain_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_id:** `DomainId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">delete</a>(domain_id: DomainId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail domains delete --domain-id <domain_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .domains
        .delete(&DomainID("domain_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_id:** `DomainId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">update</a>(domain_id: DomainId, request: UpdateDomainRequest) -> Result&lt;Domain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail domains update --domain-id <domain_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .domains
        .update(
            &DomainID("domain_id".to_string()),
            &UpdateDomainRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_id:** `DomainId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">get_zone_file</a>(domain_id: DomainId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail domains get-zone-file --domain-id <domain_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .domains
        .get_zone_file(&DomainID("domain_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_id:** `DomainId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.domains.<a href="/src/api/resources/domains/client.rs">verify</a>(domain_id: DomainId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail domains verify --domain-id <domain_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .domains
        .verify(&DomainID("domain_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_id:** `DomainId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Drafts
<details><summary><code>client.drafts.<a href="/src/api/resources/drafts/client.rs">list</a>(limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;ListDraftsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail drafts list
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .drafts
        .list(
            &DraftsListQueryRequest {
                limit: None,
                page_token: None,
                labels: vec![],
                before: None,
                after: None,
                ascending: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.drafts.<a href="/src/api/resources/drafts/client.rs">get</a>(draft_id: DraftId) -> Result&lt;Draft, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail drafts get --draft-id <draft_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .drafts
        .get(&DraftID("draft_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**draft_id:** `DraftId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.drafts.<a href="/src/api/resources/drafts/client.rs">get_attachment</a>(draft_id: DraftId, attachment_id: AttachmentId) -> Result&lt;AttachmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail drafts get-attachment --draft-id <draft_id> --attachment-id <attachment_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .drafts
        .get_attachment(
            &DraftID("draft_id".to_string()),
            &AttachmentID("attachment_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**draft_id:** `DraftId` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `AttachmentId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Lists
<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">list</a>(direction: Direction, type_: ListType, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;) -> Result&lt;ListListEntriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail lists list --direction <direction> --type <type>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .lists
        .list(
            &Direction::Send,
            &ListType::Allow,
            &ListsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">create</a>(direction: Direction, type_: ListType, request: CreateListEntryRequest) -> Result&lt;ListEntry, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail lists create --direction <direction> --type <type> --entry user@example.com
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .lists
        .create(
            &Direction::Send,
            &ListType::Allow,
            &CreateListEntryRequest {
                entry: "entry".to_string(),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">get</a>(direction: Direction, type_: ListType, entry: String) -> Result&lt;ListEntry, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail lists get --direction <direction> --type <type> --entry <entry>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .lists
        .get(
            &Direction::Send,
            &ListType::Allow,
            &"entry".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — Email address or domain.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.lists.<a href="/src/api/resources/lists/client.rs">delete</a>(direction: Direction, type_: ListType, entry: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail lists delete --direction <direction> --type <type> --entry <entry>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .lists
        .delete(
            &Direction::Send,
            &ListType::Allow,
            &"entry".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — Email address or domain.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Metrics
<details><summary><code>client.metrics.<a href="/src/api/resources/metrics/client.rs">query_events</a>(start: Option&lt;Option&lt;Start&gt;&gt;, end: Option&lt;Option&lt;End&gt;&gt;, period: Option&lt;Option&lt;Period&gt;&gt;, limit: Option&lt;Option&lt;MetricLimit&gt;&gt;, descending: Option&lt;Option&lt;Descending&gt;&gt;) -> Result&lt;QueryMetricsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Counts of email events (sent, delivered, bounced, etc.) over time for
the organization. Defaults to the last 24 hours; `start` must be within
the last 90 days, and a future `end` is clamped to now. Omit `period`
for individual event counts, or set it to sum counts into buckets of
that many seconds.

**CLI:**
```bash
agentmail metrics list
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .metrics
        .query_events(
            &MetricsQueryEventsQueryRequest {
                event_types: vec![],
                start: None,
                end: None,
                period: None,
                limit: None,
                descending: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**event_types:** `Option<MetricEventType>` 
    
</dd>
</dl>

<dl>
<dd>

**start:** `Option<Start>` 
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<End>` 
    
</dd>
</dl>

<dl>
<dd>

**period:** `Option<Period>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<MetricLimit>` 
    
</dd>
</dl>

<dl>
<dd>

**descending:** `Option<Descending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.metrics.<a href="/src/api/resources/metrics/client.rs">query_usage</a>(start: Option&lt;Option&lt;Start&gt;&gt;, end: Option&lt;Option&lt;End&gt;&gt;, period: Option&lt;Option&lt;Period&gt;&gt;, limit: Option&lt;Option&lt;MetricLimit&gt;&gt;, descending: Option&lt;Option&lt;Descending&gt;&gt;) -> Result&lt;QueryUsageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cumulative usage series for the organization. Each point is the running
total of the usage type at that timestamp, not the change within the
bucket. Defaults to the last 24 hours; `start` must be within the last
90 days, and a future `end` is clamped to now. The range divided by
`period` must not exceed 1000 buckets.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .metrics
        .query_usage(
            &MetricsQueryUsageQueryRequest {
                usage_types: vec![],
                start: None,
                end: None,
                period: None,
                limit: None,
                descending: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**usage_types:** `Option<UsageType>` 
    
</dd>
</dl>

<dl>
<dd>

**start:** `Option<Start>` 
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<End>` 
    
</dd>
</dl>

<dl>
<dd>

**period:** `Option<Period>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<MetricLimit>` 
    
</dd>
</dl>

<dl>
<dd>

**descending:** `Option<Descending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Organizations
<details><summary><code>client.organizations.<a href="/src/api/resources/organizations/client.rs">get</a>() -> Result&lt;Organization, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the organization for the authenticated API key (usage limits, counts, and billing metadata).

**CLI:**
```bash
agentmail organizations get
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client.organizations.get(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Threads
<details><summary><code>client.threads.<a href="/src/api/resources/threads/client.rs">list</a>(limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;, include_spam: Option&lt;Option&lt;IncludeSpam&gt;&gt;, include_blocked: Option&lt;Option&lt;IncludeBlocked&gt;&gt;, include_unauthenticated: Option&lt;Option&lt;IncludeUnauthenticated&gt;&gt;, include_trash: Option&lt;Option&lt;IncludeTrash&gt;&gt;, senders: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;, recipients: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;, subject: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;) -> Result&lt;ListThreadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists threads, most recent first. Pass `senders`, `recipients`, or
`subject` to filter by substring. Filtered requests are served by
search, which caps `limit` at 100. For relevance-ranked full-text
search across senders, recipients, subject, and message body, use
`Search Threads`.

**CLI:**
```bash
agentmail threads list
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .threads
        .list(
            &ThreadsListQueryRequest {
                limit: None,
                page_token: None,
                labels: vec![],
                before: None,
                after: None,
                ascending: None,
                include_spam: None,
                include_blocked: None,
                include_unauthenticated: None,
                include_trash: None,
                senders: None,
                recipients: None,
                subject: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>

<dl>
<dd>

**include_spam:** `Option<IncludeSpam>` 
    
</dd>
</dl>

<dl>
<dd>

**include_blocked:** `Option<IncludeBlocked>` 
    
</dd>
</dl>

<dl>
<dd>

**include_unauthenticated:** `Option<IncludeUnauthenticated>` 
    
</dd>
</dl>

<dl>
<dd>

**include_trash:** `Option<IncludeTrash>` 
    
</dd>
</dl>

<dl>
<dd>

**senders:** `Option<Option<Vec<String>>>` — Filter to threads whose senders contain this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `Option<Option<Vec<String>>>` — Filter to threads whose recipients contain this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<Option<Vec<String>>>` — Filter to threads whose subject contains this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.threads.<a href="/src/api/resources/threads/client.rs">search</a>(q: Option&lt;Query&gt;, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;) -> Result&lt;SearchThreadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Full-text search across threads in the organization, ranked by
relevance. The query is matched against senders, recipients, and
subject (substring) and the message body (tokenized full text). Spam,
trash, blocked, and unauthenticated threads are always excluded.
`limit` cannot exceed 100.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .threads
        .search(
            &ThreadsSearchQueryRequest {
                q: Query("q".to_string()),
                limit: None,
                page_token: None,
                before: None,
                after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**q:** `Query` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.threads.<a href="/src/api/resources/threads/client.rs">get</a>(thread_id: ThreadId) -> Result&lt;Thread, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail threads get --thread-id <thread_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .threads
        .get(&ThreadID("thread_id".to_string()), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.threads.<a href="/src/api/resources/threads/client.rs">delete</a>(thread_id: ThreadId, permanent: Option&lt;Option&lt;Option&lt;bool&gt;&gt;&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Moves the thread to trash by adding a trash label to all messages. If the thread is already in trash, it will be permanently deleted. Use `permanent=true` to force permanent deletion.

**CLI:**
```bash
agentmail threads delete --thread-id <thread_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .threads
        .delete(
            &ThreadID("thread_id".to_string()),
            &ThreadsDeleteQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>

<dl>
<dd>

**permanent:** `Option<Option<bool>>` — If true, permanently delete the thread instead of moving to trash.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.threads.<a href="/src/api/resources/threads/client.rs">update</a>(thread_id: ThreadId, request: UpdateThreadRequest) -> Result&lt;UpdateThreadResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates thread labels. Cannot add or remove system labels (sent, received, bounced, etc.). Rejects requests with a `422` for threads with 100 or more messages.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .threads
        .update(
            &ThreadID("thread_id".to_string()),
            &UpdateThreadRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.threads.<a href="/src/api/resources/threads/client.rs">get_attachment</a>(thread_id: ThreadId, attachment_id: AttachmentId) -> Result&lt;AttachmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail threads get-attachment --thread-id <thread_id> --attachment-id <attachment_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .threads
        .get_attachment(
            &ThreadID("thread_id".to_string()),
            &AttachmentID("attachment_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `AttachmentId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Inboxes ApiKeys
<details><summary><code>client.inboxes().api_keys.<a href="/src/api/resources/inboxes/api_keys/client.rs">list</a>(inbox_id: InboxesInboxId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;) -> Result&lt;ListApiKeysResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:api-keys list --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .api_keys
        .list(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesAPIKeysListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().api_keys.<a href="/src/api/resources/inboxes/api_keys/client.rs">create</a>(inbox_id: InboxesInboxId, request: CreateApiKeyRequest) -> Result&lt;CreateApiKeyResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:api-keys create --inbox-id <inbox_id> --name "My Key"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .api_keys
        .create(
            &InboxesInboxID("inbox_id".to_string()),
            &CreateAPIKeyRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().api_keys.<a href="/src/api/resources/inboxes/api_keys/client.rs">delete</a>(inbox_id: InboxesInboxId, api_key_id: ApiKeyId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:api-keys delete --inbox-id <inbox_id> --api-key-id <api_key_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .api_keys
        .delete(
            &InboxesInboxID("inbox_id".to_string()),
            &APIKeyID("api_key_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**api_key_id:** `ApiKeyId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Inboxes Drafts
<details><summary><code>client.inboxes().drafts.<a href="/src/api/resources/inboxes/drafts/client.rs">list</a>(inbox_id: InboxesInboxId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;ListDraftsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:drafts list --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .drafts
        .list(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesDraftsListQueryRequest {
                limit: None,
                page_token: None,
                labels: vec![],
                before: None,
                after: None,
                ascending: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().drafts.<a href="/src/api/resources/inboxes/drafts/client.rs">create</a>(inbox_id: InboxesInboxId, request: CreateDraftRequest) -> Result&lt;Draft, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:drafts create --inbox-id <inbox_id> --to recipient@example.com --subject "Draft subject" --text "Draft body"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .drafts
        .create(
            &InboxesInboxID("inbox_id".to_string()),
            &CreateDraftRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<DraftLabels>` 
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<DraftReplyTo>` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<DraftTo>` 
    
</dd>
</dl>

<dl>
<dd>

**cc:** `Option<DraftCc>` 
    
</dd>
</dl>

<dl>
<dd>

**bcc:** `Option<DraftBcc>` 
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<DraftSubject>` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `Option<DraftText>` 
    
</dd>
</dl>

<dl>
<dd>

**html:** `Option<DraftHtml>` 
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Option<Vec<SendAttachment>>>` — Attachments to include in draft.
    
</dd>
</dl>

<dl>
<dd>

**in_reply_to:** `Option<DraftInReplyTo>` 
    
</dd>
</dl>

<dl>
<dd>

**send_at:** `Option<DraftSendAt>` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<DraftClientId>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().drafts.<a href="/src/api/resources/inboxes/drafts/client.rs">get</a>(inbox_id: InboxesInboxId, draft_id: DraftId) -> Result&lt;Draft, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:drafts get --inbox-id <inbox_id> --draft-id <draft_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .drafts
        .get(
            &InboxesInboxID("inbox_id".to_string()),
            &DraftID("draft_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**draft_id:** `DraftId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().drafts.<a href="/src/api/resources/inboxes/drafts/client.rs">delete</a>(inbox_id: InboxesInboxId, draft_id: DraftId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:drafts delete --inbox-id <inbox_id> --draft-id <draft_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .drafts
        .delete(
            &InboxesInboxID("inbox_id".to_string()),
            &DraftID("draft_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**draft_id:** `DraftId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().drafts.<a href="/src/api/resources/inboxes/drafts/client.rs">update</a>(inbox_id: InboxesInboxId, draft_id: DraftId, request: UpdateDraftRequest) -> Result&lt;Draft, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:drafts update --inbox-id <inbox_id> --draft-id <draft_id> --subject "Updated subject"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .drafts
        .update(
            &InboxesInboxID("inbox_id".to_string()),
            &DraftID("draft_id".to_string()),
            &UpdateDraftRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**draft_id:** `DraftId` 
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<DraftReplyTo>` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<DraftTo>` 
    
</dd>
</dl>

<dl>
<dd>

**cc:** `Option<DraftCc>` 
    
</dd>
</dl>

<dl>
<dd>

**bcc:** `Option<DraftBcc>` 
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<DraftSubject>` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `Option<DraftText>` 
    
</dd>
</dl>

<dl>
<dd>

**html:** `Option<DraftHtml>` 
    
</dd>
</dl>

<dl>
<dd>

**send_at:** `Option<DraftSendAt>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().drafts.<a href="/src/api/resources/inboxes/drafts/client.rs">get_attachment</a>(inbox_id: InboxesInboxId, draft_id: DraftId, attachment_id: AttachmentId) -> Result&lt;AttachmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:drafts get-attachment --inbox-id <inbox_id> --draft-id <draft_id> --attachment-id <attachment_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .drafts
        .get_attachment(
            &InboxesInboxID("inbox_id".to_string()),
            &DraftID("draft_id".to_string()),
            &AttachmentID("attachment_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**draft_id:** `DraftId` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `AttachmentId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().drafts.<a href="/src/api/resources/inboxes/drafts/client.rs">send</a>(inbox_id: InboxesInboxId, draft_id: DraftId, request: UpdateMessageRequest) -> Result&lt;SendMessageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:drafts send --inbox-id <inbox_id> --draft-id <draft_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .drafts
        .send(
            &InboxesInboxID("inbox_id".to_string()),
            &DraftID("draft_id".to_string()),
            &UpdateMessageRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**draft_id:** `DraftId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Inboxes Events
<details><summary><code>client.inboxes().events.<a href="/src/api/resources/inboxes/events/client.rs">list</a>(inbox_id: InboxesInboxId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;ListInboxEventsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

List label change events for an inbox. Returns events in reverse chronological order by default. Use for IMAP UID projection or audit logging.

**CLI:**
```bash
agentmail inboxes:events list --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .events
        .list(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesEventsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Inboxes Lists
<details><summary><code>client.inboxes().lists.<a href="/src/api/resources/inboxes/lists/client.rs">list</a>(inbox_id: InboxesInboxId, direction: Direction, type_: ListType, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;) -> Result&lt;PodListListEntriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:lists list --inbox-id <inbox_id> --direction <direction> --type <type>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .lists
        .list(
            &InboxesInboxID("inbox_id".to_string()),
            &Direction::Send,
            &ListType::Allow,
            &InboxesListsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().lists.<a href="/src/api/resources/inboxes/lists/client.rs">create</a>(inbox_id: InboxesInboxId, direction: Direction, type_: ListType, request: CreateListEntryRequest) -> Result&lt;PodListEntry, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:lists create --inbox-id <inbox_id> --direction <direction> --type <type> --entry user@example.com
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .lists
        .create(
            &InboxesInboxID("inbox_id".to_string()),
            &Direction::Send,
            &ListType::Allow,
            &CreateListEntryRequest {
                entry: "entry".to_string(),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().lists.<a href="/src/api/resources/inboxes/lists/client.rs">get</a>(inbox_id: InboxesInboxId, direction: Direction, type_: ListType, entry: String) -> Result&lt;PodListEntry, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:lists get --inbox-id <inbox_id> --direction <direction> --type <type> --entry <entry>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .lists
        .get(
            &InboxesInboxID("inbox_id".to_string()),
            &Direction::Send,
            &ListType::Allow,
            &"entry".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — Email address or domain.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().lists.<a href="/src/api/resources/inboxes/lists/client.rs">delete</a>(inbox_id: InboxesInboxId, direction: Direction, type_: ListType, entry: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:lists delete --inbox-id <inbox_id> --direction <direction> --type <type> --entry <entry>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .lists
        .delete(
            &InboxesInboxID("inbox_id".to_string()),
            &Direction::Send,
            &ListType::Allow,
            &"entry".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — Email address or domain.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Inboxes Messages
<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">list</a>(inbox_id: InboxesInboxId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;, include_spam: Option&lt;Option&lt;IncludeSpam&gt;&gt;, include_blocked: Option&lt;Option&lt;IncludeBlocked&gt;&gt;, include_unauthenticated: Option&lt;Option&lt;IncludeUnauthenticated&gt;&gt;, include_trash: Option&lt;Option&lt;IncludeTrash&gt;&gt;, from: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;, to: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;, subject: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;) -> Result&lt;ListMessagesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists messages in the inbox, most recent first. Pass `from`, `to`, or
`subject` to filter by substring. Filtered requests are served by
search, which caps `limit` at 100. For relevance-ranked full-text
search across sender, recipients, subject, and message body, use
`Search Messages`.

**CLI:**
```bash
agentmail inboxes:messages list --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .list(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesMessagesListQueryRequest {
                limit: None,
                page_token: None,
                labels: vec![],
                before: None,
                after: None,
                ascending: None,
                include_spam: None,
                include_blocked: None,
                include_unauthenticated: None,
                include_trash: None,
                from: None,
                to: None,
                subject: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>

<dl>
<dd>

**include_spam:** `Option<IncludeSpam>` 
    
</dd>
</dl>

<dl>
<dd>

**include_blocked:** `Option<IncludeBlocked>` 
    
</dd>
</dl>

<dl>
<dd>

**include_unauthenticated:** `Option<IncludeUnauthenticated>` 
    
</dd>
</dl>

<dl>
<dd>

**include_trash:** `Option<IncludeTrash>` 
    
</dd>
</dl>

<dl>
<dd>

**from:** `Option<Option<Vec<String>>>` — Filter to messages whose sender contains this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<Option<Vec<String>>>` — Filter to messages whose recipients (to, cc, or bcc) contain this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<Option<Vec<String>>>` — Filter to messages whose subject contains this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">search</a>(inbox_id: InboxesInboxId, q: Option&lt;Query&gt;, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;) -> Result&lt;SearchMessagesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Full-text search across messages in the inbox, ranked by relevance. The
query is matched against the sender, recipients, and subject (substring)
and the message body (tokenized full text). Spam, trash, blocked, and
unauthenticated messages are always excluded. `limit` cannot exceed 100.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .search(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesMessagesSearchQueryRequest {
                q: Query("q".to_string()),
                limit: None,
                page_token: None,
                before: None,
                after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**q:** `Query` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">get</a>(inbox_id: InboxesInboxId, message_id: MessageId) -> Result&lt;Message, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:messages get --inbox-id <inbox_id> --message-id <message_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .get(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">delete</a>(inbox_id: InboxesInboxId, message_id: MessageId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Permanently deletes a message.

**CLI:**
```bash
agentmail inboxes:messages delete --inbox-id <inbox_id> --message-id <message_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .delete(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">update</a>(inbox_id: InboxesInboxId, message_id: MessageId, request: UpdateMessageRequest) -> Result&lt;UpdateMessageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:messages update --inbox-id <inbox_id> --message-id <message_id> --add-label read --remove-label unread
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .update(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            &UpdateMessageRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">batch_get</a>(inbox_id: InboxesInboxId, request: BatchGetMessagesRequest) -> Result&lt;BatchGetMessagesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetch metadata for up to 500 messages in one request. Missing or
restricted IDs are silently omitted; compare `count` against `limit`
to detect misses.

**CLI:**
```bash
agentmail inboxes:messages batch-get --inbox-id <inbox_id> --message-id <id1> --message-id <id2>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .batch_get(
            &InboxesInboxID("inbox_id".to_string()),
            &BatchGetMessagesRequest {
                message_ids: BatchGetMessagesMessageIDs(vec![MessageID("message_ids".to_string())]),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_ids:** `BatchGetMessagesMessageIds` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">batch_update</a>(inbox_id: InboxesInboxId, request: BatchUpdateMessagesRequest) -> Result&lt;BatchUpdateMessagesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Apply one label change to up to 50 messages in a single request. The
same add_labels and remove_labels apply to every message id, and at
least one of them must be provided. The update is atomic: either all
resolved messages are updated or none are. Missing or restricted ids
are silently excluded; compare `count` against `limit` to detect
exclusions.

**CLI:**
```bash
agentmail inboxes:messages batch-update --inbox-id <inbox_id> --message-id <id1> --message-id <id2> --add-label read --remove-label unread
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .batch_update(
            &InboxesInboxID("inbox_id".to_string()),
            &BatchUpdateMessagesRequest {
                message_ids: BatchUpdateMessagesMessageIDs(vec![MessageID(
                    "message_ids".to_string(),
                )]),
                add_labels: None,
                remove_labels: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_ids:** `BatchUpdateMessagesMessageIds` 
    
</dd>
</dl>

<dl>
<dd>

**add_labels:** `Option<UpdateMessageLabels>` — Label or labels to add to every message.
    
</dd>
</dl>

<dl>
<dd>

**remove_labels:** `Option<UpdateMessageLabels>` — Label or labels to remove from every message.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">get_attachment</a>(inbox_id: InboxesInboxId, message_id: MessageId, attachment_id: AttachmentId) -> Result&lt;AttachmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:messages get-attachment --inbox-id <inbox_id> --message-id <message_id> --attachment-id <attachment_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .get_attachment(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            &AttachmentID("attachment_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `AttachmentId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">get_raw</a>(inbox_id: InboxesInboxId, message_id: MessageId) -> Result&lt;RawMessageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:messages get-raw --inbox-id <inbox_id> --message-id <message_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .get_raw(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">send</a>(inbox_id: InboxesInboxId, request: SendMessageRequest) -> Result&lt;SendMessageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:messages send --inbox-id <inbox_id> --to recipient@example.com --subject "Hello" --text "Body"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .send(
            &InboxesInboxID("inbox_id".to_string()),
            &SendMessageRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">reply</a>(inbox_id: InboxesInboxId, message_id: MessageId, request: ReplyToMessageRequest) -> Result&lt;SendMessageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:messages reply --inbox-id <inbox_id> --message-id <message_id> --text "Reply text"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .reply(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            &ReplyToMessageRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<MessageLabels>` 
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<SendMessageReplyTo>` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<SendMessageTo>` 
    
</dd>
</dl>

<dl>
<dd>

**cc:** `Option<SendMessageCc>` 
    
</dd>
</dl>

<dl>
<dd>

**bcc:** `Option<SendMessageBcc>` 
    
</dd>
</dl>

<dl>
<dd>

**reply_all:** `Option<ReplyAll>` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `Option<MessageText>` 
    
</dd>
</dl>

<dl>
<dd>

**html:** `Option<MessageHtml>` 
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<SendMessageAttachments>` 
    
</dd>
</dl>

<dl>
<dd>

**headers:** `Option<SendMessageHeaders>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">reply_all</a>(inbox_id: InboxesInboxId, message_id: MessageId, request: ReplyAllMessageRequest) -> Result&lt;SendMessageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:messages reply-all --inbox-id <inbox_id> --message-id <message_id> --text "Reply text"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .reply_all(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            &ReplyAllMessageRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<MessageLabels>` 
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<SendMessageReplyTo>` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `Option<MessageText>` 
    
</dd>
</dl>

<dl>
<dd>

**html:** `Option<MessageHtml>` 
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<SendMessageAttachments>` 
    
</dd>
</dl>

<dl>
<dd>

**headers:** `Option<SendMessageHeaders>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">forward</a>(inbox_id: InboxesInboxId, message_id: MessageId, request: SendMessageRequest) -> Result&lt;SendMessageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:messages forward --inbox-id <inbox_id> --message-id <message_id> --to recipient@example.com
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .forward(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            &SendMessageRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">draft_reply</a>(inbox_id: InboxesInboxId, message_id: MessageId, request: CreateDraftReplyRequest) -> Result&lt;Draft, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a draft that replies to a message instead of sending it. The
recipients, subject, and threading are derived from the source message.
Send it later with `Send Draft`.

**CLI:**
```bash
agentmail inboxes:messages draft-reply --inbox-id <inbox_id> --message-id <message_id> --text "Reply text"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .draft_reply(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            &CreateDraftReplyRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<DraftLabels>` 
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<DraftReplyTo>` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<DraftTo>` 
    
</dd>
</dl>

<dl>
<dd>

**cc:** `Option<DraftCc>` 
    
</dd>
</dl>

<dl>
<dd>

**bcc:** `Option<DraftBcc>` 
    
</dd>
</dl>

<dl>
<dd>

**reply_all:** `Option<DraftReplyAll>` 
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<DraftSubject>` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `Option<DraftText>` 
    
</dd>
</dl>

<dl>
<dd>

**html:** `Option<DraftHtml>` 
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Option<Vec<SendAttachment>>>` — Attachments to include in draft.
    
</dd>
</dl>

<dl>
<dd>

**send_at:** `Option<DraftSendAt>` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<DraftClientId>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">draft_reply_all</a>(inbox_id: InboxesInboxId, message_id: MessageId, request: CreateDraftReplyAllRequest) -> Result&lt;Draft, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a draft that replies to every recipient of a message instead of
sending it. Recipients, subject, and threading are derived from the
source message. Send it later with `Send Draft`.

**CLI:**
```bash
agentmail inboxes:messages draft-reply-all --inbox-id <inbox_id> --message-id <message_id> --text "Reply text"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .draft_reply_all(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            &CreateDraftReplyAllRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<DraftLabels>` 
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<DraftReplyTo>` 
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<DraftSubject>` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `Option<DraftText>` 
    
</dd>
</dl>

<dl>
<dd>

**html:** `Option<DraftHtml>` 
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Option<Vec<SendAttachment>>>` — Attachments to include in draft.
    
</dd>
</dl>

<dl>
<dd>

**send_at:** `Option<DraftSendAt>` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<DraftClientId>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().messages.<a href="/src/api/resources/inboxes/messages/client.rs">draft_forward</a>(inbox_id: InboxesInboxId, message_id: MessageId, request: CreateDraftForwardRequest) -> Result&lt;Draft, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a draft that forwards a message instead of sending it. The subject
and threading are derived from the source message, whose body and
attachments are merged in at send time. Send it later with `Send Draft`.

**CLI:**
```bash
agentmail inboxes:messages draft-forward --inbox-id <inbox_id> --message-id <message_id> --to recipient@example.com
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .messages
        .draft_forward(
            &InboxesInboxID("inbox_id".to_string()),
            &MessageID("message_id".to_string()),
            &CreateDraftForwardRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**message_id:** `MessageId` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<DraftLabels>` 
    
</dd>
</dl>

<dl>
<dd>

**reply_to:** `Option<DraftReplyTo>` 
    
</dd>
</dl>

<dl>
<dd>

**to:** `Option<DraftTo>` 
    
</dd>
</dl>

<dl>
<dd>

**cc:** `Option<DraftCc>` 
    
</dd>
</dl>

<dl>
<dd>

**bcc:** `Option<DraftBcc>` 
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<DraftSubject>` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `Option<DraftText>` 
    
</dd>
</dl>

<dl>
<dd>

**html:** `Option<DraftHtml>` 
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Option<Vec<SendAttachment>>>` — Attachments to include in draft.
    
</dd>
</dl>

<dl>
<dd>

**send_at:** `Option<DraftSendAt>` 
    
</dd>
</dl>

<dl>
<dd>

**client_id:** `Option<DraftClientId>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Inboxes Metrics
<details><summary><code>client.inboxes().metrics.<a href="/src/api/resources/inboxes/metrics/client.rs">query_events</a>(inbox_id: InboxesInboxId, start: Option&lt;Option&lt;Start&gt;&gt;, end: Option&lt;Option&lt;End&gt;&gt;, period: Option&lt;Option&lt;Period&gt;&gt;, limit: Option&lt;Option&lt;MetricLimit&gt;&gt;, descending: Option&lt;Option&lt;Descending&gt;&gt;) -> Result&lt;QueryMetricsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Counts of email events (sent, delivered, bounced, etc.) over time for
the inbox. Defaults to the last 24 hours; `start` must be within the
last 90 days, and a future `end` is clamped to now. Omit `period` for
individual event counts, or set it to sum counts into buckets of that
many seconds.

**CLI:**
```bash
agentmail inboxes:metrics query --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .metrics
        .query_events(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesMetricsQueryEventsQueryRequest {
                event_types: vec![],
                start: None,
                end: None,
                period: None,
                limit: None,
                descending: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**event_types:** `Option<MetricEventType>` 
    
</dd>
</dl>

<dl>
<dd>

**start:** `Option<Start>` 
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<End>` 
    
</dd>
</dl>

<dl>
<dd>

**period:** `Option<Period>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<MetricLimit>` 
    
</dd>
</dl>

<dl>
<dd>

**descending:** `Option<Descending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().metrics.<a href="/src/api/resources/inboxes/metrics/client.rs">query_usage</a>(inbox_id: InboxesInboxId, start: Option&lt;Option&lt;Start&gt;&gt;, end: Option&lt;Option&lt;End&gt;&gt;, period: Option&lt;Option&lt;Period&gt;&gt;, limit: Option&lt;Option&lt;MetricLimit&gt;&gt;, descending: Option&lt;Option&lt;Descending&gt;&gt;) -> Result&lt;QueryUsageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cumulative usage series for the inbox. Each point is the running total
of the usage type at that timestamp, not the change within the bucket.
Inbox-scoped queries carry `storage_bytes`, `message_count`, and
`thread_count`; requested types that don't apply to the scope are
ignored. Defaults to the last 24 hours; `start` must be within the
last 90 days, and a future `end` is clamped to now. The range divided
by `period` must not exceed 1000 buckets.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .metrics
        .query_usage(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesMetricsQueryUsageQueryRequest {
                usage_types: vec![],
                start: None,
                end: None,
                period: None,
                limit: None,
                descending: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**usage_types:** `Option<UsageType>` 
    
</dd>
</dl>

<dl>
<dd>

**start:** `Option<Start>` 
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<End>` 
    
</dd>
</dl>

<dl>
<dd>

**period:** `Option<Period>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<MetricLimit>` 
    
</dd>
</dl>

<dl>
<dd>

**descending:** `Option<Descending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Inboxes Threads
<details><summary><code>client.inboxes().threads.<a href="/src/api/resources/inboxes/threads/client.rs">list</a>(inbox_id: InboxesInboxId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;, include_spam: Option&lt;Option&lt;IncludeSpam&gt;&gt;, include_blocked: Option&lt;Option&lt;IncludeBlocked&gt;&gt;, include_unauthenticated: Option&lt;Option&lt;IncludeUnauthenticated&gt;&gt;, include_trash: Option&lt;Option&lt;IncludeTrash&gt;&gt;, senders: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;, recipients: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;, subject: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;) -> Result&lt;ListThreadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists threads in the inbox, most recent first. Pass `senders`,
`recipients`, or `subject` to filter by substring. Filtered requests are
served by search, which caps `limit` at 100. For relevance-ranked
full-text search, use `Search Threads`.

**CLI:**
```bash
agentmail inboxes:threads list --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .threads
        .list(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesThreadsListQueryRequest {
                limit: None,
                page_token: None,
                labels: vec![],
                before: None,
                after: None,
                ascending: None,
                include_spam: None,
                include_blocked: None,
                include_unauthenticated: None,
                include_trash: None,
                senders: None,
                recipients: None,
                subject: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>

<dl>
<dd>

**include_spam:** `Option<IncludeSpam>` 
    
</dd>
</dl>

<dl>
<dd>

**include_blocked:** `Option<IncludeBlocked>` 
    
</dd>
</dl>

<dl>
<dd>

**include_unauthenticated:** `Option<IncludeUnauthenticated>` 
    
</dd>
</dl>

<dl>
<dd>

**include_trash:** `Option<IncludeTrash>` 
    
</dd>
</dl>

<dl>
<dd>

**senders:** `Option<Option<Vec<String>>>` — Filter to threads whose senders contain this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `Option<Option<Vec<String>>>` — Filter to threads whose recipients contain this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<Option<Vec<String>>>` — Filter to threads whose subject contains this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().threads.<a href="/src/api/resources/inboxes/threads/client.rs">search</a>(inbox_id: InboxesInboxId, q: Option&lt;Query&gt;, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;) -> Result&lt;SearchThreadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Full-text search across threads in the inbox, ranked by relevance. The
query is matched against senders, recipients, and subject (substring)
and the message body (tokenized full text). Spam, trash, blocked, and
unauthenticated threads are always excluded. `limit` cannot exceed 100.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .threads
        .search(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesThreadsSearchQueryRequest {
                q: Query("q".to_string()),
                limit: None,
                page_token: None,
                before: None,
                after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**q:** `Query` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().threads.<a href="/src/api/resources/inboxes/threads/client.rs">get</a>(inbox_id: InboxesInboxId, thread_id: ThreadId) -> Result&lt;Thread, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:threads get --inbox-id <inbox_id> --thread-id <thread_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .threads
        .get(
            &InboxesInboxID("inbox_id".to_string()),
            &ThreadID("thread_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().threads.<a href="/src/api/resources/inboxes/threads/client.rs">delete</a>(inbox_id: InboxesInboxId, thread_id: ThreadId, permanent: Option&lt;Option&lt;Option&lt;bool&gt;&gt;&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Moves the thread to trash by adding a trash label to all messages. If the thread is already in trash, it will be permanently deleted. Use `permanent=true` to force permanent deletion.

**CLI:**
```bash
agentmail inboxes:threads delete --inbox-id <inbox_id> --thread-id <thread_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .threads
        .delete(
            &InboxesInboxID("inbox_id".to_string()),
            &ThreadID("thread_id".to_string()),
            &InboxesThreadsDeleteQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>

<dl>
<dd>

**permanent:** `Option<Option<bool>>` — If true, permanently delete the thread instead of moving to trash.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().threads.<a href="/src/api/resources/inboxes/threads/client.rs">update</a>(inbox_id: InboxesInboxId, thread_id: ThreadId, request: UpdateThreadRequest) -> Result&lt;UpdateThreadResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates thread labels. Cannot add or remove system labels (sent, received, bounced, etc.). Rejects requests with a `422` for threads with 100 or more messages.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .threads
        .update(
            &InboxesInboxID("inbox_id".to_string()),
            &ThreadID("thread_id".to_string()),
            &UpdateThreadRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().threads.<a href="/src/api/resources/inboxes/threads/client.rs">get_attachment</a>(inbox_id: InboxesInboxId, thread_id: ThreadId, attachment_id: AttachmentId) -> Result&lt;AttachmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:threads get-attachment --inbox-id <inbox_id> --thread-id <thread_id> --attachment-id <attachment_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .threads
        .get_attachment(
            &InboxesInboxID("inbox_id".to_string()),
            &ThreadID("thread_id".to_string()),
            &AttachmentID("attachment_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `AttachmentId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Inboxes Webhooks
<details><summary><code>client.inboxes().webhooks.<a href="/src/api/resources/inboxes/webhooks/client.rs">list</a>(inbox_id: InboxesInboxId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;WebhooksListWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:webhooks list --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .webhooks
        .list(
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesWebhooksListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().webhooks.<a href="/src/api/resources/inboxes/webhooks/client.rs">create</a>(inbox_id: InboxesInboxId, request: WebhooksCreateInboxWebhookRequest) -> Result&lt;WebhooksWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a webhook scoped to this inbox.

**CLI:**
```bash
agentmail inboxes:webhooks create --inbox-id <inbox_id> --url https://example.com/webhook --event-type message.received
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .webhooks
        .create(
            &InboxesInboxID("inbox_id".to_string()),
            &WebhooksCreateInboxWebhookRequest {
                url: WebhooksURL("url".to_string()),
                event_types: WebhooksCreateWebhookEventTypes(EventTypes(vec![
                    EventType::MessageReceived,
                ])),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().webhooks.<a href="/src/api/resources/inboxes/webhooks/client.rs">get</a>(inbox_id: InboxesInboxId, webhook_id: WebhooksWebhookId) -> Result&lt;WebhooksWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:webhooks get --inbox-id <inbox_id> --webhook-id <webhook_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .webhooks
        .get(
            &InboxesInboxID("inbox_id".to_string()),
            &WebhooksWebhookID("webhook_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `WebhooksWebhookId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().webhooks.<a href="/src/api/resources/inboxes/webhooks/client.rs">delete</a>(inbox_id: InboxesInboxId, webhook_id: WebhooksWebhookId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:webhooks delete --inbox-id <inbox_id> --webhook-id <webhook_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .webhooks
        .delete(
            &InboxesInboxID("inbox_id".to_string()),
            &WebhooksWebhookID("webhook_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `WebhooksWebhookId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.inboxes().webhooks.<a href="/src/api/resources/inboxes/webhooks/client.rs">update</a>(inbox_id: InboxesInboxId, webhook_id: WebhooksWebhookId, request: WebhooksUpdateInboxWebhookRequest) -> Result&lt;WebhooksWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail inboxes:webhooks update --inbox-id <inbox_id> --webhook-id <webhook_id> --event-type message.received
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .inboxes
        .webhooks
        .update(
            &InboxesInboxID("inbox_id".to_string()),
            &WebhooksWebhookID("webhook_id".to_string()),
            &WebhooksUpdateInboxWebhookRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `WebhooksWebhookId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Pods ApiKeys
<details><summary><code>client.pods().api_keys.<a href="/src/api/resources/pods/api_keys/client.rs">list</a>(pod_id: PodsPodId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;) -> Result&lt;ListApiKeysResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:api-keys list --pod-id <pod_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .api_keys
        .list(
            &PodsPodID("pod_id".to_string()),
            &PodsAPIKeysListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().api_keys.<a href="/src/api/resources/pods/api_keys/client.rs">create</a>(pod_id: PodsPodId, request: CreateApiKeyRequest) -> Result&lt;CreateApiKeyResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:api-keys create --pod-id <pod_id> --name "My Key"
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .api_keys
        .create(
            &PodsPodID("pod_id".to_string()),
            &CreateAPIKeyRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().api_keys.<a href="/src/api/resources/pods/api_keys/client.rs">delete</a>(pod_id: PodsPodId, api_key_id: ApiKeyId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:api-keys delete --pod-id <pod_id> --api-key-id <api_key_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .api_keys
        .delete(
            &PodsPodID("pod_id".to_string()),
            &APIKeyID("api_key_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**api_key_id:** `ApiKeyId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Pods Domains
<details><summary><code>client.pods().domains.<a href="/src/api/resources/pods/domains/client.rs">list</a>(pod_id: PodsPodId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;ListDomainsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:domains list --pod-id <pod_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .domains
        .list(
            &PodsPodID("pod_id".to_string()),
            &PodsDomainsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().domains.<a href="/src/api/resources/pods/domains/client.rs">create</a>(pod_id: PodsPodId, request: CreateDomainRequest) -> Result&lt;Domain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:domains create --pod-id <pod_id> --domain example.com
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .domains
        .create(
            &PodsPodID("pod_id".to_string()),
            &CreateDomainRequest {
                domain: DomainName("domain".to_string()),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().domains.<a href="/src/api/resources/pods/domains/client.rs">get</a>(pod_id: PodsPodId, domain_id: DomainId) -> Result&lt;Domain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:domains get --pod-id <pod_id> --domain-id <domain_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .domains
        .get(
            &PodsPodID("pod_id".to_string()),
            &DomainID("domain_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**domain_id:** `DomainId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().domains.<a href="/src/api/resources/pods/domains/client.rs">delete</a>(pod_id: PodsPodId, domain_id: DomainId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:domains delete --pod-id <pod_id> --domain-id <domain_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .domains
        .delete(
            &PodsPodID("pod_id".to_string()),
            &DomainID("domain_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**domain_id:** `DomainId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().domains.<a href="/src/api/resources/pods/domains/client.rs">update</a>(pod_id: PodsPodId, domain_id: DomainId, request: UpdateDomainRequest) -> Result&lt;Domain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:domains update --pod-id <pod_id> --domain-id <domain_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .domains
        .update(
            &PodsPodID("pod_id".to_string()),
            &DomainID("domain_id".to_string()),
            &UpdateDomainRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**domain_id:** `DomainId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().domains.<a href="/src/api/resources/pods/domains/client.rs">get_zone_file</a>(pod_id: PodsPodId, domain_id: DomainId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:domains get-zone-file --pod-id <pod_id> --domain-id <domain_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .domains
        .get_zone_file(
            &PodsPodID("pod_id".to_string()),
            &DomainID("domain_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**domain_id:** `DomainId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().domains.<a href="/src/api/resources/pods/domains/client.rs">verify</a>(pod_id: PodsPodId, domain_id: DomainId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:domains verify --pod-id <pod_id> --domain-id <domain_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .domains
        .verify(
            &PodsPodID("pod_id".to_string()),
            &DomainID("domain_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**domain_id:** `DomainId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Pods Drafts
<details><summary><code>client.pods().drafts.<a href="/src/api/resources/pods/drafts/client.rs">list</a>(pod_id: PodsPodId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;ListDraftsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:drafts list --pod-id <pod_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .drafts
        .list(
            &PodsPodID("pod_id".to_string()),
            &PodsDraftsListQueryRequest {
                limit: None,
                page_token: None,
                labels: vec![],
                before: None,
                after: None,
                ascending: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().drafts.<a href="/src/api/resources/pods/drafts/client.rs">get</a>(pod_id: PodsPodId, draft_id: DraftId) -> Result&lt;Draft, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:drafts get --pod-id <pod_id> --draft-id <draft_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .drafts
        .get(
            &PodsPodID("pod_id".to_string()),
            &DraftID("draft_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**draft_id:** `DraftId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().drafts.<a href="/src/api/resources/pods/drafts/client.rs">get_attachment</a>(pod_id: PodsPodId, draft_id: DraftId, attachment_id: AttachmentId) -> Result&lt;AttachmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:drafts get-attachment --pod-id <pod_id> --draft-id <draft_id> --attachment-id <attachment_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .drafts
        .get_attachment(
            &PodsPodID("pod_id".to_string()),
            &DraftID("draft_id".to_string()),
            &AttachmentID("attachment_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**draft_id:** `DraftId` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `AttachmentId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Pods Inboxes
<details><summary><code>client.pods().inboxes.<a href="/src/api/resources/pods/inboxes/client.rs">list</a>(pod_id: PodsPodId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;InboxesListInboxesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:inboxes list --pod-id <pod_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .inboxes
        .list(
            &PodsPodID("pod_id".to_string()),
            &PodsInboxesListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().inboxes.<a href="/src/api/resources/pods/inboxes/client.rs">create</a>(pod_id: PodsPodId, request: InboxesCreateInboxRequest) -> Result&lt;InboxesInbox, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:inboxes create --pod-id <pod_id> --username myagent --domain example.com
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .inboxes
        .create(
            &PodsPodID("pod_id".to_string()),
            &InboxesCreateInboxRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().inboxes.<a href="/src/api/resources/pods/inboxes/client.rs">get</a>(pod_id: PodsPodId, inbox_id: InboxesInboxId) -> Result&lt;InboxesInbox, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:inboxes get --pod-id <pod_id> --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .inboxes
        .get(
            &PodsPodID("pod_id".to_string()),
            &InboxesInboxID("inbox_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().inboxes.<a href="/src/api/resources/pods/inboxes/client.rs">delete</a>(pod_id: PodsPodId, inbox_id: InboxesInboxId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:inboxes delete --pod-id <pod_id> --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .inboxes
        .delete(
            &PodsPodID("pod_id".to_string()),
            &InboxesInboxID("inbox_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().inboxes.<a href="/src/api/resources/pods/inboxes/client.rs">update</a>(pod_id: PodsPodId, inbox_id: InboxesInboxId, request: InboxesUpdateInboxRequest) -> Result&lt;InboxesInbox, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:inboxes update --pod-id <pod_id> --inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .inboxes
        .update(
            &PodsPodID("pod_id".to_string()),
            &InboxesInboxID("inbox_id".to_string()),
            &InboxesUpdateInboxRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**inbox_id:** `InboxesInboxId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Pods Lists
<details><summary><code>client.pods().lists.<a href="/src/api/resources/pods/lists/client.rs">list</a>(pod_id: PodsPodId, direction: Direction, type_: ListType, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;) -> Result&lt;PodListListEntriesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:lists list --pod-id <pod_id> --direction <direction> --type <type>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .lists
        .list(
            &PodsPodID("pod_id".to_string()),
            &Direction::Send,
            &ListType::Allow,
            &PodsListsListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().lists.<a href="/src/api/resources/pods/lists/client.rs">create</a>(pod_id: PodsPodId, direction: Direction, type_: ListType, request: CreateListEntryRequest) -> Result&lt;PodListEntry, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:lists create --pod-id <pod_id> --direction <direction> --type <type> --entry user@example.com
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .lists
        .create(
            &PodsPodID("pod_id".to_string()),
            &Direction::Send,
            &ListType::Allow,
            &CreateListEntryRequest {
                entry: "entry".to_string(),
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().lists.<a href="/src/api/resources/pods/lists/client.rs">get</a>(pod_id: PodsPodId, direction: Direction, type_: ListType, entry: String) -> Result&lt;PodListEntry, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:lists get --pod-id <pod_id> --direction <direction> --type <type> --entry <entry>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .lists
        .get(
            &PodsPodID("pod_id".to_string()),
            &Direction::Send,
            &ListType::Allow,
            &"entry".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — Email address or domain.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().lists.<a href="/src/api/resources/pods/lists/client.rs">delete</a>(pod_id: PodsPodId, direction: Direction, type_: ListType, entry: String) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:lists delete --pod-id <pod_id> --direction <direction> --type <type> --entry <entry>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .lists
        .delete(
            &PodsPodID("pod_id".to_string()),
            &Direction::Send,
            &ListType::Allow,
            &"entry".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**direction:** `Direction` 
    
</dd>
</dl>

<dl>
<dd>

**type_:** `ListType` 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — Email address or domain.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Pods Metrics
<details><summary><code>client.pods().metrics.<a href="/src/api/resources/pods/metrics/client.rs">query_events</a>(pod_id: PodsPodId, start: Option&lt;Option&lt;Start&gt;&gt;, end: Option&lt;Option&lt;End&gt;&gt;, period: Option&lt;Option&lt;Period&gt;&gt;, limit: Option&lt;Option&lt;MetricLimit&gt;&gt;, descending: Option&lt;Option&lt;Descending&gt;&gt;) -> Result&lt;QueryMetricsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Counts of email events (sent, delivered, bounced, etc.) over time for
the pod. Defaults to the last 24 hours; `start` must be within the last
90 days, and a future `end` is clamped to now. Omit `period` for
individual event counts, or set it to sum counts into buckets of that
many seconds.

**CLI:**
```bash
agentmail pods:metrics query --pod-id <pod_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .metrics
        .query_events(
            &PodsPodID("pod_id".to_string()),
            &PodsMetricsQueryEventsQueryRequest {
                event_types: vec![],
                start: None,
                end: None,
                period: None,
                limit: None,
                descending: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**event_types:** `Option<MetricEventType>` 
    
</dd>
</dl>

<dl>
<dd>

**start:** `Option<Start>` 
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<End>` 
    
</dd>
</dl>

<dl>
<dd>

**period:** `Option<Period>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<MetricLimit>` 
    
</dd>
</dl>

<dl>
<dd>

**descending:** `Option<Descending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().metrics.<a href="/src/api/resources/pods/metrics/client.rs">query_usage</a>(pod_id: PodsPodId, start: Option&lt;Option&lt;Start&gt;&gt;, end: Option&lt;Option&lt;End&gt;&gt;, period: Option&lt;Option&lt;Period&gt;&gt;, limit: Option&lt;Option&lt;MetricLimit&gt;&gt;, descending: Option&lt;Option&lt;Descending&gt;&gt;) -> Result&lt;QueryUsageResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cumulative usage series for the pod. Each point is the running total of
the usage type at that timestamp, not the change within the bucket.
Pod-scoped queries carry every usage type except `pod_count`; requested
types that don't apply to the scope are ignored. Defaults to the last
24 hours; `start` must be within the last 90 days, and a future `end`
is clamped to now. The range divided by `period` must not exceed 1000
buckets.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .metrics
        .query_usage(
            &PodsPodID("pod_id".to_string()),
            &PodsMetricsQueryUsageQueryRequest {
                usage_types: vec![],
                start: None,
                end: None,
                period: None,
                limit: None,
                descending: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**usage_types:** `Option<UsageType>` 
    
</dd>
</dl>

<dl>
<dd>

**start:** `Option<Start>` 
    
</dd>
</dl>

<dl>
<dd>

**end:** `Option<End>` 
    
</dd>
</dl>

<dl>
<dd>

**period:** `Option<Period>` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<MetricLimit>` 
    
</dd>
</dl>

<dl>
<dd>

**descending:** `Option<Descending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Pods Threads
<details><summary><code>client.pods().threads.<a href="/src/api/resources/pods/threads/client.rs">list</a>(pod_id: PodsPodId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;, include_spam: Option&lt;Option&lt;IncludeSpam&gt;&gt;, include_blocked: Option&lt;Option&lt;IncludeBlocked&gt;&gt;, include_unauthenticated: Option&lt;Option&lt;IncludeUnauthenticated&gt;&gt;, include_trash: Option&lt;Option&lt;IncludeTrash&gt;&gt;, senders: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;, recipients: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;, subject: Option&lt;Option&lt;Option&lt;Vec&lt;String&gt;&gt;&gt;&gt;) -> Result&lt;ListThreadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists threads in the pod, most recent first. Pass `senders`,
`recipients`, or `subject` to filter by substring. Filtered requests are
served by search, which caps `limit` at 100. For relevance-ranked
full-text search, use `Search Threads`.

**CLI:**
```bash
agentmail pods:threads list --pod-id <pod_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .threads
        .list(
            &PodsPodID("pod_id".to_string()),
            &PodsThreadsListQueryRequest {
                limit: None,
                page_token: None,
                labels: vec![],
                before: None,
                after: None,
                ascending: None,
                include_spam: None,
                include_blocked: None,
                include_unauthenticated: None,
                include_trash: None,
                senders: None,
                recipients: None,
                subject: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**labels:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>

<dl>
<dd>

**include_spam:** `Option<IncludeSpam>` 
    
</dd>
</dl>

<dl>
<dd>

**include_blocked:** `Option<IncludeBlocked>` 
    
</dd>
</dl>

<dl>
<dd>

**include_unauthenticated:** `Option<IncludeUnauthenticated>` 
    
</dd>
</dl>

<dl>
<dd>

**include_trash:** `Option<IncludeTrash>` 
    
</dd>
</dl>

<dl>
<dd>

**senders:** `Option<Option<Vec<String>>>` — Filter to threads whose senders contain this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>

<dl>
<dd>

**recipients:** `Option<Option<Vec<String>>>` — Filter to threads whose recipients contain this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>

<dl>
<dd>

**subject:** `Option<Option<Vec<String>>>` — Filter to threads whose subject contains this value (substring match). Repeatable; all values must match.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().threads.<a href="/src/api/resources/pods/threads/client.rs">search</a>(pod_id: PodsPodId, q: Option&lt;Query&gt;, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, before: Option&lt;Option&lt;Before&gt;&gt;, after: Option&lt;Option&lt;After&gt;&gt;) -> Result&lt;SearchThreadsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Full-text search across threads in the pod, ranked by relevance. The
query is matched against senders, recipients, and subject (substring)
and the message body (tokenized full text). Spam, trash, blocked, and
unauthenticated threads are always excluded. `limit` cannot exceed 100.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .threads
        .search(
            &PodsPodID("pod_id".to_string()),
            &PodsThreadsSearchQueryRequest {
                q: Query("q".to_string()),
                limit: None,
                page_token: None,
                before: None,
                after: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**q:** `Query` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**before:** `Option<Before>` 
    
</dd>
</dl>

<dl>
<dd>

**after:** `Option<After>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().threads.<a href="/src/api/resources/pods/threads/client.rs">get</a>(pod_id: PodsPodId, thread_id: ThreadId) -> Result&lt;Thread, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:threads get --pod-id <pod_id> --thread-id <thread_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .threads
        .get(
            &PodsPodID("pod_id".to_string()),
            &ThreadID("thread_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().threads.<a href="/src/api/resources/pods/threads/client.rs">delete</a>(pod_id: PodsPodId, thread_id: ThreadId, permanent: Option&lt;Option&lt;Option&lt;bool&gt;&gt;&gt;) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Moves the thread to trash by adding a trash label to all messages. If the thread is already in trash, it will be permanently deleted. Use `permanent=true` to force permanent deletion.

**CLI:**
```bash
agentmail pods:threads delete --pod-id <pod_id> --thread-id <thread_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .threads
        .delete(
            &PodsPodID("pod_id".to_string()),
            &ThreadID("thread_id".to_string()),
            &PodsThreadsDeleteQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>

<dl>
<dd>

**permanent:** `Option<Option<bool>>` — If true, permanently delete the thread instead of moving to trash.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().threads.<a href="/src/api/resources/pods/threads/client.rs">update</a>(pod_id: PodsPodId, thread_id: ThreadId, request: UpdateThreadRequest) -> Result&lt;UpdateThreadResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates thread labels. Cannot add or remove system labels (sent, received, bounced, etc.). Rejects requests with a `422` for threads with 100 or more messages.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .threads
        .update(
            &PodsPodID("pod_id".to_string()),
            &ThreadID("thread_id".to_string()),
            &UpdateThreadRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().threads.<a href="/src/api/resources/pods/threads/client.rs">get_attachment</a>(pod_id: PodsPodId, thread_id: ThreadId, attachment_id: AttachmentId) -> Result&lt;AttachmentResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:threads get-attachment --pod-id <pod_id> --thread-id <thread_id> --attachment-id <attachment_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .threads
        .get_attachment(
            &PodsPodID("pod_id".to_string()),
            &ThreadID("thread_id".to_string()),
            &AttachmentID("attachment_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**thread_id:** `ThreadId` 
    
</dd>
</dl>

<dl>
<dd>

**attachment_id:** `AttachmentId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Pods Webhooks
<details><summary><code>client.pods().webhooks.<a href="/src/api/resources/pods/webhooks/client.rs">list</a>(pod_id: PodsPodId, limit: Option&lt;Option&lt;Limit&gt;&gt;, page_token: Option&lt;Option&lt;PageToken&gt;&gt;, ascending: Option&lt;Option&lt;Ascending&gt;&gt;) -> Result&lt;WebhooksListWebhooksResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:webhooks list --pod-id <pod_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .webhooks
        .list(
            &PodsPodID("pod_id".to_string()),
            &PodsWebhooksListQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Limit>` 
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<PageToken>` 
    
</dd>
</dl>

<dl>
<dd>

**ascending:** `Option<Ascending>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().webhooks.<a href="/src/api/resources/pods/webhooks/client.rs">create</a>(pod_id: PodsPodId, request: WebhooksCreatePodWebhookRequest) -> Result&lt;WebhooksWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a webhook scoped to this pod.

**CLI:**
```bash
agentmail pods:webhooks create --pod-id <pod_id> --url https://example.com/webhook --event-type message.received
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .webhooks
        .create(
            &PodsPodID("pod_id".to_string()),
            &WebhooksCreatePodWebhookRequest {
                webhooks_create_inbox_webhook_request_fields: WebhooksCreateInboxWebhookRequest {
                    url: WebhooksURL("url".to_string()),
                    event_types: WebhooksCreateWebhookEventTypes(EventTypes(vec![
                        EventType::MessageReceived,
                    ])),
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().webhooks.<a href="/src/api/resources/pods/webhooks/client.rs">get</a>(pod_id: PodsPodId, webhook_id: WebhooksWebhookId) -> Result&lt;WebhooksWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:webhooks get --pod-id <pod_id> --webhook-id <webhook_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .webhooks
        .get(
            &PodsPodID("pod_id".to_string()),
            &WebhooksWebhookID("webhook_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `WebhooksWebhookId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().webhooks.<a href="/src/api/resources/pods/webhooks/client.rs">delete</a>(pod_id: PodsPodId, webhook_id: WebhooksWebhookId) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:webhooks delete --pod-id <pod_id> --webhook-id <webhook_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .webhooks
        .delete(
            &PodsPodID("pod_id".to_string()),
            &WebhooksWebhookID("webhook_id".to_string()),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `WebhooksWebhookId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.pods().webhooks.<a href="/src/api/resources/pods/webhooks/client.rs">update</a>(pod_id: PodsPodId, webhook_id: WebhooksWebhookId, request: WebhooksUpdatePodWebhookRequest) -> Result&lt;WebhooksWebhook, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

**CLI:**
```bash
agentmail pods:webhooks update --pod-id <pod_id> --webhook-id <webhook_id> --add-inbox-id <inbox_id>
```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use agentmail_sdk::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = AgentmailClient::new(config).expect("Failed to build client");
    client
        .pods
        .webhooks
        .update(
            &PodsPodID("pod_id".to_string()),
            &WebhooksWebhookID("webhook_id".to_string()),
            &WebhooksUpdatePodWebhookRequest {
                webhooks_update_inbox_webhook_request_fields: WebhooksUpdateInboxWebhookRequest {
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pod_id:** `PodsPodId` 
    
</dd>
</dl>

<dl>
<dd>

**webhook_id:** `WebhooksWebhookId` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

