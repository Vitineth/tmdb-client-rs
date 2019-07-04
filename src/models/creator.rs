/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Creator {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "profile_path", skip_serializing_if = "Option::is_none")]
    pub profile_path: Option<String>,
}

impl Creator {
    pub fn new() -> Creator {
        Creator {
            id: None,
            name: None,
            profile_path: None,
        }
    }
}
