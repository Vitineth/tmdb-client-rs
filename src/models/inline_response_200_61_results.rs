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
pub struct InlineResponse20061Results {
    #[serde(rename = "site", skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "iso_3166_1", skip_serializing_if = "Option::is_none")]
    pub iso_3166_1: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "iso_639_1", skip_serializing_if = "Option::is_none")]
    pub iso_639_1: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl InlineResponse20061Results {
    pub fn new() -> InlineResponse20061Results {
        InlineResponse20061Results {
            site: None,
            size: None,
            iso_3166_1: None,
            name: None,
            id: None,
            _type: None,
            iso_639_1: None,
            key: None,
        }
    }
}

///
#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Trailer")]
    Trailer,
    #[serde(rename = "Teaser")]
    Teaser,
    #[serde(rename = "Clip")]
    Clip,
    #[serde(rename = "Featurette")]
    Featurette,
    #[serde(rename = "Opening Credits")]
    OpeningCredits,
    #[serde(rename = "Behind the Scenes")]
    BehindTheScenes,
    #[serde(rename = "Bloopers")]
    Bloopers,
    #[serde(rename = "Recap")]
    Recap,
}
