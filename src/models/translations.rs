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
pub struct Translations {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "translations", skip_serializing_if = "Option::is_none")]
    pub translations: Option<Vec<crate::models::TranslationsTranslations>>,
}

impl Translations {
    pub fn new() -> Translations {
        Translations {
            id: None,
            translations: None,
        }
    }
}
