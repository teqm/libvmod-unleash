varnish::boilerplate!();

mod tracing_subscriber_vsl;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD as base64_url, Engine};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use tracing::{warn, Level};
use unleash_client::unleash::{Unleash, UnleashBuilder};
use unleash_client::unleash_yggdrasil::{Context as UnleashContext, VariantDef};
use varnish::vcl::ctx::Ctx;

varnish::vtc!(test01);
varnish::vtc!(test02);
varnish::vtc!(test03);
varnish::vtc!(test04);
varnish::vtc!(test05);
varnish::vtc!(test06);

const EMPTY_STRING: String = String::new();

#[allow(non_camel_case_types)]
struct client {
    unleash_client: Arc<Unleash>,
}

fn try_props_from_str(s: &str) -> Result<HashMap<String, String>, ()> {
    s.split(",")
        .map(|s| {
            let parts: Vec<&str> = s.trim().split("=").take(2).collect();

            if parts.len() != 2 {
                return Err(());
            }

            Ok((parts[0].to_string(), parts[1].to_string()))
        })
        .collect()
}

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
}

fn decode_jwt(token: &str) -> Option<Claims> {
    let parts = token.split(".").take(3).collect::<Vec<_>>();

    if parts.len() != 3 {
        return None;
    }

    match base64_url.decode(parts[1]) {
        Ok(payload) => serde_json::from_slice::<Claims>(payload.as_slice()).ok(),
        Err(_) => None,
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

struct Context<'a> {
    user_id: Option<&'a str>,
    session_id: Option<&'a str>,
    environment: Option<&'a str>,
    app_name: Option<&'a str>,
    remote_address: Option<&'a str>,
    properties: Option<&'a str>,
    jwt: Option<&'a str>,
}

impl Default for Context<'_> {
    fn default() -> Self {
        Context {
            user_id: None,
            session_id: None,
            environment: None,
            app_name: None,
            remote_address: None,
            properties: None,
            jwt: None,
        }
    }
}

impl Into<UnleashContext> for Context<'_> {
    fn into(self) -> UnleashContext {
        UnleashContext {
            user_id: self.user_id.map(String::from).or_else(|| {
                self.jwt
                    .map(|jwt| decode_jwt(jwt).map(|claims| claims.sub))
                    .flatten()
            }),
            session_id: self.session_id.map(String::from),
            environment: self.environment.map(String::from),
            app_name: self.app_name.map(String::from),
            current_time: None,
            remote_address: self.remote_address.map(String::from),
            properties: self
                .properties
                .map(|s| {
                    try_props_from_str(s)
                        .map_err(|_| {
                            warn!(
                                "failed to parse `properties` argument: {:?}",
                                self.properties
                            );
                        })
                        .ok()
                })
                .flatten(),
        }
    }
}

#[derive(Serialize)]
struct ResolvedToggle<'a> {
    project: &'a String,
    variant: &'a VariantDef,
}

impl client {
    pub fn new(
        _ctx: &mut Ctx,
        _vcl_name: &str,
        url: &str,
        token: &str,
        log_level: &str,
        refresh_interval: Duration,
        disable_metrics: bool,
    ) -> Result<Self, String> {
        tracing_subscriber_vsl::set_as_global_default(
            log_level.parse::<Level>().unwrap_or(Level::WARN),
        );

        let unleash_client = Arc::new(
            UnleashBuilder::default()
                .refresh_interval(refresh_interval)
                .disable_metrics(disable_metrics)
                .build(
                    url.to_string(),
                    "libvmod-unleash".to_string(),
                    token.to_string(),
                ),
        );

        let rt = tokio::runtime::Runtime::new().unwrap();

        let poll_handle = unleash_client.clone();

        std::thread::spawn(move || {
            rt.block_on(async move {
                poll_handle.start().await;
            });
        });

        Ok(client { unleash_client })
    }

    pub fn get_hash(
        &self,
        _ctx: &mut Ctx,
        user_id: Option<&str>,
        session_id: Option<&str>,
        environment: Option<&str>,
        app_name: Option<&str>,
        remote_address: Option<&str>,
        properties: Option<&str>,
        jwt: Option<&str>,
    ) -> String {
        let context = Context {
            user_id,
            session_id,
            environment,
            app_name,
            remote_address,
            properties,
            jwt,
        };

        let toggles = match self.unleash_client.resolve_all(&context.into()) {
            Some(toggles) => toggles,
            None => {
                return EMPTY_STRING;
            }
        };

        let enabled_toggles = toggles
            .iter()
            .filter(|(_, toggle)| toggle.enabled)
            .map(|(name, toggle)| {
                (
                    name,
                    ResolvedToggle {
                        project: &toggle.project,
                        variant: &toggle.variant,
                    },
                )
            })
            .collect::<Vec<(_, _)>>();

        let serialized = serde_json::to_vec(&enabled_toggles).unwrap();

        format!("{:x}", calculate_hash(&serialized))
    }

    pub fn is_enabled(
        &self,
        _ctx: &mut Ctx,
        name: &str,
        user_id: Option<&str>,
        session_id: Option<&str>,
        environment: Option<&str>,
        app_name: Option<&str>,
        remote_address: Option<&str>,
        properties: Option<&str>,
        jwt: Option<&str>,
    ) -> bool {
        let context = Context {
            user_id,
            session_id,
            environment,
            app_name,
            remote_address,
            properties,
            jwt,
        };

        self.unleash_client.is_enabled(name, &context.into())
    }

    pub fn get_variant(
        &self,
        _ctx: &mut Ctx,
        name: &str,
        user_id: Option<&str>,
        session_id: Option<&str>,
        environment: Option<&str>,
        app_name: Option<&str>,
        remote_address: Option<&str>,
        properties: Option<&str>,
        jwt: Option<&str>,
    ) -> String {
        let context = Context {
            user_id,
            session_id,
            environment,
            app_name,
            remote_address,
            properties,
            jwt,
        };

        let variant = self.unleash_client.get_variant(name, &context.into());

        variant.name
    }
}

#[cfg(test)]
mod test {
    use super::{decode_jwt, try_props_from_str, Context};
    use test_case::test_case;
    use unleash_client::unleash_yggdrasil::Context as UnleashContext;

    #[test_case(".eyJzdWIiOiIxMjM0NSJ9.", Some("12345"); "should decode sub claim")]
    #[test_case(".e30.", None; "should return none when sub claim is missing")]
    #[test_case(".abcdefghijklmnopqrstu.", None; "should return none when payload is an invalid base64")]
    #[test_case("eyJzdWIiOiIxMjM0NSJ9", None; "should return none when jwt is missing header or signature")]
    pub fn test_decode_jwt(token: &str, expected: Option<&str>) {
        assert_eq!(
            decode_jwt(token).map(|claims| claims.sub),
            expected.map(String::from)
        );
    }

    #[test_case(Some("userId"), Some(".eyJzdWIiOiIxMjM0NSJ9."), Some("userId"); "should use user_id if present")]
    #[test_case(None, Some(".eyJzdWIiOiIxMjM0NSJ9."), Some("12345"); "should fallback to jwt")]
    pub fn test_context_user_id(user_id: Option<&str>, jwt: Option<&str>, expected: Option<&str>) {
        let context = Context {
            user_id,
            jwt,
            ..Default::default()
        };

        assert_eq!(
            UnleashContext::from(context.into()).user_id,
            expected.map(String::from)
        );
    }

    #[test]
    pub fn test_try_props_from_str() {
        let props = try_props_from_str("key1=value1,key2=value2,key3=value3").unwrap();

        assert_eq!(props.get("key1").unwrap(), &"value1".to_string());
        assert_eq!(props.get("key2").unwrap(), &"value2".to_string());
        assert_eq!(props.get("key3").unwrap(), &"value3".to_string());
    }
}
