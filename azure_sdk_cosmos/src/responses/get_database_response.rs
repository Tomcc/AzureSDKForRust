use crate::database::Database;
use crate::from_headers::*;
use crate::ResourceQuota;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{etag_from_headers, session_token_from_headers};
use chrono::{DateTime, Utc};
use hyper::header::HeaderMap;

#[derive(Debug, Clone)]
pub struct GetDatabaseResponse {
    pub database: Database,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub etag: String,
    pub last_change: DateTime<Utc>,
    pub resource_quota: ResourceQuota,
    pub resource_usage: ResourceQuota,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for GetDatabaseResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("get database response == {}", std::str::from_utf8(body)?);

        Ok(Self {
            database: serde_json::from_slice(body)?,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            last_change: last_state_change_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
        })
    }
}
