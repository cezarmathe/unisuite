//! Models.

use crate::types;

use std::ops::Deref;
use std::ops::DerefMut;

use chrono::DateTime;
use chrono::Utc;

use serde::Deserialize;
use serde::Serialize;

use serde_diff::SerdeDiff;

/// A topic from a Moodle course.
#[derive(Clone, Debug, Deserialize, SerdeDiff, Serialize)]
pub struct Topic {
    pub name: String,
    pub assignements: Vec<Assignement>,
    pub quizes: Vec<Quiz>,
    pub resources: Vec<Resource>,
    pub urls: Vec<Url>,
}

/// A name and url container.
#[derive(Clone, Debug, Deserialize, SerdeDiff, Serialize)]
pub struct NameUrlContainer {
    pub name: String,
    pub url: types::Url,
}

/// An assignement.
#[derive(Clone, Debug, Deserialize, SerdeDiff, Serialize)]
pub struct Assignement(pub NameUrlContainer);

impl Deref for Assignement {
    type Target = NameUrlContainer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Assignement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A quiz.
#[derive(Clone, Debug, Deserialize, SerdeDiff, Serialize)]
pub struct Quiz(pub NameUrlContainer);

impl Deref for Quiz {
    type Target = NameUrlContainer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Quiz {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// An url.
#[derive(Clone, Debug, Deserialize, SerdeDiff, Serialize)]
pub struct Url(pub NameUrlContainer);

impl Deref for Url {
    type Target = NameUrlContainer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A resource.
#[derive(Clone, Debug, Deserialize, SerdeDiff, Serialize)]
pub struct Resource {
    #[serde(flatten)]
    pub ident: NameUrlContainer,
    #[serde(rename = "contents")]
    pub content: Vec<ResourceContent>,
}

impl Deref for Resource {
    type Target = NameUrlContainer;
    fn deref(&self) -> &Self::Target {
        &self.ident
    }
}

impl DerefMut for Resource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ident
    }
}

#[derive(Clone, Debug, Deserialize, SerdeDiff, Serialize)]
pub struct ResourceContent {
    #[serde(rename = "type")]
    content_type: ContentType,
    name: String,
    url: types::Url,
    #[serde(with = "chrono::serde::ts_seconds")]
    #[serde_diff(opaque)]
    mtime: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, SerdeDiff, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    File,
    // fixme 29/01/2021: moodle has more resource types.
}
