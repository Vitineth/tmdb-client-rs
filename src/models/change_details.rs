/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeDetails {
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<crate::models::ChangedetailsChanges>>,
}

impl ChangeDetails {
    pub fn new() -> ChangeDetails {
        ChangeDetails { changes: None }
    }
}
