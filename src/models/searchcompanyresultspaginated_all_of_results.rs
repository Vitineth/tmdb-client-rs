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
pub struct SearchcompanyresultspaginatedAllOfResults {
    #[serde(rename = "logo_path", skip_serializing_if = "Option::is_none")]
    pub logo_path: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl SearchcompanyresultspaginatedAllOfResults {
    pub fn new() -> SearchcompanyresultspaginatedAllOfResults {
        SearchcompanyresultspaginatedAllOfResults {
            logo_path: None,
            name: None,
            id: None,
        }
    }
}
