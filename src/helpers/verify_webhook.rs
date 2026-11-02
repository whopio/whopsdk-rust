//! Verifies the Standard Webhooks signature Whop sends on every webhook
//! delivery, then parses the body.
//!
//! This is the verification half of the `client.webhooks.unwrap` the
//! Stainless-generated SDKs shipped. Fern generates from OpenAPI paths and
//! `unwrap` was never a path, so the generated client has no equivalent. It is
//! a standalone function rather than a method on [`crate::Whop`] so that
//! nothing generated has to be patched.
//!
//! What it does NOT do, and the Stainless version did: coerce the parsed body
//! into a typed event model. Fern generates no webhook event models —
//! `WebhookEvent` is the enum of event *names* an endpoint subscribes to, not a
//! payload type — so there is nothing to coerce into, and the parsed
//! [`serde_json::Value`] is returned as-is.

use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;

/// How far a `webhook-timestamp` may sit from the current time before the
/// delivery is refused, in seconds. Whop signs the timestamp into the payload,
/// so this window is what bounds a replay.
pub const TOLERANCE_SECONDS: i64 = 5 * 60;

const ID_HEADER: &str = "webhook-id";
const TIMESTAMP_HEADER: &str = "webhook-timestamp";
const SIGNATURE_HEADER: &str = "webhook-signature";
const SIGNATURE_VERSION: &str = "v1";

/// Why a delivery was refused. Every variant means the body must not be
/// trusted.
#[derive(Debug, thiserror::Error)]
pub enum WebhookVerificationError {
    #[error("cannot verify a webhook without a key: pass the endpoint's `ws_…` signing secret")]
    MissingKey,

    #[error("the {0} header is missing or empty")]
    MissingHeader(&'static str),

    #[error("the webhook-timestamp header is not a unix timestamp: {0:?}")]
    MalformedTimestamp(String),

    #[error(
        "the webhook-timestamp is {drift_seconds}s {direction} the current time, \
         outside the ±{tolerance_seconds}s tolerance"
    )]
    TimestampOutsideTolerance {
        drift_seconds: i64,
        direction: &'static str,
        tolerance_seconds: i64,
    },

    #[error("no v1 entry in the webhook-signature header matches the body ({checked} checked)")]
    NoMatchingSignature { checked: usize },

    #[error("the signature verified but the body is not JSON: {0}")]
    MalformedBody(#[source] serde_json::Error),
}

/// Verifies `payload` against the signature headers and returns the parsed
/// body.
///
/// `payload` must be the raw, unmodified request body. Verifying a
/// re-serialized body fails: the signature covers the exact bytes sent.
///
/// `headers` is anything that iterates as name/value pairs — a `Vec`, a
/// `HashMap`, or `http::HeaderMap` mapped through `to_str`. Only `webhook-id`,
/// `webhook-timestamp` and `webhook-signature` are read, and the lookup is
/// case-insensitive.
///
/// `key` is the endpoint's signing secret exactly as Whop shows it — a
/// `ws_`-prefixed string. Pass it verbatim: do not strip the prefix and do not
/// pre-encode it. Whop's backend HMACs with the secret's literal bytes, prefix
/// included, which is where Standard Webhooks libraries go wrong — they
/// base64-*decode* whatever key they are handed, and so derive a different key
/// from the same string. This computes the HMAC directly and has nothing to
/// cancel out.
///
/// ```no_run
/// use whop_sdk::helpers::verify_webhook;
///
/// # fn example(body: &[u8], headers: Vec<(String, String)>, secret: &str) {
/// match verify_webhook(body, headers, secret) {
///     Ok(event) => println!("{}", event["type"]),
///     Err(err) => eprintln!("refused: {err}"),
/// }
/// # }
/// ```
pub fn verify_webhook<I, K, V>(
    payload: &[u8],
    headers: I,
    key: &str,
) -> Result<Value, WebhookVerificationError>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    if key.is_empty() {
        return Err(WebhookVerificationError::MissingKey);
    }

    let headers = index(headers);
    let id = header(&headers, ID_HEADER)?;
    let timestamp = header(&headers, TIMESTAMP_HEADER)?;
    let signatures = header(&headers, SIGNATURE_HEADER)?;

    check_timestamp(timestamp)?;

    let mut mac =
        Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("HMAC accepts a key of any length");
    mac.update(id.as_bytes());
    mac.update(b".");
    mac.update(timestamp.as_bytes());
    mac.update(b".");
    mac.update(payload);

    let mut checked = 0;
    for candidate in v1_signatures(signatures) {
        let Ok(raw) = STANDARD.decode(candidate) else {
            continue;
        };
        checked += 1;
        if mac.clone().verify_slice(&raw).is_ok() {
            return serde_json::from_slice(payload)
                .map_err(WebhookVerificationError::MalformedBody);
        }
    }

    Err(WebhookVerificationError::NoMatchingSignature { checked })
}

/// The header is a space-separated list of `<version>,<signature>` entries.
/// Whop sends one `v1` entry today; an unknown version is skipped rather than
/// refused, so a future version can be rolled out beside `v1` without breaking
/// a caller pinned to this release.
fn v1_signatures(header: &str) -> impl Iterator<Item = &str> {
    header
        .split_whitespace()
        .filter_map(|entry| entry.split_once(','))
        .filter(|(version, _)| *version == SIGNATURE_VERSION)
        .map(|(_, signature)| signature)
}

/// Refuses a delivery whose timestamp is outside the tolerance before any
/// signature is compared, so that a replayed delivery is refused *for its
/// timestamp* — back-dating also invalidates the signature, and a caller
/// reading the error deserves the reason that actually applies.
fn check_timestamp(raw: &str) -> Result<(), WebhookVerificationError> {
    let sent: i64 = raw
        .trim()
        .parse()
        .map_err(|_| WebhookVerificationError::MalformedTimestamp(raw.to_string()))?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since_epoch| since_epoch.as_secs() as i64)
        .unwrap_or_default();
    let drift = now - sent;

    if drift.abs() > TOLERANCE_SECONDS {
        return Err(WebhookVerificationError::TimestampOutsideTolerance {
            drift_seconds: drift.abs(),
            direction: if drift > 0 { "behind" } else { "ahead of" },
            tolerance_seconds: TOLERANCE_SECONDS,
        });
    }
    Ok(())
}

fn index<I, K, V>(headers: I) -> BTreeMap<String, String>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    headers
        .into_iter()
        .map(|(name, value)| {
            (
                name.as_ref().to_ascii_lowercase(),
                value.as_ref().to_string(),
            )
        })
        .collect()
}

fn header<'a>(
    headers: &'a BTreeMap<String, String>,
    name: &'static str,
) -> Result<&'a str, WebhookVerificationError> {
    headers
        .get(name)
        .map(String::as_str)
        .filter(|value| !value.is_empty())
        .ok_or(WebhookVerificationError::MissingHeader(name))
}

#[cfg(test)]
mod tests {
    // Every fixture here is signed the way `WebhooksManager::SignWebhook` signs
    // in whop-monorepo — a raw HMAC-SHA256 over `"<id>.<timestamp>.<body>"`
    // keyed on the literal bytes of the `ws_…` secret, base64'd, sent as
    // `v1,<signature>`. Nothing calls the code under test to build an input: a
    // helper that verifies its own output would agree with itself and say nothing
    // about the producer, which is exactly the trap
    // (`standardwebhooks` base64-*decodes* its key) this helper exists to avoid.

    use super::{verify_webhook, WebhookVerificationError, TOLERANCE_SECONDS};
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use std::time::{SystemTime, UNIX_EPOCH};

    const SECRET: &str = "ws_9f2c1d7b4e6a8039f5c2b1a7d4e6098357c1b2a4d6e809f3c5b7a1d2e4f60839";
    const OTHER_SECRET: &str =
        "ws_1a2b3c4d5e6f708192a3b4c5d6e7f80912a3b4c5d6e7f8091a2b3c4d5e6f7081";
    const ID: &str = "msg_2xNq7Zr4LbKp0aWvTcHm93Yd";

    /// Deliberately not canonical JSON: the keys are unsorted and the spacing is
    /// the backend's. Re-serializing it produces different bytes, which is what
    /// `reserialized_body_is_refused` turns into an assertion.
    const BODY: &str = r#"{"id":"msg_2xNq7Zr4LbKp0aWvTcHm93Yd","type":"product.created","data":{"id":"prod_7Hq2Lm9","title":"Detailing kit","price": 49.99},"api_version":"2026-08-01"}"#;

    fn now() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock is after the unix epoch")
            .as_secs() as i64
    }

    fn sign(id: &str, timestamp: i64, body: &[u8], secret: &str) -> String {
        let mut mac =
            Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC accepts any key length");
        mac.update(format!("{id}.{timestamp}.").as_bytes());
        mac.update(body);
        format!("v1,{}", STANDARD.encode(mac.finalize().into_bytes()))
    }

    /// The three headers a delivery carries, in the casing the backend sends.
    fn delivery(timestamp: i64, body: &[u8], secret: &str) -> Vec<(String, String)> {
        vec![
            ("webhook-id".to_string(), ID.to_string()),
            ("webhook-timestamp".to_string(), timestamp.to_string()),
            (
                "webhook-signature".to_string(),
                sign(ID, timestamp, body, secret),
            ),
        ]
    }

    fn without(headers: Vec<(String, String)>, name: &str) -> Vec<(String, String)> {
        headers
            .into_iter()
            .filter(|(header, _)| !header.eq_ignore_ascii_case(name))
            .collect()
    }

    fn expect_error(result: Result<serde_json::Value, WebhookVerificationError>) -> String {
        match result {
            Ok(body) => panic!("expected a refusal, got the parsed body: {body}"),
            Err(err) => err.to_string(),
        }
    }

    #[test]
    fn valid_signature_is_accepted_and_the_body_parsed() {
        let timestamp = now();
        let event = verify_webhook(
            BODY.as_bytes(),
            delivery(timestamp, BODY.as_bytes(), SECRET),
            SECRET,
        )
        .expect("a delivery signed the way the backend signs it");

        assert_eq!(event["id"], ID);
        assert_eq!(event["type"], "product.created");
        assert_eq!(event["data"]["id"], "prod_7Hq2Lm9");
    }

    /// The secret is used as the raw HMAC key, prefix and all. Handing the helper a
    /// base64 of the secret — what a Standard Webhooks library derives internally —
    /// must not verify, or the helper has the bug it exists to avoid.
    #[test]
    fn the_secret_is_the_raw_hmac_key() {
        let timestamp = now();
        let headers = delivery(timestamp, BODY.as_bytes(), SECRET);
        let encoded = STANDARD.encode(SECRET);

        let error = expect_error(verify_webhook(BODY.as_bytes(), headers, &encoded));
        assert!(error.contains("no v1 entry"), "{error}");
    }

    #[test]
    fn a_tampered_body_is_refused() {
        let timestamp = now();
        let headers = delivery(timestamp, BODY.as_bytes(), SECRET);
        let tampered = BODY.replace("Detailing kit", "Detailing k1t");
        assert_ne!(tampered, BODY);
        serde_json::from_str::<serde_json::Value>(&tampered).expect("still valid JSON");

        let error = expect_error(verify_webhook(tampered.as_bytes(), headers, SECRET));
        assert!(error.contains("no v1 entry"), "{error}");
    }

    /// The signature covers the exact bytes sent, so a body that has been through
    /// `serde_json` and back is a different message even though it is the same
    /// document.
    #[test]
    fn a_reserialized_body_is_refused() {
        let timestamp = now();
        let headers = delivery(timestamp, BODY.as_bytes(), SECRET);
        let reserialized =
            serde_json::to_string(&serde_json::from_str::<serde_json::Value>(BODY).unwrap())
                .unwrap();
        assert_ne!(reserialized, BODY);

        let error = expect_error(verify_webhook(reserialized.as_bytes(), headers, SECRET));
        assert!(error.contains("no v1 entry"), "{error}");
    }

    #[test]
    fn a_signature_from_another_secret_is_refused() {
        let timestamp = now();
        let headers = delivery(timestamp, BODY.as_bytes(), OTHER_SECRET);

        let error = expect_error(verify_webhook(BODY.as_bytes(), headers, SECRET));
        assert!(error.contains("no v1 entry"), "{error}");
    }

    #[test]
    fn an_empty_key_is_refused() {
        let timestamp = now();
        let headers = delivery(timestamp, BODY.as_bytes(), SECRET);

        let error = expect_error(verify_webhook(BODY.as_bytes(), headers, ""));
        assert!(error.contains("without a key"), "{error}");
    }

    #[test]
    fn each_missing_header_is_refused_by_name() {
        let timestamp = now();
        for name in ["webhook-id", "webhook-timestamp", "webhook-signature"] {
            let headers = without(delivery(timestamp, BODY.as_bytes(), SECRET), name);
            let error = expect_error(verify_webhook(BODY.as_bytes(), headers, SECRET));
            assert!(
                error.contains(name),
                "dropping {name} was refused as: {error}"
            );
        }
    }

    /// A back-dated delivery also fails the signature comparison, so "refused" on
    /// its own would not say which check fired. The tolerance has to be enforced
    /// first, and the error has to name the timestamp.
    #[test]
    fn a_stale_timestamp_is_refused_for_the_timestamp() {
        let stale = now() - TOLERANCE_SECONDS - 60;
        let headers = delivery(stale, BODY.as_bytes(), SECRET);

        let error = expect_error(verify_webhook(BODY.as_bytes(), headers, SECRET));
        assert!(error.contains("webhook-timestamp"), "{error}");
        assert!(error.contains("tolerance"), "{error}");
    }

    #[test]
    fn a_future_timestamp_is_refused_for_the_timestamp() {
        let ahead = now() + TOLERANCE_SECONDS + 60;
        let headers = delivery(ahead, BODY.as_bytes(), SECRET);

        let error = expect_error(verify_webhook(BODY.as_bytes(), headers, SECRET));
        assert!(error.contains("ahead of"), "{error}");
    }

    #[test]
    fn a_non_numeric_timestamp_is_refused() {
        let timestamp = now();
        let mut headers = delivery(timestamp, BODY.as_bytes(), SECRET);
        headers[1].1 = "not-a-timestamp".to_string();

        let error = expect_error(verify_webhook(BODY.as_bytes(), headers, SECRET));
        assert!(error.contains("not a unix timestamp"), "{error}");
    }

    /// The header can carry a space-separated list of versioned entries. An
    /// unknown version is skipped rather than refused, and the matching `v1` entry
    /// is found wherever it sits.
    #[test]
    fn the_valid_entry_is_found_behind_other_entries() {
        let timestamp = now();
        let valid = sign(ID, timestamp, BODY.as_bytes(), SECRET);
        let decoy = sign(ID, timestamp, b"something else", SECRET);
        let combined = format!(
            "v2,{} {decoy} {valid}",
            STANDARD.encode("a v2 nobody can read")
        );

        let headers = vec![
            ("webhook-id".to_string(), ID.to_string()),
            ("webhook-timestamp".to_string(), timestamp.to_string()),
            ("webhook-signature".to_string(), combined),
        ];

        let event =
            verify_webhook(BODY.as_bytes(), headers, SECRET).expect("the trailing v1 entry");
        assert_eq!(event["type"], "product.created");
    }

    /// HTTP header names are case-insensitive and a framework may hand them over
    /// however it likes; Whop sends them lowercase.
    #[test]
    fn header_casing_does_not_matter() {
        let timestamp = now();
        let headers = vec![
            ("Webhook-Id".to_string(), ID.to_string()),
            ("WEBHOOK-TIMESTAMP".to_string(), timestamp.to_string()),
            (
                "webhook-Signature".to_string(),
                sign(ID, timestamp, BODY.as_bytes(), SECRET),
            ),
        ];

        let event = verify_webhook(BODY.as_bytes(), headers, SECRET).expect("mixed-case headers");
        assert_eq!(event["id"], ID);
    }

    /// The signature is over the id too, so a delivery replayed under a different
    /// `webhook-id` is a different message.
    #[test]
    fn a_swapped_id_is_refused() {
        let timestamp = now();
        let mut headers = delivery(timestamp, BODY.as_bytes(), SECRET);
        headers[0].1 = "msg_0000000000000000000000".to_string();

        let error = expect_error(verify_webhook(BODY.as_bytes(), headers, SECRET));
        assert!(error.contains("no v1 entry"), "{error}");
    }
}
