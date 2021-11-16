mod auth_store;
mod error;

use error::{AuthResolverError, Result};
use rand::Rng;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    key: String,
    actions: Vec<Action>,
    indexes: Vec<String>,
    expires_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Key {
    pub fn create_from_value(value: Value) -> Result<Self> {
        let description = value
            .get("description")
            .map(|des| {
                from_value(des.clone())
                    .map_err(|_| AuthResolverError::InvalidApiKeyDescription(des.clone()))
            })
            .transpose()?;

        let key = generate_key();

        let actions = value
            .get("actions")
            .map(|act| {
                from_value(act.clone())
                    .map_err(|_| AuthResolverError::InvalidApiKeyAction(act.clone()))
            })
            .ok_or(AuthResolverError::MissingParameter("actions"))??;

        let indexes = value
            .get("indexes")
            .map(|ind| {
                from_value(ind.clone())
                    .map_err(|_| AuthResolverError::InvalidApiKeyIndexes(ind.clone()))
            })
            .ok_or(AuthResolverError::MissingParameter("indexes"))??;

        let expires_at = value
            .get("expiresAt")
            .map(|exp| {
                from_value(exp.clone())
                    .map_err(|_| AuthResolverError::InvalidApiKeyExpiresAt(exp.clone()))
            })
            .ok_or(AuthResolverError::MissingParameter("expiresAt"))??;

        let created_at = Utc::now();
        let updated_at = Utc::now();

        Ok(Self {
            description,
            key,
            actions,
            indexes,
            expires_at,
            created_at,
            updated_at,
        })
    }

    pub fn update_from_value(&mut self, value: Value) -> Result<()> {
        if let Some(des) = value.get("description") {
            let des = from_value(des.clone())
                .map_err(|_| AuthResolverError::InvalidApiKeyDescription(des.clone()));
            self.description = des?;
        }

        if let Some(act) = value.get("actions") {
            let act = from_value(act.clone())
                .map_err(|_| AuthResolverError::InvalidApiKeyAction(act.clone()));
            self.actions = act?;
        }

        if let Some(ind) = value.get("indexes") {
            let ind = from_value(ind.clone())
                .map_err(|_| AuthResolverError::InvalidApiKeyIndexes(ind.clone()));
            self.indexes = ind?;
        }

        if let Some(exp) = value.get("expiresAt") {
            let exp = from_value(exp.clone())
                .map_err(|_| AuthResolverError::InvalidApiKeyExpiresAt(exp.clone()));
            self.expires_at = exp?;
        }

        self.updated_at = Utc::now();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    #[serde(rename = "search")]
    Search,
    #[serde(rename = "documents.add")]
    DocumentsAdd,
    #[serde(rename = "documents.get")]
    DocumentsGet,
    #[serde(rename = "documents.delete")]
    DocumentsDelete,
    #[serde(rename = "indexes.add")]
    IndexesAdd,
    #[serde(rename = "indexes.get")]
    IndexesGet,
    #[serde(rename = "indexes.update")]
    IndexesUpdate,
    #[serde(rename = "indexes.delete")]
    IndexesDelete,
    #[serde(rename = "tasks.get")]
    TasksGet,
    #[serde(rename = "settings.get")]
    SettingsGet,
    #[serde(rename = "settings.update")]
    SettingsUpdate,
    #[serde(rename = "stats.get")]
    StatsGet,
    #[serde(rename = "dumps.create")]
    DumpsCreate,
    #[serde(rename = "dumps.get")]
    DumpsGet,
}

/// Generate a printable key of 64 characters using thread_rng.
fn generate_key() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut rng = rand::thread_rng();
    std::iter::repeat_with(|| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .take(64)
        .collect()
}