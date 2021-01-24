//! Common types.

use std::convert::TryFrom;
use std::convert::TryInto;
use std::net::SocketAddr;
use std::ops::Deref;

use serde::Deserialize;
use serde::Serialize;

/// Url type that can be serialized and deserialized as a string.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "&str", into = "String")]
pub struct Url(url::Url);

impl TryFrom<&str> for Url {
    type Error = anyhow::Error;
    fn try_from(src: &str) -> anyhow::Result<Url> {
        Ok(Url(url::Url::try_from(src)?))
    }
}

impl Into<String> for Url {
    fn into(self) -> String {
        self.as_str().to_string()
    }
}

impl Deref for Url {
    type Target = url::Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryInto<SocketAddr> for &Url {
    type Error = anyhow::Error;
    fn try_into(self) -> anyhow::Result<SocketAddr> {
        let addrs = self.socket_addrs(|| None)?;
        if let Some(value) = addrs.first() {
            Ok(value.clone())
        } else {
            anyhow::bail!("url: try into socket addr: failed")
        }
    }
}
