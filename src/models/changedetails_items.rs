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
pub struct ChangedetailsItems {
    #[serde(rename = "original_value", skip_serializing_if = "Option::is_none")]
    pub original_value: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "iso_639_1", skip_serializing_if = "Option::is_none")]
    pub iso_639_1: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ChangedetailsItems {
    pub fn new() -> ChangedetailsItems {
        ChangedetailsItems {
            original_value: None,
            action: None,
            id: None,
            time: None,
            iso_639_1: None,
            value: None,
        }
    }
}
